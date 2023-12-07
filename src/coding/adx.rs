use crate::util::{reader::*, util::clamp16};
use crate::vgmstream::{CodingType, VGMStreamChannel};

pub fn decode_adx(
    stream: &mut VGMStreamChannel,
    outbuf: &mut [i16],
    channelspacing: i32,
    first_sample: i32,
    samples_to_do: i32,
    frame_size: i32,
    coding_type: CodingType,
    codec_config: u32,
) {
    // let frame: [u8; 0x12] = [0; 0x12];
    
    let mut sample_count = 0;
    // int i, frames_in, sample_count = 0;
    // size_t bytes_per_frame, samples_per_frame;
    // int scale, coef1, coef2;
    let mut hist1 = stream.adpcm_history1_32;
    let mut hist2 = stream.adpcm_history2_32;
    let version = codec_config;

    /* external interleave (fixed size), mono */
    let bytes_per_frame = frame_size;
    let samples_per_frame = (bytes_per_frame - 0x02) * 2; /* always 32 */
    let frames_in = first_sample / samples_per_frame;
    let first_sample = first_sample % samples_per_frame;

    /* parse frame header */
    let frame_offset = stream.offset as i32 + bytes_per_frame * frames_in;

    use std::io::{Seek, Read};
    let mut frame = vec![0; bytes_per_frame as usize];
    stream.streamfile.as_mut().unwrap().reader
        .seek(std::io::SeekFrom::Start(frame_offset as u64))
        .unwrap();
    stream.streamfile.as_mut().unwrap().reader.read_exact(&mut frame).unwrap();
    // println!("{:X?}", frame);

    // read_streamfile(frame, frame_offset, bytes_per_frame, stream.streamfile); /* ignore EOF errors */
    let mut coef1 = 0;
    let mut coef2 = 0;
    let mut scale = get_s16be(&frame[0x00..]);
    match coding_type {
        CodingType::coding_CRI_ADX => {
            scale += 1;
            coef1 = stream.adpcm_coef[0];
            coef2 = stream.adpcm_coef[1];

            /* Detect EOF scale (0x8001) found in some ADX of any type, signals "stop decoding" (without this frame?).
             * Normally num_samples stops right before it, but ADXPLAY will honor it even in the middle on a file
             * (may repeat last sample buffer). Some Baroque (SAT) videos set it on file end, but num_samples goes beyond.
             * Just the upper bit triggers it even in encrypted ADX (max is 0x7FFF), but the check only here just in case. */
            if frame[0] == 0x80 && frame[1] == 0x01 {
                scale = 0; /* fix scaled click, maybe should just exit */
                println!("ADX: reached EOF scale");
            }
        }
        CodingType::coding_CRI_ADX_exp => {
            scale = 1 << (12 - scale);
            coef1 = stream.adpcm_coef[0];
            coef2 = stream.adpcm_coef[1];
        }
        CodingType::coding_CRI_ADX_fixed => {
            scale = (scale & 0x1fff) + 1;
            coef1 = stream.adpcm_coef[(&frame[0] >> 5) as usize * 2];
            coef2 = stream.adpcm_coef[(&frame[0] >> 5) as usize * 2 + 1];
        }
        CodingType::coding_CRI_ADX_enc_8 | CodingType::coding_CRI_ADX_enc_9 => {
            scale = ((scale ^ stream.adx_xor as i16) & 0x1fff) + 1; /* this seems to be used even in unencrypted ADX (compatible) */
            coef1 = stream.adpcm_coef[0];
            coef2 = stream.adpcm_coef[1];
        }
        _ => {
            scale += 1;
            coef1 = stream.adpcm_coef[0];
            coef2 = stream.adpcm_coef[1];
        }
    }

    /* decode nibbles */
    let mut count = 0;
    for i in first_sample..(first_sample + samples_to_do) {
        count = i;
        let mut sample = 0;
        let nibbles = frame[0x02 + i as usize / 2];

        sample = if i & 1 == 1 {
            /* high nibble first */
            get_low_nibble_signed(nibbles)
        } else {
            get_high_nibble_signed(nibbles)
        };

        /* Early (v3 ADX only) libs decode slightly differently (quieter?), while later libs (v4 ADX) tweaked it. V4 libs playing v3 files
         * seem to behave like V4 though, so it's not detectable but not that common (ex. ports of old games reusing v3 ADX) */
        if version == 0x0300 {
            sample = sample * scale as i32
                + ((coef1 as i32 * hist1) >> 12)
                + ((coef2 as i32 * hist2) >> 12); /* V3 lib */
        } else {
            sample = sample * scale as i32 + ((coef1 as i32 * hist1 + coef2 as i32 * hist2) >> 12);
            /* V4 lib */
        }
        sample = clamp16(sample);
        // println!("{:X?}", sample);
        outbuf[sample_count] = sample as i16;
        sample_count += channelspacing as usize;

        hist2 = hist1;
        hist1 = sample;
    }

    stream.adpcm_history1_32 = hist1;
    stream.adpcm_history2_32 = hist2;

    if (coding_type == CodingType::coding_CRI_ADX_enc_8
        || coding_type == CodingType::coding_CRI_ADX_enc_9)
        && (count % 32) == 0
    {
        for _ in 0..stream.adx_channels {
            adx_next_key(stream);
        }
    }
}

fn adx_next_key(stream: &mut VGMStreamChannel) {
    stream.adx_xor = (stream.adx_xor * stream.adx_mult + stream.adx_add) & 0x7fff;
}
