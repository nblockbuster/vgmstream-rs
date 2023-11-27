pub mod vgmstream_types;
pub mod vgmstream;
pub mod formats;
pub mod streamfile;
pub mod constants;
pub mod meta;
pub mod coding;
pub mod util;
pub mod decode;
pub mod layout;
pub mod render;
pub mod mixing;

#[cfg(test)]
mod tests {
    #[test]
    fn wwise_vorbis() {
        // let vorbis_data = include_bytes!("../test_data/wem/474329706.wem");
        // let wav_data = include_bytes!("../test_data/wem/474329706.wav");

        let vgmstream = crate::vgmstream::VGMStream::init("../test_data/wem/474329706.wem".to_string());
        // println!("{:?}", vgmstream);
        // assert_eq!(vgmstream.sample_rate, 48000);
        // assert_eq!(vgmstream.channels, 2);
    }

    // #[test]
    // fn cri_adx() {
    //     let adx_data = include_bytes!("../test_data/adx/5.1_multichannel.adx");
    //     let wav_data = include_bytes!("../test_data/adx/5.1_multichannel.wav");
    // }
}