use crate::decode::*;
use crate::vgmstream::VGMStream;

pub fn render_vgmstream_interleave(
    buffer: &mut Vec<i16>,
    sample_count: i32,
    vgmstream: &mut VGMStream,
) {
    let mut samples_written = 0;
    let mut samples_per_frame;
    let mut samples_this_block; /* used */
    let mut samples_per_frame_d = 0;
    let mut samples_this_block_d = 0; /* default */
    let mut samples_per_frame_f = 0;
    let mut samples_this_block_f = 0; /* first */
    let mut samples_per_frame_l = 0;
    let mut samples_this_block_l = 0; /* last */
    let mut has_interleave_first =
        vgmstream.interleave_first_block_size == 1 && vgmstream.channels > 1;
    let mut has_interleave_last =
        vgmstream.interleave_last_block_size == 1 && vgmstream.channels > 1;

    /* setup */
    let frame_size_d = decode_get_frame_size(vgmstream);
    samples_per_frame_d = decode_get_samples_per_frame(vgmstream);
    if frame_size_d == 0 || samples_per_frame_d == 0 {
        println!("layout_interleave: wrong values found");
        *buffer =
            vec![0; (sample_count - samples_written) as usize * 2 * vgmstream.channels as usize];
        return;
    }
    samples_this_block_d =
        vgmstream.interleave_block_size / frame_size_d as isize * samples_per_frame_d as isize;
    if has_interleave_first {
        let frame_size_f = decode_get_frame_size(vgmstream);
        samples_per_frame_f = decode_get_samples_per_frame(vgmstream); //todo samples per shortframe
        if frame_size_f == 0 || samples_per_frame_f == 0 {
            println!("layout_interleave: wrong values found");
            *buffer =
                vec![
                    0;
                    (sample_count - samples_written) as usize * 2 * vgmstream.channels as usize
                ];
            return;
        }
        samples_this_block_f =
            vgmstream.interleave_first_block_size / (frame_size_f * samples_per_frame_f) as isize;
    }
    if has_interleave_last {
        // let frame_size_l = decode_get_shortframe_size(vgmstream);
        // samples_per_frame_l = decode_get_samples_per_shortframe(vgmstream);
        // if frame_size_l == 0 || samples_per_frame_l == 0 {
        //     println!("layout_interleave: wrong values found");
        //     return None;
        // }
        // samples_this_block_l = vgmstream.interleave_last_block_size / frame_size_l * samples_per_frame_l;
    }

    /* set current values */
    if has_interleave_first && vgmstream.current_sample < samples_this_block_f {
        samples_per_frame = samples_per_frame_f;
        samples_this_block = samples_this_block_f;
    } else if has_interleave_last
        && vgmstream.current_sample - vgmstream.samples_into_block + samples_this_block_d
            > vgmstream.num_samples as isize
    {
        samples_per_frame = samples_per_frame_l as i32;
        samples_this_block = samples_this_block_l;
    } else {
        samples_per_frame = samples_per_frame_d;
        samples_this_block = samples_this_block_d;
    }

    /* mono interleaved stream with no layout set, just behave like flat layout */
    if samples_this_block == 0 && vgmstream.channels == 1 {
        samples_this_block = vgmstream.num_samples as isize;
    }

    /* write samples */
    while samples_written < sample_count {
        // int samples_to_do;

        // TODO: loop

        // if (vgmstream->loop_flag && decode_do_loop(vgmstream)) {
        //     /* handle looping, restore standard interleave sizes */
        //     if (has_interleave_first &&
        //             vgmstream->current_sample < samples_this_block_f) {
        //         /* use first interleave*/
        //         samples_per_frame = samples_per_frame_f;
        //         samples_this_block = samples_this_block_f;
        //         if (samples_this_block == 0 && vgmstream->channels == 1)
        //             samples_this_block = vgmstream->num_samples;
        //     }
        //     else if (has_interleave_last) { /* assumes that won't loop back into a interleave_last */
        //         samples_per_frame = samples_per_frame_d;
        //         samples_this_block = samples_this_block_d;
        //         if (samples_this_block == 0 && vgmstream->channels == 1)
        //             samples_this_block = vgmstream->num_samples;
        //     }

        //     continue;
        // }

        let mut samples_to_do =
            vgmstream.decode_get_samples_to_do(samples_this_block as i32, samples_per_frame);
        if samples_to_do > sample_count - samples_written {
            samples_to_do = sample_count - samples_written;
        }

        if samples_to_do == 0 {
            /* happens when interleave is not set */
            println!("layout_interleave: wrong values found");
            // memset(buffer + samples_written*vgmstream->channels, 0, (sample_count - samples_written) * vgmstream->channels * sizeof(sample_t));
            *buffer =
                vec![
                    0;
                    (sample_count - samples_written) as usize * 2 * vgmstream.channels as usize
                ];
            return;
        }

        decode_vgmstream(vgmstream, samples_written, samples_to_do, buffer);

        samples_written += samples_to_do;
        vgmstream.current_sample += samples_to_do as isize;
        vgmstream.samples_into_block += samples_to_do as isize;

        /* move to next interleaved block when all samples are consumed */
        if vgmstream.samples_into_block == samples_this_block {
            let ch = 0;

            if has_interleave_first && vgmstream.current_sample == samples_this_block_f {
                /* restore standard frame size after going past first interleave */
                samples_per_frame = samples_per_frame_d;
                samples_this_block = samples_this_block_d;
                if samples_this_block == 0 && vgmstream.channels == 1 {
                    samples_this_block = vgmstream.num_samples as isize;
                }

                for ch in 0..vgmstream.channels {
                    let skip = vgmstream.interleave_first_skip
                        * (vgmstream.channels - 1 - ch) as isize
                        + vgmstream.interleave_first_block_size
                            * (vgmstream.channels - ch) as isize
                        + vgmstream.interleave_block_size * ch as isize;
                    vgmstream.ch[ch as usize].offset += skip;
                }
            } else if has_interleave_last
                && vgmstream.current_sample + samples_this_block > vgmstream.num_samples as isize
            {
                /* adjust values again if inside last interleave */
                samples_per_frame = samples_per_frame_l;
                samples_this_block = samples_this_block_l;
                if samples_this_block == 0 && vgmstream.channels == 1 {
                    samples_this_block = vgmstream.num_samples as isize;
                }

                for ch in 0..vgmstream.channels {
                    let skip = vgmstream.interleave_block_size * (vgmstream.channels - ch) as isize
                        + vgmstream.interleave_last_block_size * ch as isize;
                    vgmstream.ch[ch as usize].offset += skip;
                }
            } else {
                for ch in 0..vgmstream.channels {
                    let skip = vgmstream.interleave_block_size * vgmstream.channels as isize;
                    vgmstream.ch[ch as usize].offset += skip;
                }
            }

            vgmstream.samples_into_block = 0;
        }
    }
}
