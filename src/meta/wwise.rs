use std::fmt::Display;

use crate::coding::coding::*;
use crate::coding::ffmpeg_opus::init_ffmpeg_wwise_opus;
use crate::coding::pcm_decoder::pcm_bytes_to_samples;
use crate::streamfile::*;
use crate::util::util::{next_chunk, ChunkType};
use crate::vgmstream::{check_extensions, CodingType, LayoutType, Streamfile, VGMStream};
use crate::vgmstream::{MetaType, VGMStreamCodecData};

/* Wwise uses a custom RIFF/RIFX header, non-standard enough that it's parsed it here.
 * There is some repetition from other metas, but not enough to bother me.
 *
 * Some info: https://www.audiokinetic.com/en/library/edge/
 * .bnk (dynamic music/loop) info: https://github.com/bnnm/wwiser
 */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum WwiseCodec {
    #[default]
    PCM,
    IMA,
    VORBIS,
    DSP,
    XMA2,
    XWMA,
    AAC,
    HEVAG,
    ATRAC9,
    OPUSNX,
    OPUS,
    OPUSCPR,
    OPUSWW,
    PTADPCM,
}

impl Display for WwiseCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WwiseCodec::PCM => write!(f, "PCM"),
            WwiseCodec::IMA => write!(f, "IMA"),
            WwiseCodec::VORBIS => write!(f, "VORBIS"),
            WwiseCodec::DSP => write!(f, "DSP"),
            WwiseCodec::XMA2 => write!(f, "XMA2"),
            WwiseCodec::XWMA => write!(f, "XWMA"),
            WwiseCodec::AAC => write!(f, "AAC"),
            WwiseCodec::HEVAG => write!(f, "HEVAG"),
            WwiseCodec::ATRAC9 => write!(f, "ATRAC9"),
            WwiseCodec::OPUSNX => write!(f, "OPUSNX"),
            WwiseCodec::OPUS => write!(f, "OPUS"),
            WwiseCodec::OPUSCPR => write!(f, "OPUSCPR"),
            WwiseCodec::OPUSWW => write!(f, "OPUSWW"),
            WwiseCodec::PTADPCM => write!(f, "PTADPCM"),
        }
    }
}

use crate::util::util::ChannelMapping;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct WwiseHeader {
    pub big_endian: bool,
    pub file_size: isize,
    pub prefetch: bool,
    pub is_wem: bool,
    pub is_bnk: bool,

    /* chunks references */
    pub fmt_offset: usize,
    pub fmt_size: isize,
    pub data_offset: usize,
    pub data_size: isize,
    pub xma2_offset: usize,
    pub xma2_size: isize,
    pub vorb_offset: usize,
    pub vorb_size: isize,
    pub wiih_offset: usize,
    pub wiih_size: isize,
    pub smpl_offset: usize,
    pub smpl_size: isize,
    pub seek_offset: usize,
    pub seek_size: isize,
    pub meta_offset: usize,
    pub meta_size: isize,

    /* standard fmt stuff */
    pub codec: WwiseCodec,
    pub format: i32,
    pub channels: i32,
    pub sample_rate: i32,
    pub block_size: i32,
    pub avg_bitrate: i32,
    pub bits_per_sample: i32,
    pub channel_type: u8,
    pub channel_layout: ChannelMapping,
    pub extra_size: isize,

    pub num_samples: i32,
    pub loop_flag: bool,
    pub loop_start_sample: i32,
    pub loop_end_sample: i32,
}

pub fn init_vgmstream_wwise(sf: &mut Streamfile) -> Option<VGMStream> {
    return init_vgmstream_wwise_bnk(sf, false);
}

fn init_vgmstream_wwise_bnk(sf: &mut Streamfile, p_prefetch: bool) -> Option<VGMStream> {
    let mut vgmstream: VGMStream = Default::default();
    let mut ww: WwiseHeader = Default::default();
    /* checks */
    if !is_id32be(sf, 0x00, "RIFF") /* LE */
    && !is_id32be(sf, 0x00, "RIFX")
    {
        /* BE */
        return None;
    }

    /* note that Wwise allows those extensions only, so custom engine exts shouldn't be added
     * .wem: newer "Wwise Encoded Media" used after the 2011.2 SDK (~july 2011)
     * .wav: older PCM/ADPCM files [Spider-Man: Web of Shadows (PC), Punch Out!! (Wii)]
     * .xma: older XMA files [Too Human (X360), Tron Evolution (X360)]
     * .ogg: older Vorbis files [The King of Fighters XII (X360)]
     * .bnk: Wwise banks for memory .wem detection (hack) */
    if !check_extensions(sf, vec!["wem", "wav", "lwav", "ogg", "logg", "xma", "bnk"]) {
        return None;
    }

    ww.is_bnk = p_prefetch;
    if !parse_wwise(sf, &mut ww) {
        return None;
    }

    let read_u32 = if ww.big_endian {
        read_u32be
    } else {
        read_u32le
    };
    let read_s32 = if ww.big_endian {
        read_s32be
    } else {
        read_s32le
    };
    let read_u16 = if ww.big_endian {
        read_u16be
    } else {
        read_u16le
    };

    let mut start_offset = ww.data_offset;

    vgmstream.meta_type = MetaType::meta_WWISE_RIFF;
    vgmstream.sample_rate = ww.sample_rate;
    vgmstream.loop_start_sample = ww.loop_start_sample;
    vgmstream.loop_end_sample = ww.loop_end_sample;
    vgmstream.channel_layout = ww.channel_layout.try_into().unwrap();
    vgmstream.stream_size = ww.data_size;

    match ww.codec {
        WwiseCodec::PCM => {
            if ww.fmt_size != 0x10
                && ww.fmt_size != 0x12
                && ww.fmt_size != 0x18
                && ww.fmt_size != 0x28
            {
                return None;
            }
            if ww.bits_per_sample != 16 {
                return None;
            }

            vgmstream.coding_type = if ww.big_endian {
                CodingType::coding_PCM16BE
            } else {
                CodingType::coding_PCM16LE
            };
            vgmstream.layout_type = if ww.channels > 1 {
                LayoutType::layout_interleave
            } else {
                LayoutType::layout_none
            };
            vgmstream.interleave_block_size = 0x02;

            if ww.prefetch {
                ww.data_size = ww.file_size - ww.data_offset as isize;
            }

            vgmstream.num_samples =
                pcm_bytes_to_samples(ww.data_size, ww.channels, ww.bits_per_sample);

            /* prefetch .bnk RIFFs that only have header and no data is possible [Metal Gear Solid V (PC)] */
            if ww.prefetch && vgmstream.num_samples == 0 {
                vgmstream.num_samples = 1; /* force something to avoid broken subsongs */
            }
        }
        WwiseCodec::VORBIS => {
            use crate::coding::vorbis::init_vorbis_custom;
            let mut data_offsets: usize = 0;
            let mut block_offsets: usize = 0;

            let mut setup_offset: usize = 0;
            let mut audio_offset: usize = 0;

            let mut cfg: VorbisCustomConfig = Default::default();

            cfg.channels = ww.channels;
            cfg.sample_rate = ww.sample_rate;
            cfg.big_endian = ww.big_endian;
            cfg.stream_end = (ww.data_offset + ww.data_size as usize) as u32;

            if ww.block_size != 0 || ww.bits_per_sample != 0 {
                return None;
            }

            /* autodetect format (fields are mostly common, see the end of the file) */
            if ww.vorb_offset != 0 {
                /* older Wwise (~<2012) */
                match ww.vorb_size {
                    0x2C |      /* earliest (~2009) [The Lord of the Rings: Conquest (PC)] */
                    0x28 => {   /* early (~2009) [UFC Undisputed 2009 (PS3), some EVE Online Apocrypha (PC)] */
                        data_offsets = 0x18;
                        block_offsets = 0;
                        cfg.header_type = WwiseHeaderType::WWV_TYPE_8;
                        cfg.packet_type = WwisePacketType::WWV_STANDARD;
                        cfg.setup_type = WwiseSetupType::WWV_HEADER_TRIAD
                    },
                    0x34 |      /* common (2010~2011) [The King of Fighters XII (PS3), Assassin's Creed II (X360)] */
                    0x32 => {   /* rare (mid 2011) [Saints Row the 3rd (PC)] */
                        data_offsets = 0x18;
                        block_offsets = 0x30;
                        cfg.header_type = WwiseHeaderType::WWV_TYPE_6;
                        cfg.packet_type = WwisePacketType::WWV_STANDARD;
                        cfg.setup_type = WwiseSetupType::WWV_EXTERNAL_CODEBOOKS;
                    },
                    0x2A => {   /* uncommon (mid 2011) [inFamous 2 (PS3), Captain America: Super Soldier (X360)] */
                        data_offsets = 0x10;
                        block_offsets = 0x28;
                        cfg.header_type = WwiseHeaderType::WWV_TYPE_2;
                        cfg.packet_type = WwisePacketType::WWV_MODIFIED;
                        cfg.setup_type = WwiseSetupType::WWV_EXTERNAL_CODEBOOKS;
                    },
                    _ => {
                        println!("WWISE: unknown vorb size 0x{:x}", ww.vorb_size);
                        return None;
                    }
                }

                vgmstream.num_samples = read_s32(sf, ww.vorb_offset as usize + 0x00);
                setup_offset = read_u32(sf, ww.vorb_offset + data_offsets + 0x00) as usize; /* within data (0 = no seek table) */
                audio_offset = read_u32(sf, ww.vorb_offset + data_offsets + 0x04) as usize; /* within data */
                if block_offsets != 0 {
                    cfg.blocksize_1_exp = read_u8(sf, ww.vorb_offset + block_offsets + 0x00) as i32; /* small */
                    cfg.blocksize_0_exp = read_u8(sf, ww.vorb_offset + block_offsets + 0x01) as i32;
                    /* big */
                }
                ww.data_size -= audio_offset as isize;

                /* detect normal packets */
                if ww.vorb_size == 0x2a {
                    /* almost all blocksizes are 0x08+0x0B except a few with 0x0a+0x0a [Captain America: Super Soldier (X360) voices/sfx] */
                    if cfg.blocksize_0_exp == cfg.blocksize_1_exp {
                        cfg.packet_type = WwisePacketType::WWV_STANDARD;
                    }
                }

                /* detect setup type:
                 * - full inline: ~2009, ex. The King of Fighters XII (X360), The Saboteur (PC)
                 * - trimmed inline: ~2010, ex. Army of Two: 40 days (X360) some multiplayer files
                 * - external: ~2010, ex. Assassin's Creed Brotherhood (X360), Dead Nation (X360) */
                if ww.vorb_size == 0x34 {
                    let setup_size = read_u16(sf, start_offset + setup_offset + 0x00);
                    let setup_id = read_u32be(sf, start_offset + setup_offset + 0x06);

                    /* if the setup after header starts with "(data)BCV" it's an inline codebook) */
                    if (setup_id & 0x00FFFFFF) == get_id32be("\0BCV") {
                        cfg.setup_type = WwiseSetupType::WWV_FULL_SETUP;
                    }
                    /* if the setup is suspiciously big it's probably trimmed inline codebooks */
                    else if setup_size > 0x200 {
                        /* an external setup it's ~0x100 max + some threshold */
                        cfg.setup_type = WwiseSetupType::WWV_INLINE_CODEBOOKS;
                    }
                }
                // println!("WWISE: vorbis initialization is stubbed.");

                vgmstream.codec_data = init_vorbis_custom(
                    sf,
                    start_offset + setup_offset,
                    VorbisCustomType::VORBIS_WWISE,
                    &mut cfg,
                );

                if vgmstream.codec_data.is_none()
                    || !matches!(
                        vgmstream.codec_data.as_ref().unwrap(),
                        VGMStreamCodecData::CustomVorbis(_)
                    )
                {
                    return None;
                }
            } else {
                /* newer Wwise (>2012) */
                let extra_offset = ww.fmt_offset + 0x18; /* after flag + channels */

                match ww.extra_size {
                    0x30 => {
                        data_offsets = 0x10;
                        block_offsets = 0x28;
                        cfg.header_type = WwiseHeaderType::WWV_TYPE_2;
                        cfg.packet_type = WwisePacketType::WWV_MODIFIED;

                        /* setup not detectable by header, so we'll try both; libvorbis should reject wrong codebooks
                         * - standard: early (<2012), ex. The King of Fighters XIII (X360)-2011/11, .ogg (cbs are from aoTuV, too)
                         * - aoTuV603: later (>2012), ex. Sonic & All-Stars Racing Transformed (PC)-2012/11, .wem */
                        cfg.setup_type = if ww.is_wem {
                            WwiseSetupType::WWV_AOTUV603_CODEBOOKS
                        } else {
                            WwiseSetupType::WWV_EXTERNAL_CODEBOOKS
                        }; /* aoTuV came along .wem */
                    }
                    _ => {
                        println!("WWISE: unknown extra size 0x{:x}", ww.vorb_size);
                        return None;
                    }
                }

                vgmstream.num_samples = read_s32(sf, extra_offset + 0x00);
                setup_offset = read_u32(sf, extra_offset + data_offsets + 0x00) as usize; /* within data */
                audio_offset = read_u32(sf, extra_offset + data_offsets + 0x04) as usize; /* within data */
                cfg.blocksize_1_exp = read_u8(sf, extra_offset + block_offsets + 0x00) as i32; /* small */
                cfg.blocksize_0_exp = read_u8(sf, extra_offset + block_offsets + 0x01) as i32; /* big */
                ww.data_size -= audio_offset as isize;

                /* mutant .wem with metadata (voice strings/etc) between seek table and vorbis setup [Gears of War 4 (PC)] */
                if ww.meta_offset != 0 {
                    /* 0x00: original setup_offset */
                    setup_offset += read_u32(sf, ww.meta_offset + 0x04) as usize;
                    /* metadata size */
                }

                /* detect normal packets */
                if ww.extra_size == 0x30 {
                    /* almost all blocksizes are 0x08+0x0B except some with 0x09+0x09 [Oddworld New 'n' Tasty! (PSV)] */
                    if cfg.blocksize_0_exp == cfg.blocksize_1_exp {
                        cfg.packet_type = WwisePacketType::WWV_STANDARD;
                    }
                }

                /* try with the selected codebooks */
                vgmstream.codec_data = init_vorbis_custom(
                    sf,
                    start_offset + setup_offset,
                    VorbisCustomType::VORBIS_WWISE,
                    &mut cfg,
                );

                if vgmstream.codec_data.is_none() {
                    return None;
                }
                if !matches!(
                    vgmstream.codec_data.as_ref().unwrap(),
                    VGMStreamCodecData::CustomVorbis(_)
                ) {
                    /* codebooks failed: try again with the other type */
                    cfg.setup_type = if ww.is_wem {
                        WwiseSetupType::WWV_EXTERNAL_CODEBOOKS
                    } else {
                        WwiseSetupType::WWV_AOTUV603_CODEBOOKS
                    };
                    vgmstream.codec_data = init_vorbis_custom(
                        sf,
                        start_offset + setup_offset,
                        VorbisCustomType::VORBIS_WWISE,
                        &mut cfg,
                    );

                    if vgmstream.codec_data.is_none()
                        || !matches!(
                            vgmstream.codec_data.as_ref().unwrap(),
                            VGMStreamCodecData::CustomVorbis(_)
                        )
                    {
                        return None;
                    }
                }
            }
            vgmstream.layout_type = LayoutType::layout_none;
            vgmstream.coding_type = CodingType::coding_VORBIS_custom;
            vgmstream.codec_endian = ww.big_endian;

            start_offset += audio_offset;

            /* Vorbis is VBR so this is very approximate percent, meh */
            if ww.prefetch {
                vgmstream.num_samples = (vgmstream.num_samples as f64
                    * (ww.file_size - start_offset as isize) as f64
                    / ww.data_size as f64) as i32;
            }
        }
        WwiseCodec::OPUSWW => {
            let mut cfg: OpusConfig = Default::default();

            if ww.block_size != 0 || ww.bits_per_sample != 0 {
                return None;
            }
            if ww.seek_offset == 0 {
                return None;
            }
            if ww.channels > 255 {
                return None;
            } /* opus limit */

            cfg.channels = ww.channels as u8;
            cfg.table_offset = ww.seek_offset;

            vgmstream.sample_rate = 48000; /* fixed in AK's code */

            /* extra: size 0x10 (though last 2 fields are beyond, AK plz) */
            /* 0x12: samples per frame */
            vgmstream.num_samples = read_s32(sf, ww.fmt_offset + 0x18);
            cfg.table_count = read_u32(sf, ww.fmt_offset + 0x1c) as i32; /* same as seek size / 2 */
            cfg.skip = read_u16(sf, ww.fmt_offset + 0x20) as i32;
            /* 0x22: codec version */
            let mut mapping = read_u8(sf, ww.fmt_offset + 0x23);
            if mapping == 1 && ww.channels > 8 {
                return None;
            } /* mapping not defined */

            if read_u8(sf, ww.fmt_offset + 0x22) != 1 {
                return None;
            }

            /* OPUS is VBR so this is very approximate percent, meh */
            if ww.prefetch {
                vgmstream.num_samples = (vgmstream.num_samples as f64
                    * (ww.file_size - start_offset as isize) as f64
                    / ww.data_size as f64) as i32;
                ww.data_size = ww.file_size - start_offset as isize;
            }

            /* AK does some wonky implicit config for multichannel (only accepted channel type is 1) */
            if ww.channel_type == 1 && mapping == 1 {
                const MAPPING_MATRIX: [[u8; 8]; 8] = [
                    /* (DeinterleaveAndRemap)*/
                    [0, 0, 0, 0, 0, 0, 0, 0],
                    [0, 1, 0, 0, 0, 0, 0, 0],
                    [0, 2, 1, 0, 0, 0, 0, 0],
                    [0, 1, 2, 3, 0, 0, 0, 0],
                    [0, 4, 1, 2, 3, 0, 0, 0],
                    [0, 4, 1, 2, 3, 5, 0, 0],
                    [0, 6, 1, 2, 3, 4, 5, 0],
                    [0, 6, 1, 2, 3, 4, 5, 7],
                ];

                if ww.channels > 8 {
                    return None;
                } /* matrix limit */

                /* find coupled (stereo) OPUS streams (simplification of ChannelConfigToMapping) */
                match ww.channel_layout {
                    ChannelMapping::mapping_7POINT1_surround => {  cfg.coupled_count = 3; }   /* 2ch+2ch+2ch+1ch+1ch, 5 streams */
                    ChannelMapping::mapping_5POINT1_surround |                                  /* 2ch+2ch+1ch+1ch, 4 streams */
                    ChannelMapping::mapping_QUAD_side => {         cfg.coupled_count = 2; }   /* 2ch+2ch, 2 streams */
                    ChannelMapping::mapping_2POINT1_xiph |                                      /* 2ch+1ch, 2 streams */
                    ChannelMapping::mapping_STEREO => {            cfg.coupled_count = 1; }   /* 2ch, 1 stream */
                    _ => {                        cfg.coupled_count = 0; }   /* 1ch, 1 stream */
                    //TODO: AK OPUS doesn't seem to handle others mappings, though AK's .h imply they exist (uses 0 coupleds?)
                }

                /* total number internal OPUS streams (should be >0) */
                cfg.stream_count = ww.channels - cfg.coupled_count;

                /* channel order */
                for i in 0..ww.channels {
                    cfg.channel_mapping[i as usize] =
                        MAPPING_MATRIX[ww.channels as usize - 1][i as usize];
                }
            } else if ww.channel_type == 1 && mapping == 255 {
                /* Overwatch 2 (PC) */

                /* only seen 12ch, but seems to be what ChannelConfigToMapping would output with > 8 */
                cfg.coupled_count = 0;

                cfg.stream_count = ww.channels - cfg.coupled_count;

                //TODO: mapping seems to be 0x2d63f / FL FR FC LFE BL BR SL SR TFL TFR TBL TBR
                // while output order seems to swap FC and LFE? (not set in passed channel mapping but reordered later)
                for i in 0..ww.channels {
                    cfg.channel_mapping[i as usize] = i as u8;
                }
            } else {
                /* mapping 0: standard opus (implicit mono/stereo)  */
                if ww.channels > 2 {
                    return None;
                }
            }

            /* Wwise Opus saves all frame sizes in the seek table */
            vgmstream.codec_data =
                init_ffmpeg_wwise_opus(sf, ww.data_offset, ww.data_size as usize, &mut cfg);
            // vgmstream.codec_data = codec;
            if vgmstream.codec_data.is_none() {
                return None;
            }
            vgmstream.coding_type = CodingType::coding_FFmpeg;
            vgmstream.layout_type = LayoutType::layout_none;
        }
        _ => {
            println!("{} not implemented!", ww.codec);
            return None;
        }
    }

    vgmstream.open_stream(sf, start_offset as isize);
    return Some(vgmstream);
}

fn parse_wwise(sf: &mut Streamfile, ww: &mut WwiseHeader) -> bool {
    ww.big_endian = is_id32be(sf, 0x00, "RIFX"); /* RIFF size not useful to detect, see below */

    let read_u32 = if ww.big_endian {
        read_u32be
    } else {
        read_u32le
    };
    let read_u16 = if ww.big_endian {
        read_u16be
    } else {
        read_u16le
    };

    ww.file_size = sf.get_size(std::ptr::null_mut()) as isize;

    if !is_id32be(sf, 0x08, "WAVE") && !is_id32be(sf, 0x08, "XWMA") {
        return false;
    }

    let mut rc: ChunkType = Default::default();
    let file_size = sf.get_size(std::ptr::null_mut()) as u32;

    /* chunks are even-aligned and don't need to add padding byte, unlike real RIFFs */
    rc.be_size = ww.big_endian;
    rc.current = 0x0c;
    while next_chunk(&mut rc, sf) {
        match rc.ctype {
            0x666d7420 => {
                /* "fmt " */
                ww.fmt_offset = rc.offset as usize;
                ww.fmt_size = rc.size as isize;
            }
            0x584D4132 => {
                /* "XMA2" */
                ww.xma2_offset = rc.offset as usize;
                ww.xma2_size = rc.size as isize;
            }
            0x64617461 => {
                /* "data" */
                ww.data_offset = rc.offset as usize;
                ww.data_size = rc.size as isize;
            }
            0x766F7262 => {
                /* "vorb" */
                ww.vorb_offset = rc.offset as usize;
                ww.vorb_size = rc.size as isize;
            }
            0x57696948 => {
                /* "WiiH" */
                ww.wiih_offset = rc.offset as usize;
                ww.wiih_size = rc.size as isize;
            }
            0x7365656B => {
                /* "seek" */
                ww.seek_offset = rc.offset as usize;
                ww.seek_size = rc.size as isize;
            }
            0x736D706C => {
                /* "smpl" */
                ww.smpl_offset = rc.offset as usize;
                ww.smpl_size = rc.size as isize;
            }
            0x6D657461 => {
                /* "meta" */
                ww.meta_offset = rc.offset as usize;
                ww.meta_size = rc.size as isize;
            }
            0x66616374 => {
                /* "fact" */
                /* Wwise never uses fact, but if somehow some file does uncomment the following: */
                //if (size == 0x10 && read_u32be(offset + 0x04, sf) == 0x4C794E20) /* "LyN " */
                //    goto fail; /* ignore LyN RIFF */
                return false;
            }

            /* "XMAc": rare XMA2 physical loop regions (loop_start_b, loop_end_b, loop_subframe_data)
             *         Can appear even in the file doesn't loop, maybe it's meant to be the playable physical region */
            /* "LIST": leftover 'cue' info from OG .wavs (ex. loop starts in Platinum Games) */
            /* "JUNK": optional padding for aligment (0-size JUNK exists too) */
            /* "akd ": extra info for Wwise? (wave peaks/loudness/HDR envelope?) */
            _ => {
                /* mainly for incorrectly ripped wems, but should allow truncated wems
                 * (could also check that fourcc is ASCII)  */
                if rc.offset + rc.size > file_size {
                    println!("WWISE: broken .wem (bad extract?)");
                    return false;
                }
            }
        }
    }

    /* use extension as a guide for certain cases */
    ww.is_wem = check_extensions(sf, vec!["wem", "bnk"]);

    /* parse format (roughly spec-compliant but some massaging is needed) */
    if ww.xma2_offset != 0 {
        /* pseudo-XMA2WAVEFORMAT, "fmt"+"XMA2" (common) or only "XMA2" [Too Human (X360)] */
        ww.format = 0x0165; /* signal for below */
    // xma2_parse_xma2_chunk(sf, ww.xma2_offset, &ww.channels, &ww.sample_rate, &ww.loop_flag, &ww.num_samples, &ww.loop_start_sample, &ww.loop_end_sample);
    } else {
        /* pseudo-WAVEFORMATEX */
        if ww.fmt_size < 0x10 {
            return false;
        }
        ww.format = read_u16(sf, ww.fmt_offset + 0x00) as i32;
        ww.channels = read_u16(sf, ww.fmt_offset + 0x02) as i32;
        ww.sample_rate = read_u32(sf, ww.fmt_offset + 0x04) as i32;
        ww.avg_bitrate = read_u32(sf, ww.fmt_offset + 0x08) as i32;
        ww.block_size = read_u16(sf, ww.fmt_offset + 0x0c) as i32;
        ww.bits_per_sample = read_u16(sf, ww.fmt_offset + 0x0e) as i32;
        if ww.fmt_size > 0x10 && ww.format != 0x0165 && ww.format != 0x0166 {
            /* ignore XMAWAVEFORMAT */
            ww.extra_size = read_u16(sf, ww.fmt_offset + 0x10) as isize;
        }
        if ww.extra_size >= 0x06 {
            /* always present (actual RIFFs only have it in WAVEFORMATEXTENSIBLE) */
            /* mostly WAVEFORMATEXTENSIBLE's bitmask (see AkSpeakerConfig.h) */
            let mapping32 = read_u32(sf, ww.fmt_offset + 0x14);
            // ww.channel_layout = mapping32.try_into().unwrap();
            /* later games (+2018?) have a pseudo-format instead to handle more cases:
             * - 8b: uNumChannels
             * - 4b: eConfigType  (0=none, 1=standard, 2=ambisonic)
             * - 19b: uChannelMask */
            if (ww.channel_layout as u32 & 0xFF) == ww.channels as u32 {
                ww.channel_type = ((mapping32 >> 8) & 0x0F) as u8;
                ww.channel_layout = (mapping32 >> 12).try_into().unwrap();
            }
        }

        if ww.format == 0x0166 { /* XMA2WAVEFORMATEX in fmt */
            // xma2_parse_fmt_chunk_extra(sf, ww.fmt_offset, &ww.loop_flag, &ww.num_samples, &ww.loop_start_sample, &ww.loop_end_sample, ww.big_endian);
        }
    }
    /* common loops ("XMA2" chunks already read them) */
    if ww.smpl_offset != 0 {
        if ww.smpl_size >= 0x34
                && read_u32(sf, ww.smpl_offset + 0x1c) == 1           /* loop count */
                && read_u32(sf, ww.smpl_offset + 0x24 + 0x04) == 0
        {
            /* loop type */
            ww.loop_flag = true;
            ww.loop_start_sample = read_u32(sf, ww.smpl_offset + 0x24 + 0x8) as i32;
            ww.loop_end_sample = read_u32(sf, ww.smpl_offset + 0x24 + 0xc) as i32 + 1;
            /* +1 like standard RIFF */
        }
    }

    if ww.data_offset == 0 {
        return false;
    }

    /* format to codec */
    match ww.format {
        0x0001 => {
            ww.codec = WwiseCodec::PCM;
        } /* older Wwise */
        0x0002 => {
            ww.codec = WwiseCodec::IMA;
        } /* newer Wwise (variable, probably means "platform's ADPCM") */
        0x0069 => {
            ww.codec = WwiseCodec::IMA;
        } /* older Wwise [Spiderman Web of Shadows (X360), LotR Conquest (PC)] */
        0x0161 => {
            ww.codec = WwiseCodec::XWMA;
        } /* WMAv2 */
        0x0162 => {
            ww.codec = WwiseCodec::XWMA;
        } /* WMAPro */
        0x0165 => {
            ww.codec = WwiseCodec::XMA2;
        } /* XMA2-chunk XMA (Wwise doesn't use XMA1) */
        0x0166 => {
            ww.codec = WwiseCodec::XMA2;
        } /* fmt-chunk XMA */
        0xAAC0 => {
            ww.codec = WwiseCodec::AAC;
        }
        0xFFF0 => {
            ww.codec = WwiseCodec::DSP;
        }
        0xFFFB => {
            ww.codec = WwiseCodec::HEVAG;
        } /* "VAG" */
        0xFFFC => {
            ww.codec = WwiseCodec::ATRAC9;
        }
        0xFFFE => {
            ww.codec = WwiseCodec::PCM;
        } /* "PCM for Wwise Authoring" */
        0xFFFF => {
            ww.codec = WwiseCodec::VORBIS;
        }
        0x3039 => {
            ww.codec = WwiseCodec::OPUSNX;
        } /* renamed from "OPUS" on Wwise 2018.1 */
        0x3040 => {
            ww.codec = WwiseCodec::OPUS;
        }
        0x3041 => {
            ww.codec = WwiseCodec::OPUSWW;
        } /* "OPUS_WEM", added on Wwise 2019.2.3, replaces OPUS */
        0x8311 => {
            ww.codec = WwiseCodec::PTADPCM;
        } /* added on Wwise 2019.1, replaces IMA */
        _ => {
            /* some .wav may end up here, only report in .wem cases (newer codecs) */
            if ww.is_wem {
                println!("WWISE: unknown codec 0x{:x} (report)", ww.format)
            }
            return false;
        }
    }

    /* identify system's ADPCM */
    if ww.format == 0x0002 {
        if (ww.extra_size == 0x0c + ww.channels as isize * 0x2e) || (ww.extra_size == 0x0a && ww.wiih_offset != 0) {
            /* newer Wwise DSP with coefs [Epic Mickey 2 (Wii), Batman Arkham Origins Blackgate (3DS)] */
            ww.codec = WwiseCodec::DSP;
            /* WiiH */
            /* few older Wwise DSP with num_samples in extra_size [Tony Hawk: Shred (Wii)] */
        } else if ww.block_size == 0x104 * ww.channels {
            /* Bayonetta 2 (Switch) */
            ww.codec = WwiseCodec::PTADPCM;
        }
    }

    /* Some Wwise .bnk (RAM) files have truncated, prefetch mirrors of another file, that
     * play while the rest of the real stream loads. We'll add basic support to avoid
     * complaints of this or that .wem not playing */
    if ww.data_offset as isize + ww.data_size > ww.file_size {
        //;VGM_LOG("WWISE: truncated data size (prefetch): (real=0x%x > riff=0x%x)\n", ww.data_size, ww.file_size);

        /* catch wrong rips as truncated tracks' file_size should be much smaller than data_size,
         * but it's possible to pre-fetch small files too [Punch Out!! (Wii)] */
        if ww.data_offset as isize + ww.data_size - ww.file_size < 0x5000 && ww.file_size > 0x10000
        {
            println!("WWISE: wrong expected size (re-rip?)");
            return false;
        }

        if ww.codec == WwiseCodec::PCM
            || ww.codec == WwiseCodec::IMA
            || ww.codec == WwiseCodec::VORBIS
            || ww.codec == WwiseCodec::DSP
            || ww.codec == WwiseCodec::XMA2
            || ww.codec == WwiseCodec::OPUSNX
            || ww.codec == WwiseCodec::OPUS
            || ww.codec == WwiseCodec::OPUSWW
            || ww.codec == WwiseCodec::PTADPCM
            || ww.codec == WwiseCodec::XWMA
            || ww.codec == WwiseCodec::ATRAC9
        {
            ww.prefetch = true; /* only seen those, probably all exist (missing XWMA, AAC, HEVAG) */
        } else {
            println!("WWISE: wrong expected size, maybe prefetch (report)");
            return false;
        }
    }

    /* Cyberpunk 2077 has some mutant .wem, with proper Wwise header and PCMEX but data is standard OPUS.
     * From init bank and CAkSound's sources, those may be piped through their plugins. They come in
     * .opuspak (no names), have wrong riff/data sizes and only seem used for sfx (other audio is Vorbis). */
    if ww.format == 0xFFFE && ww.prefetch {
        if is_id32be(sf, ww.data_offset + 0x00, "OggS") {
            ww.codec = WwiseCodec::OPUSCPR;
        }
    }

    return true;
}
