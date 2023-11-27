use crate::vgmstream::{VGMStream, CodingType};
use crate::coding::adx::*;
/* Decode samples into the buffer. Assume that we have written samples_written into the
 * buffer already, and we have samples_to_do consecutive samples ahead of us (won't call
 * more than one frame if configured above to do so).
 * Called by layouts since they handle samples written/to_do */
pub fn decode_vgmstream(vgmstream: &mut VGMStream, samples_written: i32, samples_to_do: i32, buffer: &mut Vec<i16>) {
    // C code: buffer += samples_written * vgmstream.channels; /* passed externally to simplify I guess */
    let mut output_buffer = &mut buffer.clone()[(samples_written * vgmstream.channels) as usize..];
    match vgmstream.coding_type {
        CodingType::coding_SILENCE => {
            //memset(buffer, 0, samples_to_do * vgmstream->channels * sizeof(sample_t));
            for i in 0..samples_to_do * vgmstream.channels {
                output_buffer[i as usize] = 0;
            }
            return;
        },
        CodingType::coding_CRI_ADX |
        CodingType::coding_CRI_ADX_exp |
        CodingType::coding_CRI_ADX_fixed |
        CodingType::coding_CRI_ADX_enc_8 |
        CodingType::coding_CRI_ADX_enc_9 => {
            for ch in 0..vgmstream.channels {
                decode_adx(&mut vgmstream.ch[ch as usize], &mut output_buffer[ch as usize..], vgmstream.channels, vgmstream.samples_into_block as i32, samples_to_do, vgmstream.interleave_block_size as i32, vgmstream.coding_type, vgmstream.codec_config as u32);
                // buffer[(samples_written * vgmstream.channels + ch) as usize] = output_buffer.to_vec();
                // replace the buffer with the output buffer, from ch onwards
                buffer[(samples_written * vgmstream.channels + ch) as usize..].copy_from_slice(&output_buffer[ch as usize..]);
            }
        },
        CodingType::coding_FFmpeg => {
            use crate::coding::ffmpeg::decode_ffmpeg;
            decode_ffmpeg(vgmstream, buffer, samples_to_do, vgmstream.channels);
        }
        _ => {
            return;
        }
    }
}

pub fn decode_get_frame_size(vgmstream: &VGMStream) -> i32 {
    match vgmstream.coding_type {
        CodingType::coding_SILENCE => {
            return 0;
        },
        CodingType::coding_CRI_ADX |
        CodingType::coding_CRI_ADX_fixed |
        CodingType::coding_CRI_ADX_exp |
        CodingType::coding_CRI_ADX_enc_8 |
        CodingType::coding_CRI_ADX_enc_9 => {
            return vgmstream.interleave_block_size as i32;
        }
        _ => {
            return 0;
        }
    }
}

pub fn decode_get_samples_per_frame(vgmstream: &VGMStream) -> i32 {
    /* Value returned here is the max (or less) that vgmstream will ask a decoder per
     * "decode_x" call. Decoders with variable samples per frame or internal discard
     * may return 0 here and handle arbitrary samples_to_do values internally
     * (or some internal sample buffer max too). */

    match vgmstream.coding_type {
        CodingType::coding_SILENCE => {
            return 0;
        }
        CodingType::coding_CRI_ADX |
        CodingType::coding_CRI_ADX_fixed |
        CodingType::coding_CRI_ADX_exp |
        CodingType::coding_CRI_ADX_enc_8 |
        CodingType::coding_CRI_ADX_enc_9 => {
            return (vgmstream.interleave_block_size as i32 - 2) * 2;
        }
        _ => {
            return 0;
        }
    }
}
