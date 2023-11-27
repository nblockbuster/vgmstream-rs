use std::fmt::Debug;

use aotuv_lancer_vorbis_sys::*;
use ogg_next_sys::*;

use super::coding::{VorbisCustomConfig, VorbisCustomType};
use crate::{streamfile::Streamfile, vgmstream::VGMStreamCodecData};

pub const VORBIS_DEFAULT_BUFFER_SIZE: isize = 0x8000; /* should be at least the size of the setup header, ~0x2000 */

// impl Clone for aotuv_lancer_vorbis_sys::vorbis_dsp_state {
//     fn clone(&self) -> Self {
//         return self.clone();
//     }
// }

// #[derive(Clone)]
pub struct VorbisCustomCodecData {
    pub vi: vorbis_info,      /* stream settings */
    pub vc: vorbis_comment,   /* stream comments */
    pub vd: vorbis_dsp_state, /* decoder global state */
    pub vb: vorbis_block,     /* decoder local state */
    pub op: ogg_packet,       /* fake packet for internal use */

    pub buffer: Vec<u8>, /* internal raw data buffer */
    pub buffer_size: isize,

    pub samples_to_discard: usize, /* for looping purposes */
    pub samples_full: bool,        /* flag, samples available in vorbis buffers */

    pub vtype: VorbisCustomType,    /* Vorbis subtype */
    pub config: VorbisCustomConfig, /* config depending on the mode */

    /* Wwise Vorbis: saved data to reconstruct modified packets */
    pub mode_blockflag: [u8; 64 + 1], /* max 6b+1; flags 'n stuff */
    pub mode_bits: i32,               /* bits to store mode_number */
    pub prev_blockflag: u8,           /* blockflag in the last decoded packet */
    /* Ogg-style Vorbis: packet within a page */
    pub current_packet: i32,
    /* reference for page/blocks */
    pub block_offset: usize,
    pub block_size: usize,

    pub prev_block_samples: i32, /* count for optimization */
}

impl Default for VorbisCustomCodecData {
    fn default() -> Self {
        unsafe {
            VorbisCustomCodecData {
                vi: std::mem::zeroed(),
                vc: std::mem::zeroed(),
                vd: std::mem::zeroed(),
                vb: std::mem::zeroed(),
                op: std::mem::zeroed(),

                buffer: vec![0; VORBIS_DEFAULT_BUFFER_SIZE as usize],
                buffer_size: VORBIS_DEFAULT_BUFFER_SIZE,

                samples_to_discard: 0,
                samples_full: false,

                vtype: VorbisCustomType::VORBIS_FSB,
                config: VorbisCustomConfig::default(),

                mode_blockflag: [0; 64 + 1],
                mode_bits: 0,
                prev_blockflag: 0,

                current_packet: 0,
                block_offset: 0,
                block_size: 0,

                prev_block_samples: 0,
            }
        }
    }
}

/**
 * Inits a vorbis stream of some custom variety.
 *
 * Normally Vorbis packets are stored in .ogg, which is divided into OggS pages/packets, and the first packets contain necessary
 * Vorbis setup. For custom vorbis the OggS layer is replaced/optimized, the setup can be modified or stored elsewhere
 * (i.e.- in the .exe) and raw Vorbis packets may be modified as well, presumably to shave off some kb and/or obfuscate.
 * We'll manually read/modify the data and decode it with libvorbis calls.
 *
 * Reference: https://www.xiph.org/vorbis/doc/libvorbis/overview.html
 */
pub fn init_vorbis_custom(
    sf: &mut Streamfile,
    start_offset: usize,
    vtype: VorbisCustomType,
    config: &mut VorbisCustomConfig,
) -> Option<VGMStreamCodecData> {
    let mut data: VorbisCustomCodecData = Default::default();
    // int ok;

    /* init stuff */
    // data = calloc(1,sizeof(vorbis_custom_codec_data));
    // if (!data) goto fail;

    data.buffer_size = VORBIS_DEFAULT_BUFFER_SIZE;
    // data.buffer = calloc(sizeof(uint8_t), data.buffer_size);
    data.buffer = vec![0; data.buffer_size as usize];
    /* keep around to decode too */
    data.vtype = vtype;
    // memcpy(&data.config, config, sizeof(vorbis_custom_config));
    data.config = *config;

    /* init vorbis stream state, using 3 fake Ogg setup packets (info, comments, setup/codebooks)
     * libvorbis expects parsed Ogg pages, but we'll fake them with our raw data instead */
    unsafe {
        vorbis_info_init(&mut data.vi);
        vorbis_comment_init(&mut data.vc);
    }

    data.op.packet = data.buffer.as_mut_ptr();
    data.op.b_o_s = 1; /* fake headers start */

    let mut ok = false;

    /* init header */
    match data.vtype {
        // VorbisCustomType::VORBIS_FSB => {
        //     ok = vorbis_custom_setup_init_fsb(sf, start_offset, data);
        // }
        VorbisCustomType::VORBIS_WWISE => {
            ok = vorbis_custom_setup_init_wwise(sf, start_offset, &mut data);
        }
        // VorbisCustomType::VORBIS_OGL => {
        //     ok = vorbis_custom_setup_init_ogl(sf, start_offset, data);
        // }
        // VorbisCustomType::VORBIS_SK => {
        //     ok = vorbis_custom_setup_init_sk(sf, start_offset, data);
        // }
        // VorbisCustomType::VORBIS_VID1 => {
        //     ok = vorbis_custom_setup_init_vid1(sf, start_offset, data);
        // }
        // VorbisCustomType::VORBIS_AWC => {
        //     ok = vorbis_custom_setup_init_awc(sf, start_offset, data);
        // }
        _ => {
            // println!("VORBIS: init fail at around 0x{:x}", start_offset);
            println!("VORBIS: type {} is stubbed", data.vtype);
            return None;
        }
    }
    if !ok {
        println!("VORBIS: init fail at around 0x{:x}", start_offset);
        return None;
    }

    data.op.b_o_s = 0; /* end of fake headers */

    /* init vorbis global and block state */
    unsafe {
        if vorbis_synthesis_init(&mut data.vd, &mut data.vi) != 0 {
            println!("VORBIS: init fail at around 0x{:x}", start_offset);
            return None;
        }
        if vorbis_block_init(&mut data.vd, &mut data.vb) != 0 {
            println!("VORBIS: init fail at around 0x{:x}", start_offset);
            return None;
        }
    }

    /* write output */
    config.data_start_offset = data.config.data_start_offset;

    if data.config.stream_end == 0 {
        data.config.stream_end = sf.get_size(unsafe { std::mem::transmute(&data) }) as u32;
    }

    return Some(VGMStreamCodecData::CustomVorbis(data));

    // VGM_LOG("VORBIS: init fail at around 0x%x\n", (uint32_t)start_offset);
    // free_vorbis_custom(data);
    // return NULL;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct WPacketType {
    pub header_size: usize,
    pub packet_size: i32,
    pub granulepos: i32,

    pub has_next: i32,
    pub inxt: [u8; 0x01],
}

use super::coding::WwiseSetupType;

pub fn vorbis_custom_setup_init_wwise(
    sf: &mut Streamfile,
    start_offset: usize,
    data: &mut VorbisCustomCodecData,
) -> bool {
    let mut wp: WPacketType = Default::default();
    let mut ok = false;

    if data.config.setup_type == WwiseSetupType::WWV_HEADER_TRIAD {
        /* read 3 Wwise packets with triad (id/comment/setup), each with a Wwise header */
        let mut offset = start_offset;

        let mut ibuf = &mut data.buffer.clone();

        /* normal identificacion packet */
        ok = read_packet(&mut wp, ibuf, data.buffer_size, sf, offset, data, true);
        if !ok {
            return false;
        }
        data.op.bytes = wp.packet_size;
        unsafe {
            if vorbis_synthesis_headerin(&mut data.vi, &mut data.vc, &mut data.op) != 0 {
                return false;
            }
        }
        offset += wp.header_size + wp.packet_size as usize;

        /* normal comment packet */
        ok = read_packet(&mut wp, ibuf, data.buffer_size, sf, offset, data, true);
        if !ok {
            return false;
        }
        data.op.bytes = wp.packet_size;
        unsafe {
            if vorbis_synthesis_headerin(&mut data.vi, &mut data.vc, &mut data.op) != 0 {
                return false;
            }
        }
        offset += wp.header_size + wp.packet_size as usize;

        /* normal setup packet */
        ok = read_packet(&mut wp, ibuf, data.buffer_size, sf, offset, data, true);
        if !ok {
            return false;
        }
        data.op.bytes = wp.packet_size;
        unsafe {
            if vorbis_synthesis_headerin(&mut data.vi, &mut data.vc, &mut data.op) != 0 {
                return false;
            }
        }
        offset += wp.header_size + wp.packet_size as usize;
    }

    return false;
}

fn read_packet(
    wp: &mut WPacketType,
    ibuf: &mut Vec<u8>,
    ibufsize: isize,
    sf: &mut Streamfile,
    offset: usize,
    data: &mut VorbisCustomCodecData,
    is_setup: bool,
) -> bool {
    // uint32_t (*get_u32)(const uint8_t*) = data.config.big_endian ? get_u32be : get_u32le;
    // uint16_t (*get_u16)(const uint8_t*) = data.config.big_endian ? get_u16be : get_u16le;
    // int32_t  (*get_s32)(const uint8_t*) = data.config.big_endian ? get_s32be : get_s32le;

    use super::coding::{WwiseHeaderType, WwisePacketType};
    use crate::util::reader::{get_s32be, get_s32le, get_u16be, get_u16le, get_u32be, get_u32le};

    let get_u32 = if data.config.big_endian {
        get_u32be
    } else {
        get_u32le
    };
    let get_u16 = if data.config.big_endian {
        get_u16be
    } else {
        get_u16le
    };
    let get_s32 = if data.config.big_endian {
        get_s32be
    } else {
        get_s32le
    };

    /* read header info (packet size doesn't include header size) */
    match data.config.header_type {
        WwiseHeaderType::WWV_TYPE_8 => {
            wp.header_size = 0x08;
            // read_streamfile(ibuf, offset, wp.header_size, sf);
            *ibuf = sf.read(offset, wp.header_size, data as *mut _ as *mut _);
            wp.packet_size = get_u32(&ibuf[0x00..]) as i32;
            wp.granulepos = get_s32(&ibuf[0x04..]);
        }
        WwiseHeaderType::WWV_TYPE_6 => {
            wp.header_size = 0x06;
            // read_streamfile(ibuf, offset, wp.header_size, sf);
            *ibuf = sf.read(offset, wp.header_size, data as *mut _ as *mut _);
            wp.packet_size = get_u16(&ibuf[0x00..]) as i32;
            wp.granulepos = get_s32(&ibuf[0x02..]);
        }
        WwiseHeaderType::WWV_TYPE_2 => {
            wp.header_size = 0x02;
            // read_streamfile(ibuf, offset, wp.header_size, sf);
            *ibuf = sf.read(offset, wp.header_size, data as *mut _ as *mut _);
            wp.packet_size = get_u16(&ibuf[0x00..]) as i32;
            wp.granulepos = 0; /* granule is an arbitrary unit so we could use offset instead; libvorbis has no need for it */
        }
        _ => {
            /* ? */
            wp.header_size = 0;
            wp.packet_size = 0;
            wp.granulepos = 0;
        }
    }

    if wp.header_size == 0 || wp.packet_size == 0 {
        return false;
    }

    /* read packet data */
    {
        let mut read_size = wp.packet_size;

        /* mod packets need next packet's first byte (6 bits) except at EOF, so read now too */
        if !is_setup && data.config.packet_type == WwisePacketType::WWV_MODIFIED {
            read_size += wp.header_size as i32 + 0x01;
        }

        if wp.header_size == 0 || read_size > ibufsize as i32 {
            return false;
        }

        *ibuf = sf.read(offset + wp.header_size, read_size as usize, data as *mut _ as *mut _);
        if ibuf.len() < wp.packet_size as usize {
            println!("Wwise Vorbis: truncated packet");
            return false;
        }

        if !is_setup
            && data.config.packet_type == WwisePacketType::WWV_MODIFIED
            && ibuf.len() == read_size as usize
        {
            wp.has_next = 1;
            wp.inxt[0] = ibuf[(wp.packet_size + wp.header_size as i32) as usize];
        } else {
            wp.has_next = 0;
        }
    }

    return true;
}
