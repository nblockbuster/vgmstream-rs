use crate::streamfile::Streamfile;

#[inline(always)]
pub fn clamp16(val: i32) -> i32 {
    if val > 32767 {
        return 32767;
    } else if val < -32768 {
        return -32768;
    } else {
        return val;
    }
}

pub fn swap_samples_le(buf: &mut Vec<i16>, count: i32) {
    /* Windows can't be BE... I think */
    for i in 0..count {
        /* 16b sample in memory: aabb where aa=MSB, bb=LSB */
        let b0: u8 = (buf[i as usize] & 0xff) as u8;
        let b1: u8 = (buf[i as usize] >> 8) as u8;
        // C: uint8_t *p = (uint8_t*)&(buf[i]);
        let mut p: Vec<u8> = buf[i as usize].to_le_bytes().to_vec();
        /* 16b sample in buffer: bbaa where bb=LSB, aa=MSB */
        p[0] = b0;
        p[1] = b1;
        buf[i as usize] = i16::from_le_bytes(p.try_into().unwrap());
        /* when endianness is LE, buffer has bbaa already so this function can be skipped */
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct ChunkType {
    pub ctype: u32,      /* chunk id/fourcc */
    pub size: u32,      /* chunk size */
    pub offset: u32,    /* chunk offset (after type/size) */
    pub current: i32,   /* start position, or next chunk after size (set to -1 to break) */
    pub max: u32,       /* max offset, or filesize if not set */

    pub le_type: bool,        /* read type as LE instead of more common BE */
    pub be_size: bool,        /* read type as BE instead of more common LE */
    pub full_size: bool,      /* chunk size includes type+size */
    pub alignment: bool,      /* chunks with odd size need to be aligned to even, per RIFF spec */
}

pub fn next_chunk(chunk: &mut ChunkType, sf: &mut Streamfile) -> bool {
    use crate::streamfile::{read_u32be, read_u32le};
    let read_u32type = if !chunk.le_type { read_u32be } else { read_u32le };
    let read_u32size = if chunk.be_size { read_u32be } else { read_u32le };
    

    // uint32_t (*read_u32type)(off_t,STREAMFILE*) = !chunk->le_type ? read_u32be : read_u32le;
    // uint32_t (*read_u32size)(off_t,STREAMFILE*) = chunk->be_size ? read_u32be : read_u32le;

    if chunk.max == 0 {
        chunk.max = sf.get_size(std::ptr::null_mut()) as u32;
    }

    if chunk.current >= chunk.max as i32 {
        return false;   
    }
    /* can be used to signal "stop" */
    if chunk.current < 0 {
        return false;
    }

    chunk.ctype = read_u32type(sf, chunk.current as usize + 0x00);
    chunk.size = read_u32size(sf, chunk.current as usize + 0x04);

    chunk.offset = chunk.current as u32 + 0x04 + 0x04;
    chunk.current += if chunk.full_size { chunk.size as i32 } else { 0x08 + chunk.size as i32 };
    //;VGM_LOG("CHUNK: %x, %x, %x\n", dc.offset, chunk->type, chunk->size);

    /* read past data */
    if chunk.ctype == 0xFFFFFFFF || chunk.size == 0xFFFFFFFF {
        return false;
    }

    /* empty chunk with 0 size is ok, seen in some formats (XVAG uses it as end marker, Wwise in JUNK) */
    if chunk.ctype == 0 /*|| chunk->size == 0*/ {
        return false;
    }

    /* more chunks remain */
    return true;
}

pub mod SpeakerT {
    pub const speaker_FL: u32  = 1 << 0;     /* front left */
    pub const speaker_FR: u32  = 1 << 1;     /* front right */
    pub const speaker_FC: u32  = 1 << 2;     /* front center */
    pub const speaker_LFE: u32 = 1 << 3;     /* low frequency effects */
    pub const speaker_BL: u32  = 1 << 4;     /* back left */
    pub const speaker_BR: u32  = 1 << 5;     /* back right */
    pub const speaker_FLC: u32 = 1 << 6;     /* front left center */
    pub const speaker_FRC: u32 = 1 << 7;     /* front right center */
    pub const speaker_BC: u32  = 1 << 8;     /* back center */
    pub const speaker_SL: u32  = 1 << 9;     /* side left */
    pub const speaker_SR: u32  = 1 << 10;    /* side right */
    pub const speaker_TC: u32  = 1 << 11;    /* top center*/
    pub const speaker_TFL: u32 = 1 << 12;    /* top front left */
    pub const speaker_TFC: u32 = 1 << 13;    /* top front center */
    pub const speaker_TFR: u32 = 1 << 14;    /* top front right */
    pub const speaker_TBL: u32 = 1 << 15;    /* top back left */
    pub const speaker_TBC: u32 = 1 << 16;    /* top back center */
    pub const speaker_TBR: u32 = 1 << 17;    /* top back left */

}

/* typical mappings that metas may use to set channel_layout (but plugin must actually use it)
 * (in order, so 3ch file could be mapped to FL FR FC or FL FR LFE but not LFE FL FR)
 * not too sure about names but no clear standards */
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum ChannelMapping {
    #[default]
    mapping_MONO             = SpeakerT::speaker_FC,
    mapping_STEREO           = SpeakerT::speaker_FL | SpeakerT::speaker_FR,
    mapping_2POINT1          = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_LFE,
    mapping_2POINT1_xiph     = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC, /* aka 3STEREO? */
    mapping_QUAD             = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_BL  | SpeakerT::speaker_BR,
    mapping_QUAD_surround    = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_BC,
    mapping_QUAD_side        = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_SL  | SpeakerT::speaker_SR,
    mapping_5POINT0          = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_LFE | SpeakerT::speaker_BL | SpeakerT::speaker_BR,
    mapping_5POINT0_xiph     = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_BL | SpeakerT::speaker_BR,
    mapping_5POINT0_surround = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_SL | SpeakerT::speaker_SR,
    mapping_5POINT1          = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_LFE | SpeakerT::speaker_BL | SpeakerT::speaker_BR,
    mapping_5POINT1_surround = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_LFE | SpeakerT::speaker_SL | SpeakerT::speaker_SR,
    mapping_7POINT0          = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_LFE | SpeakerT::speaker_BC | SpeakerT::speaker_FLC | SpeakerT::speaker_FRC,
    mapping_7POINT1          = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_LFE | SpeakerT::speaker_BL | SpeakerT::speaker_BR  | SpeakerT::speaker_FLC | SpeakerT::speaker_FRC,
    mapping_7POINT1_surround = SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_LFE | SpeakerT::speaker_BL | SpeakerT::speaker_BR  | SpeakerT::speaker_SL  | SpeakerT::speaker_SR,
}

// impl TryInto<u32> for ChannelMapping {
//     type Error = ();

//     fn try_into(self) -> Result<u32, Self::Error> {
//         Ok(self as u32)
//     }
// }

impl Into<u32> for ChannelMapping {
    fn into(self) -> u32 {
        self as u32
    }
}

impl From<u32> for ChannelMapping {
    fn from(v: u32) -> Self {
        match v {
            SpeakerT::speaker_FC => ChannelMapping::mapping_MONO,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR => ChannelMapping::mapping_STEREO,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_LFE => ChannelMapping::mapping_2POINT1,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC => ChannelMapping::mapping_2POINT1_xiph,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_BL  | SpeakerT::speaker_BR => ChannelMapping::mapping_QUAD,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_BC => ChannelMapping::mapping_QUAD_surround,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_SL  | SpeakerT::speaker_SR => ChannelMapping::mapping_QUAD_side,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_LFE | SpeakerT::speaker_BL | SpeakerT::speaker_BR => ChannelMapping::mapping_5POINT0,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_BL | SpeakerT::speaker_BR => ChannelMapping::mapping_5POINT0_xiph,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_SL | SpeakerT::speaker_SR => ChannelMapping::mapping_5POINT0_surround,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_LFE | SpeakerT::speaker_BL | SpeakerT::speaker_BR => ChannelMapping::mapping_5POINT1,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_LFE | SpeakerT::speaker_SL | SpeakerT::speaker_SR => ChannelMapping::mapping_5POINT1_surround,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_LFE | SpeakerT::speaker_BC | SpeakerT::speaker_FLC | SpeakerT::speaker_FRC => ChannelMapping::mapping_7POINT0,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC | SpeakerT::speaker_LFE | SpeakerT::speaker_BL | SpeakerT::speaker_BR  | SpeakerT::speaker_FLC | SpeakerT::speaker_FRC => ChannelMapping::mapping_7POINT1,
            SpeakerT::speaker_FL | SpeakerT::speaker_FR | SpeakerT::speaker_FC  | SpeakerT::speaker_LFE | SpeakerT::speaker_BL | SpeakerT::speaker_BR  | SpeakerT::speaker_SL  | SpeakerT::speaker_SR => ChannelMapping::mapping_7POINT1_surround,
            _ => ChannelMapping::mapping_STEREO,
        }
    }
}