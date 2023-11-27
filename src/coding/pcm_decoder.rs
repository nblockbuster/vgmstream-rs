pub fn pcm_bytes_to_samples(bytes: isize, channels: i32, bits_per_sample: i32) -> i32 {
    if channels <= 0 || bits_per_sample <= 0 {
        return 0;
    }
    return (bytes * 8) as i32 / channels / bits_per_sample;
}