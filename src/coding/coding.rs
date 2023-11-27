use std::fmt::Display;

use crate::streamfile::Streamfile;

// TODO: also make vorbis wrapper AAAAAAAAA

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OggVorbisIO { //todo simplify
    pub streamfile: Option<Streamfile>,
    pub start: i64, /* file offset where the Ogg starts */
    pub offset: i64, /* virtual offset, from 0 to size */
    pub size: i64, /* virtual size of the Ogg */

    /* decryption setup */
    // void (*decryption_callback)(void* ptr, size_t size, size_t nmemb, void* datasource);
    // uint8_t scd_xor;
    // off_t scd_xor_length;
    // uint32_t xor_value;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum VorbisCustomType {
    #[default]
    VORBIS_FSB,         /* FMOD FSB: simplified/external setup packets, custom packet headers */
    VORBIS_WWISE,       /* Wwise WEM: many variations (custom setup, headers and data) */
    VORBIS_OGL,         /* Shin'en OGL: custom packet headers */
    VORBIS_SK,          /* Silicon Knights AUD: "OggS" replaced by "SK" */
    VORBIS_VID1,        /* Neversoft VID1: custom packet blocks/headers */
    VORBIS_AWC,         /* Rockstar AWC: custom packet blocks/headers */
}

impl Display for VorbisCustomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VorbisCustomType::VORBIS_FSB => write!(f, "VORBIS_FSB"),
            VorbisCustomType::VORBIS_WWISE => write!(f, "VORBIS_WWISE"),
            VorbisCustomType::VORBIS_OGL => write!(f, "VORBIS_OGL"),
            VorbisCustomType::VORBIS_SK => write!(f, "VORBIS_SK"),
            VorbisCustomType::VORBIS_VID1 => write!(f, "VORBIS_VID1"),
            VorbisCustomType::VORBIS_AWC => write!(f, "VORBIS_AWC"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum WwiseSetupType {
    #[default]
    WWV_HEADER_TRIAD,
    WWV_FULL_SETUP,
    WWV_INLINE_CODEBOOKS,
    WWV_EXTERNAL_CODEBOOKS,
    WWV_AOTUV603_CODEBOOKS,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum WwiseHeaderType {
    #[default]
    WWV_TYPE_8,
    WWV_TYPE_6,
    WWV_TYPE_2,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum WwisePacketType {
    #[default]
    WWV_STANDARD,
    WWV_MODIFIED,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct VorbisCustomConfig {
    pub channels: i32,
    pub sample_rate: i32,
    pub blocksize_0_exp: i32,
    pub blocksize_1_exp: i32,

    pub setup_id: u32,
    pub big_endian: bool,
    pub stream_end: u32,

    pub setup_type: WwiseSetupType,
    pub header_type: WwiseHeaderType,
    pub packet_type: WwisePacketType,

    pub header_offset: isize,
    pub data_start_offset: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct OpusConfig {
    pub channels: u8,
    pub skip: i32,
    pub sample_rate: i32,
    /* multichannel-only */
    pub coupled_count: i32,
    pub stream_count: i32,
    pub channel_mapping: [u8;255],
    /* frame table */
    pub table_offset: usize,
    pub table_count: i32,
    /* fixed frames */
    pub frame_size: u16,
}

impl Default for OpusConfig {
    fn default() -> Self {
        Self {
            channels: 0,
            skip: 0,
            sample_rate: 0,
            coupled_count: 0,
            stream_count: 0,
            channel_mapping: [0;255],
            table_offset: 0,
            table_count: 0,
            frame_size: 0,
        }
    }
}