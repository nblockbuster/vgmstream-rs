use crate::vgmstream::VGMStream;
use crate::decode::*;

pub fn render_vgmstream_flat(buffer: &mut Vec<i16>, sample_count: i32, vgmstream: &mut VGMStream) {
    let mut samples_written = 0;
    let mut samples_per_frame = decode_get_samples_per_frame(vgmstream);
    let mut samples_this_block = vgmstream.num_samples; /* do all samples if possible */

    while samples_written < sample_count {
        let mut samples_to_do = 0;

        // if vgmstream.loop_flag && decode_do_loop(vgmstream) {
        //     /* handle looping */
        //     continue;
        // }

        samples_to_do = vgmstream.decode_get_samples_to_do(samples_this_block, samples_per_frame);
        if samples_to_do > sample_count - samples_written {
            samples_to_do = sample_count - samples_written;
        }

        if samples_to_do == 0 { /* when decoding more than num_samples */
            println!("FLAT: samples_to_do 0");
            // memset(outbuf + samples_written * vgmstream->channels, 0, (sample_count - samples_written) * vgmstream->channels * sizeof(sample_t));
            *buffer = vec![0; (sample_count - samples_written) as usize * 2 * vgmstream.channels as usize];
        }

        decode_vgmstream(vgmstream, samples_written, samples_to_do, buffer);

        samples_written += samples_to_do;
        vgmstream.current_sample += samples_to_do as isize;
        vgmstream.samples_into_block += samples_to_do as isize;
    }
}