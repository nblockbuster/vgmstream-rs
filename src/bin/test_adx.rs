use vgmstream_rs::{vgmstream::VGMStream, render::render_vgmstream};
const SAMPLE_BUFFER_SIZE: usize = 32768;
use std::time::Instant;
pub fn main() {
    let start = Instant::now();
    let vgmstream = VGMStream::init("test_data/adx/mono.adx".to_string());
    println!("{}hz, {} channels, {} samples total, {}s length", vgmstream.sample_rate, vgmstream.ch.len(), vgmstream.num_samples, vgmstream.num_samples / vgmstream.sample_rate);
    // assert_eq!(vgmstream.sample_rate, 48000);
    // assert_eq!(vgmstream.channels, 2);
    // println!("{:?}", vgmstream);
    write_file(&mut vgmstream.clone());
    let end = start.elapsed();
    println!("{}ms", end.as_millis());
}

fn write_file(vgmstream: &mut VGMStream) {
    let mut buffer: Vec<i16> = vec![0; SAMPLE_BUFFER_SIZE * 2 * vgmstream.channels as usize];
    // let mut a = 0;
    // a = render_vgmstream(&mut buffer, SAMPLE_BUFFER_SIZE as i32, vgmstream);
    // println!("{}", buffer.len());
    let mut wav_header: Vec<u8> = vec![0;0x100];
    let len_samples = vgmstream.get_samples();
    make_wav_header(&mut wav_header, len_samples, vgmstream.sample_rate, vgmstream.channels, false, 0, 0);
    let mut file = std::fs::File::create("test_data/adx/mono_aaaaa.bin.wav").unwrap();
    let mut writer = std::io::BufWriter::new(file);

    writer.write_all(&wav_header).unwrap();

    use std::io::Write;
    
    for i in (0..len_samples).step_by(SAMPLE_BUFFER_SIZE) {
        let mut to_get = SAMPLE_BUFFER_SIZE;
        if i + SAMPLE_BUFFER_SIZE as i32 > len_samples {
            to_get = (len_samples - i) as usize;
        }

        render_vgmstream(&mut buffer, to_get as i32, vgmstream);
        use vgmstream_rs::util::util::swap_samples_le;
        swap_samples_le(&mut buffer, vgmstream.channels * to_get as i32); /* write PC endian */
        // fwrite(buf, sizeof(sample_t) * channels, to_get, outfile);
        let mut buffer2: Vec<u8> = vec![0; to_get * vgmstream.channels as usize * 2];
        for i in 0..to_get * vgmstream.channels as usize {
            buffer2[i * 2] = (buffer[i] & 0xff) as u8;
            buffer2[i * 2 + 1] = (buffer[i] >> 8) as u8;
        }
        writer.write_all(&buffer2).unwrap();
    }



    // write raw buffer
    // for i in 0..buffer.len() {
    //     writer.write_all(&buffer[i].to_le_bytes()).unwrap();
    // }
    writer.flush().unwrap();
}

fn make_wav_header(buf: &mut Vec<u8>, sample_count: i32, sample_rate: i32, channels: i32, smpl_chunk: bool, loop_start: i32, loop_end: i32) -> usize {
    let mut data_size = sample_count * channels * 2;
    let mut header_size: i32 = 0x2c;
    if smpl_chunk && loop_end != 0 {
        header_size += 0x3c+ 0x08;
    }

    if header_size > buf.len() as i32 {
        return 0;
    }

    // memcpy(buf+0x00, "RIFF", 0x04); /* RIFF header */
    buf[0x00..0x04].copy_from_slice(&"RIFF".as_bytes());
    // put_32bitLE(buf+0x04, (int32_t)(header_size - 0x08 + data_size)); /* size of RIFF */
    buf[0x04..0x08].copy_from_slice(&(header_size - 0x08 + data_size as i32).to_le_bytes());

    // memcpy(buf+0x08, "WAVE", 4); /* WAVE header */
    buf[0x08..0x0c].copy_from_slice(&"WAVE".as_bytes());

    // memcpy(buf+0x0c, "fmt ", 0x04); /* WAVE fmt chunk */
    buf[0x0c..0x10].copy_from_slice(&"fmt ".as_bytes());

    // put_s32le(buf+0x10, 0x10); /* size of WAVE fmt chunk */
    buf[0x10..0x14].copy_from_slice(&i32::to_le_bytes(0x10));
    // put_s16le(buf+0x14, 0x0001); /* codec PCM */
    buf[0x14..0x16].copy_from_slice(&i16::to_le_bytes(0x0001));
    // put_s16le(buf+0x16, channels); /* channel count */
    buf[0x16..0x18].copy_from_slice(&(channels as i16).to_le_bytes());
    // put_s32le(buf+0x18, sample_rate); /* sample rate */
    buf[0x18..0x1c].copy_from_slice(&sample_rate.to_le_bytes());
    // put_s32le(buf+0x1c, sample_rate * channels * sizeof(sample_t)); /* bytes per second */
    buf[0x1c..0x20].copy_from_slice(&(sample_rate * channels * 2).to_le_bytes());
    // put_s16le(buf+0x20, (int16_t)(channels * sizeof(sample_t))); /* block align */
    buf[0x20..0x22].copy_from_slice(&(channels as i16 * 2).to_le_bytes());
    // put_s16le(buf+0x22, sizeof(sample_t) * 8); /* significant bits per sample */
    buf[0x22..0x24].copy_from_slice(&i16::to_le_bytes(2 * 8));

    // if (smpl_chunk && loop_end) {
    //     make_smpl_chunk(buf+0x24, loop_start, loop_end);
    //     memcpy(buf+0x24+0x3c+0x08, "data", 0x04); /* WAVE data chunk */
    //     put_u32le(buf+0x28+0x3c+0x08, (int32_t)data_size); /* size of WAVE data chunk */
    // }
    // else {
        // memcpy(buf+0x24, "data", 0x04); /* WAVE data chunk */
        buf[0x24..0x28].copy_from_slice(&"data".as_bytes());
        // put_s32le(buf+0x28, (int32_t)data_size); /* size of WAVE data chunk */
        buf[0x28..0x2c].copy_from_slice(&data_size.to_le_bytes());
    // }

    /* could try to add channel_layout, but would need to write WAVEFORMATEXTENSIBLE (maybe only if arg flag?) */

    return header_size as usize;
}