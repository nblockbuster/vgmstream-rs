pub const VGMSTREAM_MAX_CHANNELS: i32 = 64;
pub const VGMSTREAM_MIN_SAMPLE_RATE: i32 = 300; /* 300 is Wwise min */
pub const VGMSTREAM_MAX_SAMPLE_RATE: i32 = 192000; /* found in some FSB5 */
pub const VGMSTREAM_MAX_SUBSONGS: i32 = 65535; /* +20000 isn't that uncommon */
pub const VGMSTREAM_MAX_NUM_SAMPLES: i32 = 1000000000; /* no ~5h vgm hopefully */
pub const MSADPCM_MAX_BLOCK_SIZE: isize = 0x800;
pub const STREAMFILE_DEFAULT_BUFFER_SIZE: usize = 0x8000;