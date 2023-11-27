use crate::vgmstream::VGMStream;
use crate::decode::*;
use crate::render::render_vgmstream;
use crate::vgmstream_types::SegmentedLayoutData;

const VGMSTREAM_SEGMENT_SAMPLE_BUFFER: i32 = 8192;

pub fn render_vgmstream_segmented(sample_count: i32, vgmstream: &mut VGMStream) -> Option<Vec<i16>> {
    let mut samples_written = 0;
    let mut samples_this_block: i32 = 0;
    let data = vgmstream.segmented_layout_data.as_ref().unwrap();
    let use_internal_buffer;
    // let mut current_channels = 0;
    let mut current_segment = data.current_segment;

    /* normally uses outbuf directly (faster?) but could need internal buffer if downmixing */
    if vgmstream.channels != data.input_channels || data.mixed_channels {
        use_internal_buffer = true;
    }

    if current_segment >= data.segment_count {
        println!("SEGMENT: wrong current segment");
        return None;
    }

    samples_this_block = data.segments[data.current_segment as usize].get_samples();
    // current_channels = mixing_info(data.segments[data->current_segment], NULL);

    while samples_written < sample_count {
        let mut samples_to_do = 0;

        // if (vgmstream.loop_flag && decode_do_loop(vgmstream)) {
        //     /* handle looping (loop_layout has been called below, changes segments/state) */
        //     samples_this_block = vgmstream_get_samples(data->segments[data->current_segment]);
        //     mixing_info(data->segments[data->current_segment], NULL, &current_channels);
        //     continue;
        // }

        /* detect segment change and restart (after loop, but before decode, to allow looping to kick in) */
        if vgmstream.samples_into_block >= samples_this_block as isize {
            current_segment+=1;

            if data.current_segment >= data.segment_count { /* when decoding more than num_samples */
                println!("SEGMENTED: reached last segment");
                return None;
            }

            /* in case of looping spanning multiple segments */
            // data.segments[data.current_segment].reset();

            samples_this_block = data.segments[data.current_segment as usize].get_samples();
            // mixing_info(data->segments[data->current_segment], NULL, &current_channels);
            vgmstream.samples_into_block = 0;
            continue;
        }


        samples_to_do = vgmstream.decode_get_samples_to_do(samples_this_block, sample_count);
        if samples_to_do > sample_count - samples_written {
            samples_to_do = sample_count - samples_written;
        }
        if samples_to_do > VGMSTREAM_SEGMENT_SAMPLE_BUFFER /*&& use_internal_buffer*/ { /* always for fade/etc mixes */
            samples_to_do = VGMSTREAM_SEGMENT_SAMPLE_BUFFER;            
        }

        if samples_to_do < 0 { /* 0 is ok? */
            println!("SEGMENTED: wrong samples_to_do {} found", samples_to_do);
            return None;
        }
        // let mut buffer: Option<Vec<i16>> = if use_internal_buffer { Some(data.buffer.clone()) } else { Some(vec![samples_written as i16 * data.output_channels as i16;0]) };
        // render_vgmstream(samples_to_do, &mut data.segments[data.current_segment as usize]);

        // if use_internal_buffer {
        //     copy_samples(outbuf, data, current_channels, samples_to_do, samples_written);
        // }

        samples_written += samples_to_do;
        vgmstream.current_sample += samples_to_do as isize;
        vgmstream.samples_into_block += samples_to_do as isize;
    }

    None
// decode_fail:
//     memset(outbuf + samples_written * data->output_channels, 0, (sample_count - samples_written) * data->output_channels * sizeof(sample_t));
}