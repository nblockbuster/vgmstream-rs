use rsmpeg::{avutil::AVMem, ffi::AVRational};

use crate::{
    streamfile::Streamfile,
    vgmstream::{VGMStream, STREAMFILE_DEFAULT_BUFFER_SIZE, VGMStreamCodecData},
};

use super::ffmpeg_opus::FFmpegCodecData;

pub fn init_ffmpeg_offset(sf: &mut Streamfile, start: u64, size: u64) -> Option<FFmpegCodecData> {
    return init_ffmpeg_header_offset(sf, &Vec::new(), start, size);
}

pub fn init_ffmpeg_header_offset(
    sf: &mut Streamfile,
    header: &Vec<u8>,
    start: u64,
    size: u64,
) -> Option<FFmpegCodecData> {
    return init_ffmpeg_header_offset_subsong(sf, header, start, size, 0);
}

use std::{
    borrow::BorrowMut,
    sync::atomic::{AtomicUsize, Ordering},
};

static G_FFMPEG_INITIALIZED: AtomicUsize = AtomicUsize::new(0);

/* Global FFmpeg init */
pub unsafe fn g_init_ffmpeg() {
    if G_FFMPEG_INITIALIZED.load(Ordering::SeqCst) == 1 {
        while G_FFMPEG_INITIALIZED.load(Ordering::SeqCst) < 2 {
            /* active wait for lack of a better way */
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    } else if G_FFMPEG_INITIALIZED.load(Ordering::SeqCst) == 0 {
        G_FFMPEG_INITIALIZED.fetch_add(1, Ordering::SeqCst);
        rsmpeg::ffi::av_log_set_flags(rsmpeg::ffi::AV_LOG_SKIP_REPEATED as i32);
        rsmpeg::ffi::av_log_set_level(rsmpeg::ffi::AV_LOG_ERROR as i32);
        G_FFMPEG_INITIALIZED.fetch_add(1, Ordering::SeqCst);
    }
}

/**
 * Manually init FFmpeg, from a fake header / offset.
 *
 * Takes a fake header, to trick FFmpeg into demuxing/decoding the stream.
 * This header will be seamlessly inserted before 'start' offset, and total filesize will be 'header_size' + 'size'.
 * The header buffer will be copied and memory-managed internally.
 * NULL header can used given if the stream has internal data recognized by FFmpeg at offset.
 * Stream index can be passed if the file has multiple audio streams that FFmpeg can demux (1=first).
 */
pub fn init_ffmpeg_header_offset_subsong(
    sf: &mut Streamfile,
    header: &Vec<u8>,
    start: u64,
    size: u64,
    target_subsong: i32,
) -> Option<FFmpegCodecData> {
    let mut data: FFmpegCodecData = Default::default();
    let mut size = size;
    // int errcode;

    /* check values */
    if !header.is_empty() {
        return None;
    }

    if size == 0 || start + size > sf.get_size(std::ptr::null_mut()) as u64 {
        assert_ne!(
            size,
            0,
            "FFMPEG: wrong start+size found: {} + {:x} > {:x}",
            start,
            size,
            sf.get_size(std::ptr::null_mut())
        );
        size = sf.get_size(std::ptr::null_mut()) as u64 - start;
    }

    /* initial FFmpeg setup */
    unsafe { g_init_ffmpeg() };

    /* basic setup */
    // data = calloc(1, sizeof(ffmpeg_codec_data));
    // if (!data) return NULL;

    // data.sf = reopen_streamfile(sf, 0);
    data.sf = Some(sf.clone());
    // if data.sf.is_none() {
    //     return None;
    // }
    /* fake header to trick FFmpeg into demuxing/decoding the stream */
    if header.len() > 0 {
        data.header_size = header.len() as u64;
        // data.header_block = av_memdup(header, header_size);
        data.header_block = header.clone();
        if data.header_block.is_empty() {
            return None;
        }
    }

    data.start = start;
    data.offset = data.start;
    data.size = size;
    data.logical_offset = 0;
    data.logical_size = data.header_size + data.size;

    /* setup FFmpeg's internals, attempt to autodetect format and gather some info */
    unsafe {
        let mut errcode = init_ffmpeg_config(&mut data, target_subsong, false);
        if errcode < 0 {
            return None;
        }
    }
    /* reset non-zero values */
    data.read_packet = true;

    /* setup other values */
    unsafe {
        let streams: Vec<rsmpeg::ffi::AVStream> = Vec::from_raw_parts(*(*data.formatCtx).streams, (*data.formatCtx).nb_streams as usize, (*data.formatCtx).nb_streams as usize);
        let stream = streams[data.stream_index as usize];
        let mut tb: AVRational = std::mem::zeroed();
        tb.num = 1;
        tb.den = (*data.codecCtx).sample_rate;
        // AVStream* stream = data.formatCtx->streams[data.stream_index];
        // AVRational tb = {0};

        // tb.num = 1; tb.den = data.codecCtx->sample_rate;

        /* try to guess frames/samples (duration isn't always set) */
        data.total_samples = rsmpeg::ffi::av_rescale_q(stream.duration, stream.time_base, tb);
        if data.total_samples < 0 {
            data.total_samples = 0;
        }

        /* read start samples to be skipped (encoder delay), info only.
         * Not too reliable though, see ffmpeg_set_skip_samples */
        if stream.start_time != 0 && stream.start_time != rsmpeg::ffi::AV_NOPTS_VALUE {
            data.skip_samples =
                rsmpeg::ffi::av_rescale_q(stream.start_time, stream.time_base, tb);
        }
        if data.skip_samples < 0 {
            data.skip_samples = 0;
        }

        /* check ways to skip encoder delay/padding, for debugging purposes (some may be old/unused/encoder only/etc) */
        //VGM_ASSERT(data.codecCtx->internal->skip_samples > 0, ...); /* for codec use, not accessible */

        // VGM_ASSERT(data.codecCtx.delay > 0, "FFMPEG: delay %i\n", data.codecCtx.delay);//delay: OPUS
        // VGM_ASSERT(stream.codecpar.initial_padding > 0, "FFMPEG: initial_padding %i\n", stream.codecpar.initial_padding);//delay: OPUS
        // VGM_ASSERT(stream.codecpar.trailing_padding > 0, "FFMPEG: trailing_padding %i\n", stream.codecpar.trailing_padding);
        // VGM_ASSERT(stream.codecpar.seek_preroll > 0, "FFMPEG: seek_preroll %i\n", stream.codecpar.seek_preroll);//seek delay: OPUS
        // VGM_ASSERT(stream.start_time > 0, "FFMPEG: start_time %i\n", stream.start_time); //delay

        /* also negative timestamp for formats like OGG/OPUS */
        /* not using it: BINK, FLAC, ATRAC3, XMA, MPC, WMA (may use internal skip samples) */
    }

    /* setup decent seeking for faulty formats */
    // errcode = init_seek(data);
    // if (errcode < 0) {
    //     println!("FFMPEG: can't init_seek, error={} (using force_seek)", errcode);
    //     ffmpeg_set_force_seek(data);
    // }

    return Some(data);
}

const FFMPEG_DEFAULT_IO_BUFFER_SIZE: usize = STREAMFILE_DEFAULT_BUFFER_SIZE;

pub unsafe fn init_ffmpeg_config(
    data: &mut FFmpegCodecData,
    target_subsong: i32,
    reset: bool,
) -> i32 {
    let mut errcode = 0;

    /* basic IO/format setup */
    data.buffer = rsmpeg::ffi::av_malloc(FFMPEG_DEFAULT_IO_BUFFER_SIZE) as *mut u8;
    if data.buffer.is_null() {
        if errcode < 0 {
            return errcode;
        }
        return -1;
    }

    // let mut buffer = rsmpeg::avutil::AVMem::new(FFMPEG_DEFAULT_IO_BUFFER_SIZE);

    data.ioCtx = rsmpeg::ffi::avio_alloc_context(
        data.buffer,
        FFMPEG_DEFAULT_IO_BUFFER_SIZE as i32,
        0,
        data as *mut FFmpegCodecData as *mut _,
        Some(ffmpeg_read),
        None,
        Some(ffmpeg_seek),
    );

    if data.ioCtx.is_null() {
        if errcode < 0 {
            return errcode;
        }
        return -1;
    }

    data.formatCtx = rsmpeg::ffi::avformat_alloc_context();
    if data.formatCtx.is_null() {
        if errcode < 0 {
            return errcode;
        }
        return -1;
    }

    (*data.formatCtx).pb = data.ioCtx;

    //data.inputFormatCtx = av_find_input_format("h264"); /* set directly? */
    /* on reset could use AVFormatContext.iformat to reload old format too */

    // let mut format_ctx_mutmut: *mut *mut rsmpeg::ffi::AVFormatContext = &mut data.formatCtx;
    // let u8_vec = Vec::from_raw_parts(
    //     data.buffer,
    //     FFMPEG_DEFAULT_IO_BUFFER_SIZE,
    //     FFMPEG_DEFAULT_IO_BUFFER_SIZE,
    // );

    errcode = rsmpeg::ffi::avformat_open_input(
        &mut data.formatCtx,
        std::ptr::null_mut() as *mut std::os::raw::c_char,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    if errcode < 0 {
        return errcode;
    }

    errcode = rsmpeg::ffi::avformat_find_stream_info(data.formatCtx, std::ptr::null_mut());
    if errcode < 0 {
        return errcode;
    }

    /* find valid audio stream and set other streams to discard */
    {
        // int i, stream_index, stream_count;

        let mut stream_index = -1;
        let mut stream_count = 0;
        if reset {
            stream_index = data.stream_index;
        }

        for i in 0..(*data.formatCtx).nb_streams {
            let mut streams: Vec<rsmpeg::ffi::AVStream> = Vec::from_raw_parts(*(*data.formatCtx).streams, (*data.formatCtx).nb_streams as usize, (*data.formatCtx).nb_streams as usize);
            let mut stream = streams[i as usize];

            if stream.codecpar != std::ptr::null_mut()
                && (*stream.codecpar).codec_type == rsmpeg::ffi::AVMediaType_AVMEDIA_TYPE_AUDIO
            {
                stream_count += 1;

                /* select Nth audio stream if specified, or first one */
                if stream_index < 0 || (target_subsong > 0 && stream_count == target_subsong) {
                    stream_index = i as i32;
                }
            }

            if i as i32 != stream_index {
                stream.discard = rsmpeg::ffi::AVDiscard_AVDISCARD_ALL; /* disable demuxing for other streams */
            }
            streams[i as usize] = stream;

            (*data.formatCtx).streams = &mut streams.as_mut_ptr();
        }
        if stream_count < target_subsong {
            if errcode < 0 {
                return errcode;
            }
            return -1;
        }
        if stream_index < 0 {
            if errcode < 0 {
                return errcode;
            }
            return -1;
        }

        data.stream_index = stream_index;
        data.stream_count = stream_count;
    }

    /* setup codec with stream info */
    data.codecCtx = rsmpeg::ffi::avcodec_alloc_context3(std::ptr::null_mut());
    if data.codecCtx.is_null() {
        if errcode < 0 {
            return errcode;
        }
        return -1;
    }

    let streams: Vec<rsmpeg::ffi::AVStream> = Vec::from_raw_parts(*(*data.formatCtx).streams, (*data.formatCtx).nb_streams as usize, (*data.formatCtx).nb_streams as usize);
    let stream = streams[data.stream_index as usize];
    errcode = rsmpeg::ffi::avcodec_parameters_to_context(data.codecCtx, stream.codecpar.cast_const());
    if errcode < 0 {
        return errcode;
    }

    /* deprecated and seemingly not needed */
    //av_codec_set_pkt_timebase(data.codecCtx, stream->time_base);

    /* not useddeprecated and seemingly not needed */
    data.codec = rsmpeg::ffi::avcodec_find_decoder((*data.codecCtx).codec_id) as *mut rsmpeg::ffi::AVCodec;
    if data.codec.is_null() {
        if errcode < 0 {
            return errcode;
        }
        return -1;
    }

    errcode = rsmpeg::ffi::avcodec_open2(data.codecCtx, data.codec, std::ptr::null_mut());
    if errcode < 0 {
        return errcode;
    }

    /* prepare codec and frame/packet buffers */
    data.packet = std::mem::transmute_copy(&rsmpeg::ffi::av_malloc(std::mem::size_of::<
        rsmpeg::ffi::AVPacket,
    >())); /* av_packet_alloc? */
    if data.packet.is_null() {
        if errcode < 0 {
            return errcode;
        }
        return -1;
    }
    rsmpeg::ffi::av_new_packet(data.packet, 0);
    //av_packet_unref?

    data.frame = rsmpeg::ffi::av_frame_alloc();
    if data.frame.is_null() {
        if errcode < 0 {
            return errcode;
        }
        return -1;
    }
    rsmpeg::ffi::av_frame_unref(data.frame);

    // copy buffer to data.buffer, without making it panic on drop
    // data.buffer = buffer;
    // data.buffer.copy_from(buffer, FFMPEG_DEFAULT_IO_BUFFER_SIZE);

    return 0;
}

/* ******************************************** */
/* AVIO CALLBACKS                               */
/* ******************************************** */

/* AVIO callback: read stream, handling custom data */
pub unsafe extern "C" fn ffmpeg_read(
    mut opaque: *mut std::ffi::c_void,
    mut buf: *mut u8,
    read_size: i32,
) -> i32 {
    let mut data: &mut FFmpegCodecData = std::mem::transmute(opaque);
    // let bytes = 0;
    let mut max_to_copy = 0;
    let mut read_size = read_size;
    /* clamp reads */
    if data.logical_offset + read_size as u64 > data.logical_size {
        read_size = (data.logical_size - data.logical_offset) as i32;
    }
    if read_size == 0 {
        return rsmpeg::ffi::AVERROR_EOF;
    }

    /* handle reads on inserted header */
    if data.header_size != 0 && data.logical_offset < data.header_size {
        max_to_copy = data.header_size - data.logical_offset;
        if max_to_copy > read_size as u64 {
            max_to_copy = read_size as u64;
        }

        buf.copy_from(
            data.header_block.as_ptr().add(data.logical_offset as usize),
            max_to_copy as usize,
        );
        read_size -= max_to_copy as i32;
        data.logical_offset += max_to_copy;

        if read_size == 0 {
            /* offset still in header */
            return if max_to_copy != 0 {
                max_to_copy as i32
            } else {
                rsmpeg::ffi::AVERROR_EOF
            };
        }
    }

    /* main read */

    let mut sf2 = data.sf.clone();
    let mut opus_data = sf2.as_mut().unwrap().data;
    let read_func = data.sf.as_ref().unwrap().read.unwrap();
    let data_read = read_func(&mut data.sf.as_mut().unwrap(), data.offset as usize, read_size as usize, opus_data);

    sf2.as_mut().unwrap().data = opus_data;
    data.sf = sf2;

    buf.copy_from(
        data_read.as_ptr(),
        read_size as usize,
    );
    data.offset += read_size as u64;
    data.logical_offset += read_size as u64;

    return read_size;
}

/* AVIO callback: seek stream, handling custom data */
pub unsafe extern "C" fn ffmpeg_seek(
    opaque: *mut std::ffi::c_void,
    offset: i64,
    whence: i32,
) -> i64 {
    let mut data: &mut FFmpegCodecData = std::mem::transmute(opaque);
    let mut offset = offset;
    let mut whence = whence;
    /* get cache'd size */
    if whence as u32 & rsmpeg::ffi::AVSEEK_SIZE != 0 {
        return data.logical_size as i64;
    }

    whence &= !(rsmpeg::ffi::AVSEEK_SIZE as i32 | rsmpeg::ffi::AVSEEK_FORCE as i32);
    /* find the final offset FFmpeg sees (within fake header + virtual size) */
    match whence as u32 {
        rsmpeg::ffi::SEEK_SET => {} /* absolute */
        rsmpeg::ffi::SEEK_CUR => {
            /* relative to current */
            offset += data.logical_offset as i64;
        }
        rsmpeg::ffi::SEEK_END => {
            /* relative to file end (should be negative) */
            offset += data.logical_size as i64;
        }
        _ => {}
    }

    /* clamp offset; fseek does this too */
    if offset > data.logical_size as i64 {
        offset = data.logical_size as i64;
    } else if offset < 0 {
        offset = 0;
    }

    /* seeks inside fake header */
    if offset < data.header_size as i64 {
        data.logical_offset = offset as u64;
        data.offset = data.start;
        return 0;
    }

    /* main seek */
    data.logical_offset = offset as u64;
    data.offset = data.start + (offset as u64 - data.header_size);
    return 0;
}

pub unsafe fn ffmpeg_get_codec_name(data: &FFmpegCodecData) -> &str {
    if data.codec.is_null() {
        return "";
    }
    if !(*data.codec).long_name.is_null() {
        return std::str::from_utf8_unchecked(
            std::ffi::CStr::from_ptr((*data.codec).long_name).to_bytes(),
        );
    }
    if !(*data.codec).name.is_null() {
        return std::str::from_utf8_unchecked(
            std::ffi::CStr::from_ptr((*data.codec).name).to_bytes(),
        );
    }
    return "";
}

pub fn ffmpeg_set_force_seek(data: &mut FFmpegCodecData) {
    /* some formats like Smacker are so buggy that any seeking is impossible (even on video players),
     * or MPC with an incorrectly parsed seek table (using as 0 some non-0 seek offset).
     * whatever, we'll just kill and reconstruct FFmpeg's config every time */
    data.force_seek = true;
    reset_ffmpeg(data); /* reset state from trying to seek */
    //stream = data.formatCtx->streams[data.stream_index];
}

pub fn reset_ffmpeg(data: &mut FFmpegCodecData) {
    seek_ffmpeg(data, 0);
}

pub fn seek_ffmpeg(data: &mut FFmpegCodecData, num_sample: i32) {
    /* Start from 0 and discard samples until sample (slower but not too noticeable).
     * Due to many FFmpeg quirks seeking to a sample is erratic at best in most formats. */

    if data.force_seek {
        let mut errcode = 0;

        /* kill+redo everything to allow seeking for extra-buggy formats,
         * kinda horrid but seems fast enough and very few formats need this */
        unsafe {
            free_ffmpeg_config(data);

            data.offset = data.start;
            data.logical_offset = 0;

            errcode = init_ffmpeg_config(data, 0, true);
            if errcode < 0 {
                println!("FFMPEG: error during force_seek");
                data.bad_init = true; /* internals were probably free'd */
                return;
            }
        }
    } else {
        unsafe {
            rsmpeg::ffi::avformat_seek_file(
                data.formatCtx,
                data.stream_index,
                0,
                0,
                0,
                rsmpeg::ffi::AVSEEK_FLAG_ANY as i32,
            );
            rsmpeg::ffi::avcodec_flush_buffers(data.codecCtx);
        }
    }

    data.samples_consumed = 0;
    data.samples_filled = 0;
    data.samples_discard = num_sample;

    data.read_packet = true;
    data.end_of_stream = false;
    data.end_of_audio = false;

    /* consider skip samples (encoder delay), if manually set */
    if data.skip_samples_set {
        data.samples_discard += data.skip_samples as i32;
        /* internally FFmpeg may skip (skip_samples/start_skip_samples) too */
    }

    return;
}

pub unsafe fn free_ffmpeg_config(data: &mut FFmpegCodecData) {
    if !data.packet.is_null() {
        rsmpeg::ffi::av_packet_unref(data.packet);
        rsmpeg::ffi::av_free(data.packet as *mut _);
        data.packet = std::ptr::null_mut();
    }
    if !data.frame.is_null() {
        rsmpeg::ffi::av_frame_unref(data.frame);
        rsmpeg::ffi::av_free(data.frame as *mut _);
        data.frame = std::ptr::null_mut();
    }
    if !data.codecCtx.is_null() {
        rsmpeg::ffi::avcodec_close(data.codecCtx);
        rsmpeg::ffi::avcodec_free_context(data.codecCtx as *mut *mut rsmpeg::ffi::AVCodecContext);
        data.codecCtx = std::ptr::null_mut();
    }
    if !data.formatCtx.is_null() {
        rsmpeg::ffi::avformat_close_input(data.formatCtx as *mut *mut rsmpeg::ffi::AVFormatContext);
        //avformat_free_context(data.formatCtx); /* done in close_input */
        data.formatCtx = std::ptr::null_mut();
    }
    if !data.ioCtx.is_null() {
        /* buffer passed in is occasionally freed and replaced.
         * the replacement must be free'd as well (below) */
        data.buffer = (*data.ioCtx).buffer;
        // data.buffer = AVMem::new((*data.ioCtx).buffer_size as usize);
        // data.buffer.copy_from((*data.ioCtx).buffer, (*data.ioCtx).buffer_size as usize);
        rsmpeg::ffi::avio_context_free(data.ioCtx as *mut *mut rsmpeg::ffi::AVIOContext);
        //av_free(data.ioCtx); /* done in context_free (same thing) */
        data.ioCtx = std::ptr::null_mut();
    }
    if data.buffer.is_null() {
        // rsmpeg::ffi::av_free(data.buffer.as_mut_ptr() as *mut _);
        // data.buffer = AVMem::new(0);
        data.buffer = std::ptr::null_mut();
    }

    //todo avformat_find_stream_info may cause some Win Handle leaks? related to certain option
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AVSampleFormat {
    AV_SAMPLE_FMT_NONE = -1,
    /// unsigned 8 bits
    AV_SAMPLE_FMT_U8,
    /// signed 16 bits
    AV_SAMPLE_FMT_S16,
    /// signed 32 bits
    AV_SAMPLE_FMT_S32,
    /// float
    AV_SAMPLE_FMT_FLT,
    /// double
    AV_SAMPLE_FMT_DBL,

    /// unsigned 8 bits, planar
    AV_SAMPLE_FMT_U8P,
    /// signed 16 bits, planar
    AV_SAMPLE_FMT_S16P,
    /// signed 32 bits, planar
    AV_SAMPLE_FMT_S32P,
    /// float, planar
    AV_SAMPLE_FMT_FLTP,
    /// double, planar
    AV_SAMPLE_FMT_DBLP,
    /// signed 64 bits
    AV_SAMPLE_FMT_S64,
    /// signed 64 bits, planar
    AV_SAMPLE_FMT_S64P,

    /// Number of sample formats. DO NOT USE if linking dynamically
    AV_SAMPLE_FMT_NB,
}

impl Into<i32> for AVSampleFormat {
    fn into(self) -> i32 {
        self as i32
    }
}

impl From<i32> for AVSampleFormat {
    fn from(value: i32) -> Self {
        match value {
            -1 => AVSampleFormat::AV_SAMPLE_FMT_NONE,
            0 => AVSampleFormat::AV_SAMPLE_FMT_U8,
            1 => AVSampleFormat::AV_SAMPLE_FMT_S16,
            2 => AVSampleFormat::AV_SAMPLE_FMT_S32,
            3 => AVSampleFormat::AV_SAMPLE_FMT_FLT,
            4 => AVSampleFormat::AV_SAMPLE_FMT_DBL,
            5 => AVSampleFormat::AV_SAMPLE_FMT_U8P,
            6 => AVSampleFormat::AV_SAMPLE_FMT_S16P,
            7 => AVSampleFormat::AV_SAMPLE_FMT_S32P,
            8 => AVSampleFormat::AV_SAMPLE_FMT_FLTP,
            9 => AVSampleFormat::AV_SAMPLE_FMT_DBLP,
            10 => AVSampleFormat::AV_SAMPLE_FMT_S64,
            11 => AVSampleFormat::AV_SAMPLE_FMT_S64P,
            _ => AVSampleFormat::AV_SAMPLE_FMT_NONE,
        }
    }
}

pub fn copy_samples(data: &mut FFmpegCodecData, outbuf: &mut [i16], samples_to_do: i32) {
    let channels = unsafe { (*data.codecCtx).ch_layout.nb_channels };
    let is_planar = unsafe {
        rsmpeg::ffi::av_sample_fmt_is_planar((*data.codecCtx).sample_fmt) == 1 && (channels > 1)
    };
    let mut ibuf;

    if is_planar {
        ibuf = unsafe { *(*data.frame).extended_data };
    } else {
        ibuf = unsafe { (*data.frame).data[0] };
    }

    match unsafe { (*data.codecCtx).sample_fmt.into() } {
        /* unused? */
        AVSampleFormat::AV_SAMPLE_FMT_U8P => {
            if is_planar {
                let mut ibuf: &[&[u8]] = unsafe {
                    std::slice::from_raw_parts(
                        (*data.frame).extended_data as *const &[u8],
                        channels as usize,
                    )
                }; 
                samples_u8p_to_s16(outbuf, ibuf, channels, samples_to_do, data.samples_consumed);
            }
        }
        AVSampleFormat::AV_SAMPLE_FMT_U8 => {
            let mut ibuf: &[u8] = unsafe {
                std::slice::from_raw_parts((*data.frame).data[0], channels as usize)
            };
            samples_u8_to_s16(outbuf, ibuf, channels, samples_to_do, data.samples_consumed);
        }
        /* common */
        AVSampleFormat::AV_SAMPLE_FMT_S16P => {
            if is_planar {
                let mut ibuf: &[&[i16]] = unsafe {
                    std::slice::from_raw_parts(
                        (*data.frame).extended_data as *const &[i16],
                        channels as usize,
                    )
                };
                samples_s16p_to_s16(outbuf, ibuf, channels, samples_to_do, data.samples_consumed);
            }
        }
        AVSampleFormat::AV_SAMPLE_FMT_S16 => {
            let mut ibuf: &[i16] = unsafe {
                std::slice::from_raw_parts((*data.frame).data[0] as *const i16, channels as usize)
            };
            samples_s16_to_s16(outbuf, ibuf, channels, samples_to_do, data.samples_consumed);
        }
        /* possibly FLAC and other lossless codecs */
        AVSampleFormat::AV_SAMPLE_FMT_S32P => {
            if is_planar {
                let mut ibuf: &[&[i32]] = unsafe {
                    std::slice::from_raw_parts(
                        (*data.frame).extended_data as *const &[i32],
                        channels as usize,
                    )
                };
                samples_s32p_to_s16(outbuf, ibuf, channels, samples_to_do, data.samples_consumed);
            }
        }
        AVSampleFormat::AV_SAMPLE_FMT_S32 => {
            let mut ibuf: &[i32] = unsafe {
                std::slice::from_raw_parts((*data.frame).data[0] as *const i32, channels as usize)
            };
            samples_s32_to_s16(outbuf, ibuf, channels, samples_to_do, data.samples_consumed);
        }
        /* mainly MDCT-like codecs (Ogg, AAC, etc) */
        AVSampleFormat::AV_SAMPLE_FMT_FLTP => {
            if is_planar {
                let mut ibuf: &[&[f32]] = unsafe {
                    std::slice::from_raw_parts(
                        (*data.frame).extended_data as *const &[f32],
                        channels as usize,
                    )
                };
                samples_fltp_to_s16(
                    outbuf,
                    ibuf,
                    channels,
                    samples_to_do,
                    data.samples_consumed,
                    data.invert_floats_set,
                );
            }
        }
        AVSampleFormat::AV_SAMPLE_FMT_FLT => {
            let mut ibuf: &[f32] = unsafe {
                std::slice::from_raw_parts((*data.frame).data[0] as *const f32, channels as usize)
            };
            samples_flt_to_s16(
                outbuf,
                ibuf,
                channels,
                samples_to_do,
                data.samples_consumed,
                data.invert_floats_set,
            );
        }
        /* possibly PCM64 only (not enabled) */
        AVSampleFormat::AV_SAMPLE_FMT_DBLP => {
            if is_planar {
                let mut ibuf: &[&[f64]] = unsafe {
                    std::slice::from_raw_parts(
                        (*data.frame).extended_data as *const &[f64],
                        channels as usize,
                    )
                };
                samples_dblp_to_s16(outbuf, ibuf, channels, samples_to_do, data.samples_consumed);
            }
        }
        AVSampleFormat::AV_SAMPLE_FMT_DBL => {
            let mut ibuf: &[f64] = unsafe {
                std::slice::from_raw_parts((*data.frame).data[0] as *const f64, channels as usize)
            };
            samples_dbl_to_s16(outbuf, ibuf, channels, samples_to_do, data.samples_consumed);
        }
        _ => {}
    }

    if data.channel_remap_set {
        remap_audio(outbuf, samples_to_do, channels, data.channel_remap);
    }
}

// static void remap_audio(sample_t* outbuf, int sample_count, int channels, int* channel_mappings) {
//     int ch_from,ch_to,s;
//     sample_t temp;
//     for (s = 0; s < sample_count; s++) {
//         for (ch_from = 0; ch_from < channels; ch_from++) {
//             if (ch_from > 32)
//                 continue;

//             ch_to = channel_mappings[ch_from];
//             if (ch_to < 1 || ch_to > 32 || ch_to > channels-1 || ch_from == ch_to)
//                 continue;

//             temp = outbuf[s*channels + ch_from];
//             outbuf[s*channels + ch_from] = outbuf[s*channels + ch_to];
//             outbuf[s*channels + ch_to] = temp;
//         }
//     }
// }
pub fn remap_audio(outbuf: &mut [i16], sample_count: i32, channels: i32, channel_mappings: [i32;32]) {
    for s in 0..sample_count {
        for ch_from in 0..channels {
            if ch_from > 32 {
                continue;
            }

            let ch_to = channel_mappings[ch_from as usize];
            if ch_to < 1
                || ch_to > 32
                || ch_to > channels - 1
                || ch_from as i32 == ch_to as i32
            {
                continue;
            }

            let temp = outbuf[(s * channels + ch_from) as usize];
            outbuf[(s * channels + ch_from) as usize] = outbuf[(s * channels + ch_to) as usize];
            outbuf[(s * channels + ch_to) as usize] = temp;
        }
    }
}


/* decode samples of any kind of FFmpeg format */
pub fn decode_ffmpeg(
    vgmstream: &mut VGMStream,
    outbuf: &mut [i16],
    mut samples_to_do: i32,
    channels: i32,
) {
    let mut data = vgmstream.codec_data.as_mut().unwrap();
    
    match data {
        VGMStreamCodecData::CustomFFmpeg(data) => {
            while samples_to_do > 0 {
                if data.samples_consumed < data.samples_filled {
                    /* consume samples */
                    let mut samples_to_get = data.samples_filled - data.samples_consumed;
                    if data.samples_discard != 0 {
                        /* discard samples for looping */
                        if samples_to_get > data.samples_discard {
                            samples_to_get = data.samples_discard;
                        }
                        data.samples_discard -= samples_to_get;
                    } else {
                        /* get max samples and copy */
                        if samples_to_get > samples_to_do {
                            samples_to_get = samples_to_do;
                        }
        
                        copy_samples(data, outbuf, samples_to_get);
        
                        samples_to_do -= samples_to_get;
                        // outbuf += samples_to_get * channels;
                    }
        
                    /* mark consumed samples */
                    data.samples_consumed += samples_to_get;
                } else {
                    let ok = unsafe { decode_ffmpeg_frame(data) };
                    if !ok {
                        println!("FFMPEG: decode fail, missing {} samples", samples_to_do);
                        samples_silence_s16(outbuf, channels, samples_to_do);
                        return;
                    }
                }
            }
        }
        _ => {}
    }
}

/* decodes a new frame to internal data */
pub unsafe fn decode_ffmpeg_frame(data: &mut FFmpegCodecData) -> bool {
    let mut errcode = 0;
    let mut frame_error = false;
    if data.bad_init {
        return false;
    }

    /* ignore once file is done (but not on EOF as FFmpeg can output samples until end_of_audio) */
    if /*data.end_of_stream ||*/ data.end_of_audio {
        println!("FFMPEG: decode after end of audio");
        return false;
    }


    /* read data packets until valid is found */
    while data.read_packet && !data.end_of_audio {
        if !data.end_of_stream {
            /* reset old packet */
            rsmpeg::ffi::av_packet_unref(data.packet);

            /* read encoded data from demuxer into packet */
            errcode = rsmpeg::ffi::av_read_frame(data.formatCtx, data.packet);
            if errcode < 0 {
                if errcode == rsmpeg::ffi::AVERROR_EOF {
                    data.end_of_stream = true; /* no more data to read (but may "drain" samples) */
                }
                else {
                    println!("FFMPEG: av_read_frame errcode={}", errcode);
                    frame_error = true; //goto fail;
                }

                if !(*data.formatCtx).pb.is_null() && (*(*data.formatCtx).pb).error != 0{
                    println!("FFMPEG: pb error={}", (*(*data.formatCtx).pb).error);
                    frame_error = true; //goto fail;
                }
            }

            /* ignore non-selected streams */
            if (*data.packet).stream_index != data.stream_index {
                continue;
            }
        }

        /* send encoded data to frame decoder (NULL at EOF to "drain" samples below) */
        errcode = rsmpeg::ffi::avcodec_send_packet(data.codecCtx, if data.end_of_stream {std::ptr::null_mut()} else { data.packet });
        if errcode < 0 {
            if errcode != rsmpeg::ffi::AVERROR(rsmpeg::ffi::EAGAIN) {
                println!("FFMPEG: avcodec_send_packet errcode={}", errcode);
                frame_error = true; //goto fail;
            }
        }

        data.read_packet = false; /* got data */
    }

    /* decode frame samples from sent packet or "drain" samples*/
    if !frame_error {
        /* receive uncompressed sample data from decoded frame */
        errcode = rsmpeg::ffi::avcodec_receive_frame(data.codecCtx, data.frame);
        if errcode < 0 {
            if errcode == rsmpeg::ffi::AVERROR_EOF {
                data.end_of_audio = true; /* no more audio, file is fully decoded */
            }
            else if errcode == rsmpeg::ffi::AVERROR(rsmpeg::ffi::EAGAIN) {
                data.read_packet = true; /* 0 samples, request more encoded data */
            }
            else {
                println!("FFMPEG: avcodec_receive_frame errcode={}", errcode);
                frame_error = true;//goto fail;
            }
        }
    }

    /* on frame_error simply uses current frame (possibly with nb_samples=0), which mirrors ffmpeg's output
     * (ex. BlazBlue X360 022_btl_az.xwb) */


    data.samples_consumed = 0;
    data.samples_filled = (*data.frame).nb_samples;
    return true;
}


pub fn samples_silence_s16(obuf: &mut [i16], ochs: i32, samples: i32) {
    let total_samples = (samples * ochs) as usize;
    for s in 0..total_samples {
        obuf[s] = 0;
    }
}

pub fn samples_u8_to_s16(obuf: &mut [i16], ibuf: &[u8], ichs: i32, samples: i32, skip: i32) {
    let total_samples = (samples * ichs) as usize;
    for s in 0..total_samples {
        obuf[s] = ((ibuf[skip as usize * ichs as usize + s] as i32 - 0x80) << 8) as i16;
    }
}

pub fn samples_u8p_to_s16(obuf: &mut [i16], ibuf: &[&[u8]], ichs: i32, samples: i32, skip: i32) {
    for ch in 0..ichs {
        for s in 0..samples {
            obuf[(s * ichs + ch) as usize] =
                ((ibuf[ch as usize][skip as usize + s as usize] as i32 - 0x80) << 8) as i16;
        }
    }
}

pub fn samples_s16_to_s16(obuf: &mut [i16], ibuf: &[i16], ichs: i32, samples: i32, skip: i32) {
    let total_samples = (samples * ichs) as usize;
    for s in 0..total_samples {
        obuf[s] = ibuf[skip as usize * ichs as usize + s];
    }
}

pub fn samples_s16p_to_s16(obuf: &mut [i16], ibuf: &[&[i16]], ichs: i32, samples: i32, skip: i32) {
    for ch in 0..ichs {
        for s in 0..samples {
            obuf[(s * ichs + ch) as usize] = ibuf[ch as usize][skip as usize + s as usize];
        }
    }
}

pub fn samples_s32_to_s16(obuf: &mut [i16], ibuf: &[i32], ichs: i32, samples: i32, skip: i32) {
    let total_samples = (samples * ichs) as usize;
    for s in 0..total_samples {
        obuf[s] = (ibuf[skip as usize * ichs as usize + s] >> 16) as i16;
    }
}

pub fn samples_s32p_to_s16(obuf: &mut [i16], ibuf: &[&[i32]], ichs: i32, samples: i32, skip: i32) {
    for ch in 0..ichs {
        for s in 0..samples {
            obuf[(s * ichs + ch) as usize] =
                (ibuf[ch as usize][skip as usize + s as usize] >> 16) as i16;
        }
    }
}

use crate::util::util::clamp16;

pub fn samples_flt_to_s16(
    obuf: &mut [i16],
    ibuf: &[f32],
    ichs: i32,
    samples: i32,
    skip: i32,
    invert: bool,
) {
    let total_samples = (samples * ichs) as usize;
    let scale = if invert { -32768.0 } else { 32768.0 };
    for s in 0..total_samples {
        obuf[s] = clamp16((ibuf[skip as usize * ichs as usize + s] * scale) as i32) as i16;
    }
}

pub fn samples_fltp_to_s16(
    obuf: &mut [i16],
    ibuf: &[&[f32]],
    ichs: i32,
    samples: i32,
    skip: i32,
    invert: bool,
) {
    let scale = if invert { -32768.0 } else { 32768.0 };
    for ch in 0..ichs {
        for s in 0..samples {
            obuf[(s * ichs + ch) as usize] =
                clamp16((ibuf[ch as usize][skip as usize + s as usize] * scale) as i32) as i16;
        }
    }
}


pub fn samples_dbl_to_s16(obuf: &mut [i16], ibuf: &[f64], ichs: i32, samples: i32, skip: i32) {
    let total_samples = (samples * ichs) as usize;
    for s in 0..total_samples {
        obuf[s] = clamp16((ibuf[skip as usize * ichs as usize + s] * 32768.0) as i32) as i16;
    }
}

pub fn samples_dblp_to_s16(obuf: &mut [i16], ibuf: &[&[f64]], ichs: i32, samples: i32, skip: i32) {
    for ch in 0..ichs {
        for s in 0..samples {
            obuf[(s * ichs + ch) as usize] =
                clamp16((ibuf[ch as usize][skip as usize + s as usize] * 32768.0) as i32) as i16;
        }
    }
}

