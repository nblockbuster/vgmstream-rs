use crate::{streamfile::Streamfile, vgmstream::VGMStreamCodecData};
use rsmpeg::{
    avutil::AVMem,
    ffi::{AVCodec, AVCodecContext, AVFormatContext, AVFrame, AVIOContext, AVPacket},
};

use super::coding::OpusConfig;

#[derive(Debug, Clone)]
pub struct FFmpegCodecData {
    /*** IO internals ***/
    pub sf: Option<Streamfile>,

    pub start: u64,          // absolute start within the streamfile
    pub offset: u64,         // absolute offset within the streamfile
    pub size: u64,           // max size within the streamfile
    pub logical_offset: u64, // computed offset FFmpeg sees (including fake header)
    pub logical_size: u64,   // computed size FFmpeg sees (including fake header)

    pub header_size: u64, // fake header (parseable by FFmpeg) prepended on reads
    pub header_block: Vec<u8>, // fake header data (ie. RIFF)

    /*** internal state ***/
    // config
    pub stream_count: i32, /* FFmpeg audio streams (ignores video/etc) */
    pub stream_index: i32,
    pub total_samples: i64, /* may be 0 and innacurate */
    pub skip_samples: i64,  /* number of start samples that will be skipped (encoder delay) */
    pub channel_remap_set: bool,
    pub channel_remap: [i32; 32], /* map of channel > new position */
    pub invert_floats_set: bool,
    pub skip_samples_set: bool, /* flag to know skip samples were manually added from vgmstream */
    pub force_seek: bool,       /* flags for special seeking in faulty formats */
    pub bad_init: bool,

    // FFmpeg context used for metadata
    pub codec: *mut AVCodec,

    /* FFmpeg decoder state */
    pub buffer: *mut u8,
    pub ioCtx: *mut AVIOContext,
    pub formatCtx: *mut AVFormatContext,
    pub codecCtx: *mut AVCodecContext,
    pub frame: *mut AVFrame,   /* last decoded frame */
    pub packet: *mut AVPacket, /* last read data packet */

    pub read_packet: bool,
    pub end_of_stream: bool,
    pub end_of_audio: bool,

    /* sample state */
    pub samples_discard: i32,
    pub samples_consumed: i32,
    pub samples_filled: i32,
}

impl Default for FFmpegCodecData {
    fn default() -> Self {
        Self {
            sf: Some(Default::default()),
            start: 0,
            offset: 0,
            size: 0,
            logical_offset: 0,
            logical_size: 0,
            header_size: 0,
            header_block: Vec::new(),
            stream_count: 0,
            stream_index: 0,
            total_samples: 0,
            skip_samples: 0,
            channel_remap_set: false,
            channel_remap: [0; 32],
            invert_floats_set: false,
            skip_samples_set: false,
            force_seek: false,
            bad_init: false,
            codec: std::ptr::null_mut(),
            buffer: std::ptr::null_mut(),
            ioCtx: std::ptr::null_mut(),
            formatCtx: std::ptr::null_mut(),
            codecCtx: std::ptr::null_mut(),
            frame: std::ptr::null_mut(),
            packet: std::ptr::null_mut(),
            read_packet: false,
            end_of_stream: false,
            end_of_audio: false,
            samples_discard: 0,
            samples_consumed: 0,
            samples_filled: 0,
        }
    }
}

pub struct OpusIOData {
    /* config */
    pub otype: OpusType,
    pub stream_offset: usize,
    pub stream_size: usize,

    /* list of OPUS frame sizes, for variations that preload this (must alloc/dealloc on init/close) */
    pub table_offset: usize,
    pub table_count: i32,
    pub frame_table: Vec<u16>,

    /* fixed frame size for variations that use this */
    pub frame_size: u16,

    /* state */
    pub logical_offset: isize, /* offset that corresponds to physical_offset */
    pub physical_offset: usize, /* actual file offset */

    pub block_size: usize,    /* current block size */
    pub page_size: usize,     /* current OggS page size */
    pub page_buffer: Vec<u8>, /* OggS page (observed max is ~0xc00) */
    pub sequence: usize,      /* OggS sequence */
    pub samples_done: usize,  /* OggS granule */

    pub head_buffer: Vec<u8>, /* OggS head page */
    // pub head_size: usize,     /* OggS head page size */
    pub logical_size: usize,
}

impl Default for OpusIOData {
    fn default() -> Self {
        Self {
            otype: OpusType::OPUS_SWITCH,
            stream_offset: 0,
            stream_size: 0,
            table_offset: 0,
            table_count: 0,
            frame_table: Vec::new(),
            frame_size: 0,
            logical_offset: 0,
            physical_offset: 0,
            block_size: 0,
            page_size: 0,
            page_buffer: vec![0; 0x2000],
            sequence: 0,
            samples_done: 0,
            head_buffer: vec![0; 0x100],
            // head_size: 0,
            logical_size: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpusType {
    OPUS_SWITCH,
    OPUS_UE4_v1,
    OPUS_UE4_v2,
    OPUS_EA,
    OPUS_EA_M,
    OPUS_X,
    OPUS_FSB,
    OPUS_WWISE,
    OPUS_FIXED,
}

pub fn init_ffmpeg_wwise_opus(
    sf: &mut Streamfile,
    data_offset: usize,
    data_size: usize,
    cfg: &mut OpusConfig,
) -> Option<VGMStreamCodecData> {
    return init_ffmpeg_custom_opus_config(sf, data_offset, data_size, cfg, OpusType::OPUS_WWISE);
}

pub fn init_ffmpeg_custom_opus_config(
    sf: &mut Streamfile,
    start_offset: usize,
    data_size: usize,
    cfg: &mut OpusConfig,
    otype: OpusType,
) -> Option<VGMStreamCodecData> {
    use crate::coding::ffmpeg::init_ffmpeg_offset;

    let mut ffmpeg_data: Option<FFmpegCodecData> = None;
    let mut temp_sf: Option<Streamfile> = None;

    temp_sf = setup_opus_streamfile(sf, cfg, start_offset, data_size, otype);
    if temp_sf.is_none() {
        return None;
    }

    let mut temp_sf = temp_sf.as_mut().unwrap();
    let tsize = temp_sf.get_size(std::ptr::null_mut()) as u64;
    ffmpeg_data = init_ffmpeg_offset(&mut temp_sf, 0x00, tsize);
    if ffmpeg_data.is_none() {
        return None;
    }

    let mut ffmpeg_data = ffmpeg_data.unwrap();

    /* FFmpeg + libopus: skips samples, notifies skip in codecCtx->delay/initial_padding (not in stream->skip_samples)
     * FFmpeg + opus: skip samples but loses them on reset/seek to 0, also notifies skip in codecCtx->delay/initial_padding */

    /* quick fix for non-libopus (not sure how to detect better since both share AV_CODEC_ID_OPUS)*/
    unsafe {
        use crate::coding::ffmpeg::{ffmpeg_get_codec_name, ffmpeg_set_force_seek};
        let name = ffmpeg_get_codec_name(&mut ffmpeg_data);
        if name != "" && (name.as_bytes()[0] == b'O' || name.as_bytes()[0] == b'o') {
            /* "Opus" vs "libopus" */
            //ffmpeg_set_skip_samples(ffmpeg_data, cfg.skip); /* can't overwrite internal decoder skip */
            ffmpeg_set_force_seek(&mut ffmpeg_data);
        }
    }

    // temp_sf.close();
    return Some(VGMStreamCodecData::CustomFFmpeg(ffmpeg_data));
}

// TODO: this needs to be actually implemented
pub fn setup_opus_streamfile(
    sf: &mut Streamfile,
    cfg: &mut OpusConfig,
    stream_offset: usize,
    stream_size: usize,
    otype: OpusType,
) -> Option<Streamfile> {
    let mut new_sf: Streamfile = sf.clone();
    let mut io_data: *mut OpusIOData = std::ptr::null_mut();

    io_data = Box::into_raw(Box::new(OpusIOData::default()));

    if cfg.sample_rate == 0 {
        cfg.sample_rate = 48000; /* default / only value for opus */
    }
    unsafe {
        (*io_data).otype = otype;
        (*io_data).stream_offset = stream_offset;
        (*io_data).stream_size = stream_size;
        (*io_data).physical_offset = stream_offset;
        (*io_data).table_offset = cfg.table_offset;
        (*io_data).table_count = cfg.table_count;
        (*io_data).frame_size = cfg.frame_size;

        (*io_data).head_buffer = make_oggs_first(cfg);
        if (*io_data).head_buffer.len() == 0 {
            return None;
        }
    }

    /* setup subfile */
    // new_sf = open_wrap_streamfile(sf);
    // new_sf = open_io_streamfile_ex_f(new_sf, &io_data, sizeof(opus_io_data), opus_io_read, opus_io_size, opus_io_init, opus_io_close);
    new_sf.data = io_data as *mut std::ffi::c_void;
    new_sf.read = Some(opus_io_read);
    new_sf.get_size = Some(opus_io_size);
    new_sf.close = Some(opus_io_close);

    unsafe { opus_io_init(&mut new_sf, io_data) };

    Some(new_sf)
}

/* Convers custom Opus packets to Ogg Opus, so the resulting data is larger than physical data. */
pub fn opus_io_read(
    sf: &mut Streamfile,
    offset: usize,
    length: usize,
    in_data: *mut std::ffi::c_void,
) -> Vec<u8> {
    let mut total_read = 0;
    let mut dest = Vec::new();
    // in_data *mut c_void -> *mut OpusIOData -> &mut OpusIOData
    let mut data = unsafe { &mut *(in_data as *mut OpusIOData) };
    let mut offset = offset;
    let mut length = length;
    /* ignore bad reads */
    if offset > data.logical_size {
        return Vec::new();
    }

    /* previous offset: re-start as we can't map logical<>physical offsets */
    if offset < data.logical_offset as usize || data.logical_offset < 0 {
        data.physical_offset = data.stream_offset;
        data.logical_offset = 0x00;
        data.page_size = 0;
        data.samples_done = 0;
        data.sequence = 2; /* appended header+comment is 0/1 */

        if offset >= data.head_buffer.len() {
            data.logical_offset = data.head_buffer.len() as isize;
        }
    }

    /* insert fake header */
    if offset < data.head_buffer.len() {
        // size_t bytes_consumed, to_read;

        let mut bytes_consumed = offset - data.logical_offset as usize;
        let mut to_read = data.head_buffer.len() - bytes_consumed;
        if to_read > length {
            to_read = length;
        }
        // memcpy(dest, data.head_buffer + bytes_consumed, to_read);
        dest = data.head_buffer[bytes_consumed..bytes_consumed + to_read].to_vec();

        total_read += to_read;
        // dest += to_read;
        offset += to_read;
        length -= to_read;
        data.logical_offset += to_read as isize;
    }

    /* read blocks, one at a time */
    while length > 0 {
        /* ignore EOF */
        if data.logical_offset >= data.logical_size as isize {
            break;
        }

        use crate::streamfile::{read_u16be, read_u16le, read_u32be, read_u8};

        /* process new block */
        if data.page_size == 0 {
            // size_t data_size, skip_size, oggs_size, packet_samples = 0;
            let mut data_size = 0;
            let mut skip_size = 0;
            let mut oggs_size = 0;
            let mut packet_samples = 0;

            match data.otype {
                OpusType::OPUS_SWITCH => {
                    /* format seem to come from opus_test and not Nintendo-specific */
                    data_size = read_u32be(sf, data.physical_offset);
                    skip_size = 0x08; /* size + Opus state(?) */
                }
                OpusType::OPUS_UE4_v1 | OpusType::OPUS_FSB => {
                    data_size = read_u16le(sf, data.physical_offset) as u32;
                    skip_size = 0x02;
                }
                OpusType::OPUS_UE4_v2 => {
                    data_size = read_u16le(sf, data.physical_offset + 0x00) as u32;
                    packet_samples = read_u16le(sf, data.physical_offset + 0x02) as usize;
                    skip_size = 0x02 + 0x02;
                }
                OpusType::OPUS_EA => {
                    data_size = read_u16be(sf, data.physical_offset) as u32;
                    skip_size = 0x02;
                }
                OpusType::OPUS_EA_M => {
                    let mut flag = read_u8(sf, data.physical_offset + 0x00);
                    if flag == 0x48 {
                        /* should start on 0x44 though */
                        data.physical_offset +=
                            read_u16be(sf, data.physical_offset + 0x02) as usize;
                        flag = read_u8(sf, data.physical_offset + 0x00);
                    }
                    data_size = read_u16be(sf, data.physical_offset + 0x02) as u32;
                    skip_size = if flag == 0x45 { data_size } else { 0x08 };
                    data_size -= skip_size;
                }
                OpusType::OPUS_X | OpusType::OPUS_WWISE => {
                    data_size = get_table_frame_size(&*data, data.sequence as i32 - 2) as u32;
                    skip_size = 0;
                }
                OpusType::OPUS_FIXED => {
                    data_size = data.frame_size as u32;
                    skip_size = 0;
                }
                _ => {
                    return Vec::new();
                }
            }

            oggs_size = 0x1b + data_size / 0xFF + 1; /* OggS page: base size + lacing values */

            data.block_size = (data_size + skip_size) as usize;
            data.page_size = (oggs_size + data_size) as usize;

            if data.page_size > data.page_buffer.len() {
                /* happens on bad reads/EOF too */
                println!(
                    "OPUS: buffer can't hold OggS at {:x}, size={:x}\n",
                    data.physical_offset, data.page_size
                );
                data.page_size = 0;
                break;
            }

            /* create fake OggS page (full page for checksums) */
            /* store page data */
            data.page_buffer[oggs_size as usize..(oggs_size + data_size) as usize].copy_from_slice(
                &sf.read(
                    data.physical_offset + skip_size as usize,
                    data_size as usize,
                    sf.data,
                ),
            );
            if packet_samples == 0 {
                packet_samples = opus_get_packet_samples(
                    &data.page_buffer[oggs_size as usize..].to_vec(),
                    data_size as i32,
                );
            }
            data.samples_done += packet_samples;
            make_oggs_page(
                &mut data.page_buffer,
                data_size as i32,
                data.sequence as i32,
                data.samples_done as i32,
            );
            data.sequence += 1;
        }

        /* move to next block */
        if offset >= data.logical_offset as usize + data.page_size {
            data.physical_offset += data.block_size;
            data.logical_offset += data.page_size as isize;
            data.page_size = 0;
            continue;
        }

        /* read data */
        {
            // size_t bytes_consumed, to_read;

            let mut bytes_consumed = offset - data.logical_offset as usize;
            let mut to_read = data.page_size - bytes_consumed;
            if to_read > length {
                to_read = length;
            }
            // memcpy(dest, data.page_buffer + bytes_consumed, to_read);
            dest = data.page_buffer[bytes_consumed..bytes_consumed + to_read].to_vec();
            total_read += to_read;
            // dest += to_read;
            offset += to_read;
            length -= to_read;

            if to_read == 0 {
                break; /* error/EOF */
            }
        }
    }

    let mut file = std::fs::File::create("test_oggdata1.bin").unwrap();
    use std::io::Write;
    file.write_all(&dest.as_slice()).unwrap();

    return dest;
}

pub unsafe fn opus_io_init(sf: &mut Streamfile, data: *mut OpusIOData) {
    //;VGM_LOG("OPUS: init\n");
    /* read table containing frame sizes */
    if (*data).table_count != 0 {
        //;VGM_LOG("OPUS: reading table, offset=%lx, entries=%i\n", data.table_offset, data.table_count);

        // data.frame_table = malloc(data.table_count * sizeof(uint16_t));
        (*data).frame_table = vec![0; (*data).table_count as usize];
        // if data.frame_table.is_none() {
        //     return false;
        // }
        use crate::streamfile::read_u16le;
        for i in 0..(*data).table_count {
            (*data).frame_table[i as usize] =
                read_u16le(sf, (*data).table_offset + i as usize * 0x02);
        }
    }

    (*data).logical_offset = -1; /* force reset in case old data was cloned when re-opening SFs */
    (*data).logical_size = opus_io_size(sf, data as *mut std::ffi::c_void);
    /* force size */
}

pub fn opus_io_close(sf: &mut Streamfile, data: *mut std::ffi::c_void) {
    //;VGM_LOG("OPUS: closing\n");
    let mut data: &mut OpusIOData = unsafe { std::mem::transmute(data) };
    // free(data.frame_table);
    data.frame_table = Vec::new();
}

pub fn opus_io_size(sf: &mut Streamfile, data: *mut std::ffi::c_void) -> usize {
    // off_t offset, max_offset;
    // size_t logical_size = 0;
    let mut dataptr = data as *mut OpusIOData;
    let mut packet = 0;

    let mut data: &mut OpusIOData =  { std::mem::transmute(data) };

    if data.logical_size != 0 {
        return data.logical_size;
    }

    if data.stream_offset + data.stream_size
        > sf.get_size(unsafe { dataptr as *mut std::ffi::c_void })
    {
        println!(
            "OPUS: wrong streamsize {:x} + {:x} vs {:x}\n",
            data.stream_offset,
            data.stream_size,
            sf.get_size(unsafe { dataptr as *mut std::ffi::c_void })
        );
        return 0;
    }

    let mut offset = data.stream_offset;
    let mut max_offset = data.stream_offset + data.stream_size;
    let mut logical_size = data.head_buffer.len();
    use crate::streamfile::{read_u16be, read_u16le, read_u32be, read_u8};
    /* get size of the logical stream */
    while offset < max_offset {
        // size_t data_size, skip_size, oggs_size;
        let mut data_size = 0;
        let mut skip_size = 0;
        let mut oggs_size = 0;

        match data.otype {
            OpusType::OPUS_SWITCH => {
                data_size = read_u32be(sf, offset);
                skip_size = 0x08;
            }
            OpusType::OPUS_UE4_v1 | OpusType::OPUS_FSB => {
                data_size = read_u16le(sf, offset) as u32;
                skip_size = 0x02;
            }
            OpusType::OPUS_UE4_v2 => {
                data_size = read_u16le(sf, offset) as u32;
                skip_size = 0x02 + 0x02;
            }
            OpusType::OPUS_EA => {
                data_size = read_u16be(sf, offset) as u32;
                skip_size = 0x02;
            }
            OpusType::OPUS_EA_M => {
                let mut flag = read_u8(sf, offset + 0x00);
                if flag == 0x48 {
                    offset += read_u16be(sf, offset + 0x02) as usize;
                    flag = read_u8(sf, offset + 0x00);
                }
                data_size = read_u16be(sf, offset + 0x02) as u32;
                skip_size = if flag == 0x45 { data_size } else { 0x08 };
                data_size -= skip_size;
            }
            OpusType::OPUS_X | OpusType::OPUS_WWISE => {
                data_size = get_table_frame_size(&data, packet) as u32;
                skip_size = 0x00;
            }
            OpusType::OPUS_FIXED => {
                data_size = data.frame_size as u32;
                skip_size = 0;
            }
            _ => {
                return 0;
            }
        }

        /* FSB pads data after end (total size without frame headers is given but not too useful here) */
        if (data.otype == OpusType::OPUS_FSB || data.otype == OpusType::OPUS_EA_M) && data_size == 0
        {
            break;
        }

        if data_size == 0 {
            println!("OPUS: data_size is 0 at {:x}\n", offset);
            return 0; /* bad rip? or could 'break' and truck along */
        }

        oggs_size = 0x1b + (data_size / 0xFF + 1); /* OggS page: base size + lacing values */

        offset += (data_size + skip_size) as usize;
        logical_size += (oggs_size + data_size) as usize;
        packet += 1;
    }

    /* logical size can be bigger though */
    if offset > sf.get_size(unsafe { dataptr as *mut std::ffi::c_void }) {
        println!("OPUS: wrong size");
        return 0;
    }

    data.logical_size = logical_size;
    return data.logical_size;
}

pub fn make_oggs_first(cfg: &mut OpusConfig) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![0; 256];
    let page_size = 0x1c; /* fixed for header page */
    /* make header (first data, then page for checksum) */
    let mut bytes = make_opus_header(&mut buf[page_size..].as_mut(), cfg);

    // let mut file = std::fs::File::create("test_head_opusheader.bin").unwrap();
    // use std::io::Write;
    // file.write_all(&buf).unwrap();

    make_oggs_page(&mut buf.as_mut_slice(), bytes, 0, 0);

    let mut buf_done = page_size as i32 + bytes;

    // let mut file = std::fs::File::create("test_head_oggspage.bin").unwrap();
    // file.write_all(&buf).unwrap();

    /* make comment */
    bytes = make_opus_comment(&mut buf[page_size..].as_mut());

    // let mut file = std::fs::File::create("test_head_opuscomment.bin").unwrap();
    // file.write_all(&buf).unwrap();

    make_oggs_page(&mut buf.as_mut_slice(), bytes, 1, 0);
    // buf[..bytes as usize].copy_from_slice(&buf2[..bytes as usize]);

    buf_done += page_size as i32 + bytes;
    buf = buf[..buf_done as usize].to_vec();

    let mut file = std::fs::File::create("test_head_all.bin").unwrap();
    use std::io::Write;
    file.write_all(&buf.as_slice()).unwrap();

    return buf.to_vec();
}

pub fn make_opus_header(buf: &mut [u8], cfg: &mut OpusConfig) -> i32 {
    let mut header_size: i32 = 0x13;
    let mut mapping_family: u8 = 0;

    /* Opus can't play a Nch file unless the channel mapping is properly configured (not implicit).
     * A 8ch file may be 2ch+2ch+1ch+1ch+2ch; this is defined with a "channel mapping":
     * - mapping family:
     *   0 = standard (single stream mono/stereo, >2ch = error, and table MUST be ommited)
     *   1 = standard multichannel (1..8ch), using Vorbis channel layout (needs table)
     *   255 = undefined (1..255ch)  application defined (needs table)
     * - mapping table:
     *   - stream count: internal opus streams (>= 1), of 1/2ch
     *   - coupled count: internal stereo streams (<= streams)
     *   - mappings: one byte per channel with the channel position (0..Nch), or 255 (silence)
     */

    /* set mapping family */
    if cfg.channels > 2 || cfg.stream_count > 1 {
        mapping_family = 1; //todo test 255
        header_size += 0x01 + 0x01 + cfg.channels as i32; /* table size */
    }

    if cfg.skip < 0 {
        println!("OPUS: wrong skip {}", cfg.skip);
        cfg.skip = 0; /* ??? */
    }

    if header_size > buf.len() as i32 {
        println!("OPUS: buffer can't hold header");
        return 0;
    }

    use crate::streamfile::get_id32be;

    buf[0x00..0x04].copy_from_slice(&get_id32be("Opus").to_be_bytes());
    buf[0x04..0x08].copy_from_slice(&get_id32be("Head").to_be_bytes());
    buf[0x08] = 1; /* version */
    buf[0x09] = cfg.channels;
    buf[0x0A..0x0C].copy_from_slice(&(cfg.skip as i16).to_le_bytes());
    buf[0x0C..0x10].copy_from_slice(&cfg.sample_rate.to_le_bytes());
    buf[0x10..0x12].copy_from_slice(&0i16.to_le_bytes()); /* output gain */
    buf[0x12] = mapping_family;

    // put_u32be(buf+0x00, get_id32be("Opus"));
    // put_u32be(buf+0x04, get_id32be("Head"));
    // put_u8   (buf+0x08, 1); /* version */
    // put_u8   (buf+0x09, cfg.channels);
    // put_s16le(buf+0x0A, cfg.skip);
    // put_u32le(buf+0x0c, cfg.sample_rate);
    // put_u16le(buf+0x10, 0); /* output gain */
    // put_u8   (buf+0x12, mapping_family);

    /* set mapping table */
    if mapping_family > 0 {
        /* total streams (mono/stereo) */
        // put_u8(buf+0x13, cfg.stream_count);
        buf[0x13] = cfg.stream_count as u8;
        /* stereo streams (6ch can be 2ch+2ch+1ch+1ch = 2 coupled in 4 streams) */
        // put_u8(buf+0x14, cfg.coupled_count);
        buf[0x14] = cfg.coupled_count as u8;
        /* mapping per channel (order of channels, ex: 00 01 04 05 02 03) */
        for i in 0..cfg.channels {
            // put_u8(buf+0x15+i, cfg.channel_mapping[i]);
            buf[0x15 + i as usize] = cfg.channel_mapping[i as usize] as u8;
        }
    }

    return header_size;
}

pub fn make_oggs_page(buf: &mut [u8], data_size: i32, page_sequence: i32, granule: i32) -> i32 {
    // size_t page_done, lacing_done = 0;
    let absolute_granule: u64 = granule as u64; /* wrong values seem validated (0, less than real samples, etc) */
    // let mut  header_type_flag = (page_sequence==0 ? 2 : 0);
    // let mut  stream_serial_number = 0x7667; /* 0 is legal, but should be specified */
    // let mut  checksum = 0;
    // let mut  segment_count;

    let header_type_flag = if page_sequence == 0 { 2 } else { 0 };
    let stream_serial_number = 0x7667;

    if 0x1b + (data_size / 0xFF + 1) + data_size > buf.len() as i32 {
        println!("OPUS: buffer can't hold OggS page\n");
        return 0;
    }

    use crate::streamfile::get_id32be;

    let segment_count = data_size / 0xFF + 1;
    buf[0x00..0x04].copy_from_slice(&get_id32be("OggS").to_be_bytes());
    buf[0x04] = 0;
    buf[0x05] = header_type_flag;
    buf[0x06..0x0A].copy_from_slice(&((absolute_granule >> 0 & 0xFFFFFFFF) as u32).to_le_bytes());
    buf[0x0A..0x0E].copy_from_slice(&((absolute_granule >> 32 & 0xFFFFFFFF) as u32).to_le_bytes());
    buf[0x0E..0x12].copy_from_slice(&(stream_serial_number as u32).to_le_bytes());
    buf[0x12..0x16].copy_from_slice(&(page_sequence as u32).to_le_bytes());
    buf[0x16..0x1A].copy_from_slice(&0u32.to_le_bytes());
    buf[0x1A] = segment_count as u8;

    /* segment table: size N in "lacing values" (ex. 0x20E=0xFF+FF+10; 0xFF=0xFF+00) */
    let mut page_done = 0x1B;
    let mut lacing_done = 0;
    while lacing_done < data_size {
        let mut bytes = data_size - lacing_done;
        if bytes > 0xFF {
            bytes = 0xFF;
        }

        // put_u8(buf+page_done, bytes);
        buf[page_done] = bytes as u8;
        page_done += 1;
        lacing_done += bytes;

        if lacing_done == data_size && bytes == 0xFF {
            // put_u8(buf+page_done, 0x00);
            buf[page_done] = 0x00;
            page_done += 1;
        }
    }

    /* data */
    page_done += data_size as usize;

    /* final checksum */
    let checksum = get_oggs_checksum(buf, page_done as i32);
    // put_u32le(buf+0x16, checksum);
    buf[0x16..0x1A].copy_from_slice(&checksum.to_le_bytes());

    let mut file = std::fs::File::create("test_head_oggspage.bin").unwrap();
    use std::io::Write;
    file.write_all(&buf[..page_done]).unwrap();

    return page_done as i32;
}

pub fn make_opus_comment(buf: &mut [u8]) -> i32 {
    let vendor_string = "vgmstream";
    let user_comment_0_string = "vgmstream Opus converter";
    let comment_size = 0x14 + vendor_string.len() + user_comment_0_string.len();

    if comment_size > buf.len() {
        println!("OPUS: buffer can't hold comment");
        return 0;
    }

    buf[0x00..0x04].copy_from_slice(&0x4F707573u32.to_be_bytes());
    buf[0x04..0x08].copy_from_slice(&0x54616773u32.to_be_bytes());
    buf[0x08..0x0C].copy_from_slice(&(vendor_string.len() as u32).to_le_bytes());
    buf[0x0C..0x0C + vendor_string.len()].copy_from_slice(vendor_string.as_bytes());
    buf[0x0c + vendor_string.len() + 0x00..0x0c + vendor_string.len() + 0x04]
        .copy_from_slice(&1u32.to_le_bytes());
    buf[0x0c + vendor_string.len() + 0x04..0x0c + vendor_string.len() + 0x08]
        .copy_from_slice(&(user_comment_0_string.len() as u32).to_le_bytes());
    buf[0x0c + vendor_string.len() + 0x08
        ..0x0c + vendor_string.len() + 0x08 + user_comment_0_string.len()]
        .copy_from_slice(user_comment_0_string.as_bytes());

    return comment_size as i32;
}

/* from ww2ogg - from Tremor (lowmem) */
pub const CRC_LOOKUP: [u32; 256] = [
    0x00000000, 0x04c11db7, 0x09823b6e, 0x0d4326d9, 0x130476dc, 0x17c56b6b, 0x1a864db2, 0x1e475005,
    0x2608edb8, 0x22c9f00f, 0x2f8ad6d6, 0x2b4bcb61, 0x350c9b64, 0x31cd86d3, 0x3c8ea00a, 0x384fbdbd,
    0x4c11db70, 0x48d0c6c7, 0x4593e01e, 0x4152fda9, 0x5f15adac, 0x5bd4b01b, 0x569796c2, 0x52568b75,
    0x6a1936c8, 0x6ed82b7f, 0x639b0da6, 0x675a1011, 0x791d4014, 0x7ddc5da3, 0x709f7b7a, 0x745e66cd,
    0x9823b6e0, 0x9ce2ab57, 0x91a18d8e, 0x95609039, 0x8b27c03c, 0x8fe6dd8b, 0x82a5fb52, 0x8664e6e5,
    0xbe2b5b58, 0xbaea46ef, 0xb7a96036, 0xb3687d81, 0xad2f2d84, 0xa9ee3033, 0xa4ad16ea, 0xa06c0b5d,
    0xd4326d90, 0xd0f37027, 0xddb056fe, 0xd9714b49, 0xc7361b4c, 0xc3f706fb, 0xceb42022, 0xca753d95,
    0xf23a8028, 0xf6fb9d9f, 0xfbb8bb46, 0xff79a6f1, 0xe13ef6f4, 0xe5ffeb43, 0xe8bccd9a, 0xec7dd02d,
    0x34867077, 0x30476dc0, 0x3d044b19, 0x39c556ae, 0x278206ab, 0x23431b1c, 0x2e003dc5, 0x2ac12072,
    0x128e9dcf, 0x164f8078, 0x1b0ca6a1, 0x1fcdbb16, 0x018aeb13, 0x054bf6a4, 0x0808d07d, 0x0cc9cdca,
    0x7897ab07, 0x7c56b6b0, 0x71159069, 0x75d48dde, 0x6b93dddb, 0x6f52c06c, 0x6211e6b5, 0x66d0fb02,
    0x5e9f46bf, 0x5a5e5b08, 0x571d7dd1, 0x53dc6066, 0x4d9b3063, 0x495a2dd4, 0x44190b0d, 0x40d816ba,
    0xaca5c697, 0xa864db20, 0xa527fdf9, 0xa1e6e04e, 0xbfa1b04b, 0xbb60adfc, 0xb6238b25, 0xb2e29692,
    0x8aad2b2f, 0x8e6c3698, 0x832f1041, 0x87ee0df6, 0x99a95df3, 0x9d684044, 0x902b669d, 0x94ea7b2a,
    0xe0b41de7, 0xe4750050, 0xe9362689, 0xedf73b3e, 0xf3b06b3b, 0xf771768c, 0xfa325055, 0xfef34de2,
    0xc6bcf05f, 0xc27dede8, 0xcf3ecb31, 0xcbffd686, 0xd5b88683, 0xd1799b34, 0xdc3abded, 0xd8fba05a,
    0x690ce0ee, 0x6dcdfd59, 0x608edb80, 0x644fc637, 0x7a089632, 0x7ec98b85, 0x738aad5c, 0x774bb0eb,
    0x4f040d56, 0x4bc510e1, 0x46863638, 0x42472b8f, 0x5c007b8a, 0x58c1663d, 0x558240e4, 0x51435d53,
    0x251d3b9e, 0x21dc2629, 0x2c9f00f0, 0x285e1d47, 0x36194d42, 0x32d850f5, 0x3f9b762c, 0x3b5a6b9b,
    0x0315d626, 0x07d4cb91, 0x0a97ed48, 0x0e56f0ff, 0x1011a0fa, 0x14d0bd4d, 0x19939b94, 0x1d528623,
    0xf12f560e, 0xf5ee4bb9, 0xf8ad6d60, 0xfc6c70d7, 0xe22b20d2, 0xe6ea3d65, 0xeba91bbc, 0xef68060b,
    0xd727bbb6, 0xd3e6a601, 0xdea580d8, 0xda649d6f, 0xc423cd6a, 0xc0e2d0dd, 0xcda1f604, 0xc960ebb3,
    0xbd3e8d7e, 0xb9ff90c9, 0xb4bcb610, 0xb07daba7, 0xae3afba2, 0xaafbe615, 0xa7b8c0cc, 0xa379dd7b,
    0x9b3660c6, 0x9ff77d71, 0x92b45ba8, 0x9675461f, 0x8832161a, 0x8cf30bad, 0x81b02d74, 0x857130c3,
    0x5d8a9099, 0x594b8d2e, 0x5408abf7, 0x50c9b640, 0x4e8ee645, 0x4a4ffbf2, 0x470cdd2b, 0x43cdc09c,
    0x7b827d21, 0x7f436096, 0x7200464f, 0x76c15bf8, 0x68860bfd, 0x6c47164a, 0x61043093, 0x65c52d24,
    0x119b4be9, 0x155a565e, 0x18197087, 0x1cd86d30, 0x029f3d35, 0x065e2082, 0x0b1d065b, 0x0fdc1bec,
    0x3793a651, 0x3352bbe6, 0x3e119d3f, 0x3ad08088, 0x2497d08d, 0x2056cd3a, 0x2d15ebe3, 0x29d4f654,
    0xc5a92679, 0xc1683bce, 0xcc2b1d17, 0xc8ea00a0, 0xd6ad50a5, 0xd26c4d12, 0xdf2f6bcb, 0xdbee767c,
    0xe3a1cbc1, 0xe760d676, 0xea23f0af, 0xeee2ed18, 0xf0a5bd1d, 0xf464a0aa, 0xf9278673, 0xfde69bc4,
    0x89b8fd09, 0x8d79e0be, 0x803ac667, 0x84fbdbd0, 0x9abc8bd5, 0x9e7d9662, 0x933eb0bb, 0x97ffad0c,
    0xafb010b1, 0xab710d06, 0xa6322bdf, 0xa2f33668, 0xbcb4666d, 0xb8757bda, 0xb5365d03, 0xb1f740b4,
];

/* from ww2ogg */
pub fn get_oggs_checksum(data: &mut [u8], bytes: i32) -> u32 {
    let mut crc_reg: u32 = 0;

    for i in 0..bytes {
        crc_reg = (crc_reg << 8)
            ^ CRC_LOOKUP[(((crc_reg >> 24) & 0xff) as u32 ^ data[i as usize] as u32) as usize];
    }

    return crc_reg;
}

impl Streamfile {
    pub fn opus_io_read(&mut self, offset: usize, length: usize, data: &mut OpusIOData) -> Vec<u8> {
        let mut total_read = 0;
        let mut dest: Vec<u8> = Vec::new();
        let mut offset = offset;
        let mut length = length;
        /* ignore bad reads */
        if offset < 0 || offset > data.logical_size {
            return Vec::new();
        }

        /* previous offset: re-start as we can't map logical<>physical offsets */
        if offset < data.logical_offset as usize || data.logical_offset < 0 {
            data.physical_offset = data.stream_offset;
            data.logical_offset = 0x00;
            data.page_size = 0;
            data.samples_done = 0;
            data.sequence = 2; /* appended header+comment is 0/1 */

            if offset >= data.head_buffer.len() {
                data.logical_offset = data.head_buffer.len() as isize;
            }
        }

        /* insert fake header */
        if offset < data.head_buffer.len() {
            // size_t bytes_consumed, to_read;

            let mut bytes_consumed = offset - data.logical_offset as usize;
            let mut to_read = data.head_buffer.len() - bytes_consumed;
            if to_read > length {
                to_read = length;
            }
            // memcpy(dest, data.head_buffer + bytes_consumed, to_read);
            dest = data.head_buffer[bytes_consumed..bytes_consumed + to_read].to_vec();

            let mut file = std::fs::File::create("test_append_head.bin").unwrap();
            use std::io::Write;
            file.write_all(&dest.as_slice()).unwrap();

            total_read += to_read;
            //dest += to_read;
            offset += to_read;
            length -= to_read;
            data.logical_offset += to_read as isize;
        }

        /* read blocks, one at a time */
        while length > 0 {
            /* ignore EOF */
            if data.logical_offset >= data.logical_size as isize {
                break;
            }

            /* process new block */
            if data.page_size == 0 {
                // size_t data_size, skip_size, oggs_size, packet_samples = 0;
                let mut data_size = 0;
                let mut skip_size = 0;
                let mut packet_samples = 0;
                use crate::streamfile::{read_u16be, read_u16le, read_u32be, read_u8};
                match data.otype {
                    OpusType::OPUS_SWITCH => {
                        /* format seem to come from opus_test and not Nintendo-specific */
                        data_size = read_u32be(self, data.physical_offset);
                        skip_size = 0x08; /* size + Opus state(?) */
                    }
                    OpusType::OPUS_UE4_v1 | OpusType::OPUS_FSB => {
                        data_size = read_u16le(self, data.physical_offset) as u32;
                        skip_size = 0x02;
                    }
                    OpusType::OPUS_UE4_v2 => {
                        data_size = read_u16le(self, data.physical_offset + 0x00) as u32;
                        packet_samples = read_u16le(self, data.physical_offset + 0x02);
                        skip_size = 0x02 + 0x02;
                    }
                    OpusType::OPUS_EA => {
                        data_size = read_u16be(self, data.physical_offset) as u32;
                        skip_size = 0x02;
                    }
                    OpusType::OPUS_EA_M => {
                        let mut flag = read_u8(self, data.physical_offset + 0x00);
                        if flag == 0x48 {
                            /* should start on 0x44 though */
                            data.physical_offset +=
                                read_u16be(self, data.physical_offset + 0x02) as usize;
                            flag = read_u8(self, data.physical_offset + 0x00);
                        }
                        data_size = read_u16be(self, data.physical_offset + 0x02) as u32;
                        skip_size = if flag == 0x45 { data_size } else { 0x08 };
                        data_size -= skip_size;
                    }
                    OpusType::OPUS_X | OpusType::OPUS_WWISE => {
                        data_size = get_table_frame_size(data, data.sequence as i32 - 2) as u32;
                        skip_size = 0;
                    }
                    OpusType::OPUS_FIXED => {
                        data_size = data.frame_size as u32;
                        skip_size = 0;
                    }
                    _ => {
                        return Vec::new();
                    }
                }

                let mut oggs_size = 0x1b + (data_size / 0xFF + 1); /* OggS page: base size + lacing values */

                data.block_size = (data_size + skip_size) as usize;
                data.page_size = (oggs_size + data_size) as usize;

                if data.page_size > data.page_buffer.len() {
                    /* happens on bad reads/EOF too */
                    println!(
                        "OPUS: buffer can't hold OggS at {:x}, size={:x}\n",
                        data.physical_offset, data.page_size
                    );
                    data.page_size = 0;
                }

                /* create fake OggS page (full page for checksums) */
                // read_streamfile(data.page_buffer+oggs_size, data.physical_offset + skip_size, data_size, sf); /* store page data */
                if packet_samples == 0 {
                    packet_samples = opus_get_packet_samples(
                        &data.page_buffer[oggs_size as usize..].to_vec(),
                        data_size as i32,
                    ) as u16;
                }
                data.samples_done += packet_samples as usize;
                make_oggs_page(
                    &mut data.page_buffer,
                    data_size as i32,
                    data.sequence as i32,
                    data.samples_done as i32,
                );
                data.sequence += 1;
            }

            /* move to next block */
            if offset >= data.logical_offset as usize + data.page_size {
                data.physical_offset += data.block_size;
                data.logical_offset += data.page_size as isize;
                data.page_size = 0;
                continue;
            }

            /* read data */
            {
                // size_t bytes_consumed, to_read;

                let mut bytes_consumed = offset - data.logical_offset as usize;
                let mut to_read = data.page_size - bytes_consumed;
                if to_read > length {
                    to_read = length;
                }
                // memcpy(dest, data.page_buffer + bytes_consumed, to_read);
                dest = data.page_buffer[bytes_consumed..bytes_consumed + to_read].to_vec();

                total_read += to_read;
                // dest += to_read;
                offset += to_read;
                length -= to_read;

                if to_read == 0 {
                    break; /* error/EOF */
                }
            }
        }

        let mut file = std::fs::File::create("test_ogg_data.bin").unwrap();
        use std::io::Write;
        file.write_all(&dest.as_slice()).unwrap();

        return dest;
    }
}

pub fn opus_get_packet_samples(buf: &Vec<u8>, len: i32) -> usize {
    return opus_packet_get_nb_frames(buf, len) as usize
        * opus_packet_get_samples_per_frame(buf, 48000) as usize;
}

/* from opus_decoder.c's opus_packet_get_samples_per_frame */
pub fn opus_packet_get_samples_per_frame(data: &Vec<u8>, fs: u32) -> u32 {
    let mut audiosize: u32 = 0;
    if data[0] & 0x80 != 0 {
        audiosize = (data[0] as u32 >> 3) & 0x3;
        audiosize = (fs << audiosize) / 400;
    } else if (data[0] & 0x60) == 0x60 {
        audiosize = if data[0] & 0x08 != 0 {
            fs / 50
        } else {
            fs / 100
        };
    } else {
        audiosize = (data[0] as u32 >> 3) & 0x3;
        if audiosize == 3 {
            audiosize = fs * 60 / 1000;
        } else {
            audiosize = (fs << audiosize) / 100;
        }
    }
    return audiosize;
}

/* from opus_decoder.c's opus_packet_get_nb_frames */
pub fn opus_packet_get_nb_frames(packet: &Vec<u8>, len: i32) -> i32 {
    let mut count = 0;
    if len < 1 {
        return 0;
    }
    count = packet[0] as i32 & 0x3;
    if count == 0 {
        return 1;
    } else if count != 3 {
        return 2;
    } else if len < 2 {
        return 0;
    } else {
        return packet[1] as i32 & 0x3F;
    }
}

/* some formats store all frames in a table, rather than right before the frame */
pub fn get_table_frame_size(data: &OpusIOData, frame: i32) -> u16 {
    if frame < 0 || frame >= data.table_count {
        println!(
            "OPUS: wrong requested frame {}, count={}\n",
            frame, data.table_count
        );
        return 0;
    }

    //;VGM_LOG("OPUS: frame %i size=%x\n", frame, data.frame_table[frame]);
    return data.frame_table[frame as usize];
}
