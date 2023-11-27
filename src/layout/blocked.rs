use crate::vgmstream::VGMStream;
use crate::decode::*;

pub fn render_vgmstream_blocked(sample_count: i32, vgmstream: &mut VGMStream) -> Option<Vec<i16>> {
    let mut frame_size = decode_get_frame_size(&vgmstream);
    let mut samples_per_frame = decode_get_samples_per_frame(&vgmstream);
    let mut samples_this_block = 0;
    let mut samples_written = 0;

    if vgmstream.current_block_samples != 0 {
        samples_this_block = vgmstream.current_block_samples;
    } else if frame_size == 0 { /* assume 4 bit */ //TODO: decode_get_frame_size() really should return bits... */
        samples_this_block = vgmstream.current_block_size as i32 * 2 * samples_per_frame;
    } else {
        samples_this_block = vgmstream.current_block_size as i32 / frame_size * samples_per_frame;
    }

    let mut output_buffer: Option<Vec<i16>> = None;

    while samples_written < sample_count {
        let mut samples_to_do = 0; 


        // TODO: handle looping

        // if vgmstream.loop_flag && decode_do_loop(vgmstream) {
        //     /* handle looping, readjust back to loop start values */
        //     if (vgmstream->current_block_samples) {
        //         samples_this_block = vgmstream->current_block_samples;
        //     } else if (frame_size == 0) { /* assume 4 bit */ //TODO: decode_get_frame_size() really should return bits... */
        //         samples_this_block = vgmstream->current_block_size * 2 * samples_per_frame;
        //     } else {
        //         samples_this_block = vgmstream->current_block_size / frame_size * samples_per_frame;
        //     }
        //     continue;
        // }

        if samples_this_block < 0 {
            /* probably block bug or EOF, next calcs would give wrong values/segfaults/infinite loop */
            println!("layout_blocked: wrong block samples at 0x{:x}", vgmstream.current_block_offset);
            // memset(buffer + samples_written*vgmstream->channels, 0, (sample_count - samples_written) * vgmstream->channels * sizeof(sample_t));
            output_buffer = None;
            break;
        }

        if vgmstream.current_block_offset < 0 || vgmstream.current_block_offset == 0xFFFFFFFF {
            /* probably block bug or EOF, block functions won't be able to read anything useful/infinite loop */
            println!("layout_blocked: wrong block offset found");
            // memset(buffer + samples_written*vgmstream->channels, 0, (sample_count - samples_written) * vgmstream->channels * sizeof(sample_t));
            output_buffer = None;
            break;
        }

        samples_to_do = vgmstream.decode_get_samples_to_do(samples_this_block, samples_per_frame);
        if samples_to_do > sample_count - samples_written {
            samples_to_do = sample_count - samples_written;
        }

        if samples_to_do > 0 {
            /* samples_this_block = 0 is allowed (empty block, do nothing then move to next block) */
            // output_buffer = decode_vgmstream(vgmstream, samples_written, samples_to_do);
        }

        samples_written += samples_to_do;
        vgmstream.current_sample += samples_to_do as isize;
        vgmstream.samples_into_block += samples_to_do as isize;


        /* move to next block when all samples are consumed */
        if vgmstream.samples_into_block == samples_this_block as isize { /* don't go past last block */ //todo
            // block_update(vgmstream->next_block_offset,vgmstream);

            /* update since these may change each block */
            frame_size = decode_get_frame_size(vgmstream);
            samples_per_frame = decode_get_samples_per_frame(vgmstream);
            if vgmstream.current_block_samples != 0 {
                samples_this_block = vgmstream.current_block_samples;
            } else if frame_size == 0 { /* assume 4 bit */ //TODO: decode_get_frame_size() really should return bits... */
                samples_this_block = vgmstream.current_block_size as i32 * 2 * samples_per_frame;
            } else {
                samples_this_block = vgmstream.current_block_size as i32 / frame_size * samples_per_frame;
            }

            vgmstream.samples_into_block = 0;
        }

    }


    return output_buffer;
}