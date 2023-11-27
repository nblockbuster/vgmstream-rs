#![allow(non_camel_case_types)]

use crate::{vgmstream::Streamfile, coding::vorbis::VorbisCustomCodecData};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct PlayConfig {
    pub config_set: bool, /* some of the mods below are set */

    // TODO: bools, probably
    
    /* modifiers */
    pub play_forever: i32,
    pub ignore_loop: i32,
    pub force_loop: i32,
    pub really_force_loop: i32,
    pub ignore_fade: i32,

    /* processing */
    pub loop_count: f64,
    pub pad_begin: i32,
    pub trim_begin: i32,
    pub body_time: i32,
    pub trim_end: i32,
    pub fade_delay: f64,
    pub fade_time: f64,
    pub pad_end: i32,

    pub pad_begin_s: f64,
    pub trim_begin_s: f64,
    pub body_time_s: f64,
    pub trim_end_s: f64,
    pub pad_end_s: f64,

    /* internal flags */
    pub pad_begin_set: i32,
    pub trim_begin_set: i32,
    pub body_time_set: i32,
    pub loop_count_set: i32,
    pub trim_end_set: i32,
    pub fade_delay_set: i32,
    pub fade_time_set: i32,
    pub pad_end_set: i32,

    /* for lack of a better place... */
    pub is_txtp: i32,
    pub is_mini_txtp: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PlayState {
    pub input_channels: i32,
    pub output_channels: i32,

    pub pad_begin_duration: i32,
    pub pad_begin_left: i32,
    pub trim_begin_duration: i32,
    pub trim_begin_left: i32,
    pub body_duration: i32,
    pub fade_duration: i32,
    pub fade_left: i32,
    pub fade_start: i32,
    pub pad_end_duration: i32,
    pub pad_end_start: i32,

    pub play_duration: i32,      /* total samples that the stream lasts (after applying all config) */
    pub play_position: i32,      /* absolute sample where stream is */
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VGMStreamChannel {
    pub streamfile: Option<Streamfile>,             /* file used by this channel */
    pub channel_start_offset: isize,        /* where data for this channel begins */
    pub offset: isize,                      /* current location in the file */

    pub frame_header_offset: isize,         /* offset of the current frame header (for WS) */
    pub samples_left_in_frame: i32,         /* for WS */

    /* format specific */

    /* adpcm */
    pub adpcm_coef: [i16; 16],              /* formats with decode coefficients built in (DSP, some ADX) */
    pub adpcm_coef_3by32: [i32; 0x60],      /* Level-5 0x555 */
    pub vadpcm_coefs: [i16; 8*2*8],         /* VADPCM: max 8 groups * max 2 order * fixed 8 subframe coefs */

    pub adpcm_history1_16: i16,             /* previous sample */
    pub adpcm_history1_32: i32,

    pub adpcm_history2_16: i16,             /* previous previous sample */
    pub adpcm_history2_32: i32,

    pub adpcm_history3_16: i16,
    pub adpcm_history3_32: i32,

    pub adpcm_history4_16: i16,
    pub adpcm_history4_32: i32,

    //double adpcm_history1_double;
    //double adpcm_history2_double;

    pub adpcm_step_index: i32,               /* for IMA */
    pub adpcm_scale: i32,                    /* for MS ADPCM */

    /* state for G.721 decoder, sort of big but we might as well keep it around */
    pub g72x_state: G72xState,

    /* ADX encryption */
    pub adx_channels: i32,
    pub adx_xor: u16,
    pub adx_mult: u16,
    pub adx_add: u16,
}

impl Default for VGMStreamChannel {
    fn default() -> Self {
        Self {
            streamfile: None,
            channel_start_offset: 0,
            offset: 0,
            frame_header_offset: 0,
            samples_left_in_frame: 0,
            adpcm_coef: [0; 16],
            adpcm_coef_3by32: [0; 0x60],
            vadpcm_coefs: [0; 8*2*8],
            adpcm_history1_16: 0,
            adpcm_history1_32: 0,
            adpcm_history2_16: 0,
            adpcm_history2_32: 0,
            adpcm_history3_16: 0,
            adpcm_history3_32: 0,
            adpcm_history4_16: 0,
            adpcm_history4_32: 0,
            adpcm_step_index: 0,
            adpcm_scale: 0,
            g72x_state: G72xState::default(),
            adx_channels: 0,
            adx_xor: 0,
            adx_mult: 0,
            adx_add: 0,
        }
    }
}

#[derive(Default)]
pub struct VGMStream {
    /* basic config */
    pub num_samples: i32,               /* the actual max number of samples */
    pub sample_rate: i32,               /* sample rate in Hz */
    pub channels: i32,                  /* number of channels */
    pub coding_type: CodingType,        /* type of encoding */
    pub layout_type: LayoutType,        /* type of layout */
    pub meta_type: MetaType,            /* type of metadata */

    /* looping config */
    pub loop_flag: bool,                 /* is this stream looped? */
    pub loop_start_sample: i32,         /* first sample of the loop (included in the loop) */
    pub loop_end_sample: i32,           /* last sample of the loop (not included in the loop) */

    /* layouts/block config */
    pub interleave_block_size: isize,       /* interleave, or block/frame size (depending on the codec) */
    pub interleave_first_block_size: isize, /* different interleave for first block */
    pub interleave_first_skip: isize,       /* data skipped before interleave first (needed to skip other channels) */
    pub interleave_last_block_size: isize,  /* smaller interleave for last block */
    pub frame_size: isize,                  /* for codecs with configurable size */

    /* subsong config */
    pub num_streams: i32,                   /* for multi-stream formats (0=not set/one stream, 1=one stream) */
    pub stream_index: i32,                  /* selected subsong (also 1-based) */
    pub stream_size: isize,                 /* info to properly calculate bitrate in case of subsongs */
    pub stream_name: String,          /* name of the current stream (info), if the file stores it and it's filled */

    /* mapping config (info for plugins) see channel_mappings.h */
    pub channel_layout: u32,                /* order: FL FR FC LFE BL BR FLC FRC BC SL SR etc (WAVEFORMATEX flags where FL=lowest bit set) */

    /* other config */
    pub allow_dual_stereo: bool,            /* search for dual stereo (file_L.ext + file_R.ext = single stereo file) */

    /* layout/block state */
    pub full_block_size: isize,             /* actual data size of an entire block (ie. may be fixed, include padding/headers, etc) */
    pub current_sample: isize,              /* sample point within the file (for loop detection) */
    pub samples_into_block: isize,          /* number of samples into the current block/interleave/segment/etc */
    pub current_block_offset: isize,        /* start of this block (offset of block header) */
    pub current_block_size: isize,          /* size in usable bytes of the block we're in now (used to calculate num_samples per block) */
    pub current_block_samples: i32,         /* size in samples of the block we're in now (used over current_block_size if possible) */
    pub next_block_offset: isize,           /* offset of header of the next block */

    /* loop state (saved when loop is hit to restore later) */
    pub loop_current_sample: i32,           /* saved from current_sample (same as loop_start_sample, but more state-like) */
    pub loop_samples_into_block: i32,       /* saved from samples_into_block */
    pub loop_block_offset: isize,           /* saved from current_block_offset */
    pub loop_block_size: isize,             /* saved from current_block_size */
    pub loop_block_samples: i32,            /* saved from current_block_samples */
    pub loop_next_block_offset: isize,      /* saved from next_block_offset */
    pub hit_loop: i32,                      /* save config when loop is hit, but first time only */

    /* decoder config/state */
    pub codec_endian: bool,                 /* little/big endian marker; name is left vague but usually means big endian */
    pub codec_config: i32,                  /* flags for codecs or layouts with minor variations; meaning is up to them */
    pub ws_output_size: i32,                /* WS ADPCM: output bytes for this block */

    /* main state */
    // VGMSTREAMCHANNEL* ch;           /* array of channels */
    // VGMSTREAMCHANNEL* start_ch;     /* shallow copy of channels as they were at the beginning of the stream (for resets) */
    // VGMSTREAMCHANNEL* loop_ch;      /* shallow copy of channels as they were at the loop point (for loops) */
    // void* start_vgmstream;          /* shallow copy of the VGMSTREAM as it was at the beginning of the stream (for resets) */

    // void* mixing_data;              /* state for mixing effects */

    /* Optional data the codec needs for the whole stream. This is for codecs too
     * different from vgmstream's structure to be reasonably shoehorned.
     * Note also that support must be added for resetting, looping and
     * closing for every codec that uses this, as it will not be handled. */
    // void* codec_data;
    /* Same, for special layouts. layout_data + codec_data may exist at the same time. */
    // void* layout_data;


    pub ch: Vec<VGMStreamChannel>,
    pub start_ch: Vec<VGMStreamChannel>,
    pub loop_ch: Vec<VGMStreamChannel>,

    pub start_vgmstream: Option<Box<VGMStream>>,

    // todo: uh
    // pub mixing_data: *c_void,
    pub codec_data: Option<VGMStreamCodecData>,
    pub layered_layout_data: Option<LayeredLayoutData>,
    pub segmented_layout_data: Option<SegmentedLayoutData>,

    /* play config/state */
    pub config_enabled: bool,           /* config can be used */
    pub config: PlayConfig,             /* player config (applied over decoding) */
    pub pstate: PlayState,              /* player state (applied over decoding) */
    pub loop_count: i32,                /* counter of complete loops (1=looped once) */
    pub loop_target: i32,               /* max loops before continuing with the stream end (loops forever if not set) */
    // todo: is this right?
    pub tmpbuf: Vec<u8>,                /* garbage buffer used for seeking/trimming */
    pub tmpbuf_size: isize,             /* for all channels (samples = tmpbuf_size / channels) */
}

use crate::coding::ffmpeg_opus::FFmpegCodecData;

/* i feel like theres a better way to implement this? */
// #[derive(Debug, Clone, Default)]
// pub struct VGMStreamCodecData {
//     pub custom_vorbis: Option<VorbisCustomCodecData>,
//     pub custom_ffmpeg: Option<FFmpegCodecData>,
// }

pub enum VGMStreamCodecData {
    CustomVorbis(VorbisCustomCodecData),
    CustomFFmpeg(FFmpegCodecData),
}

/* for files made of "continuous" segments, one per section of a song (using a complete sub-VGMSTREAM) */
#[derive(Default)]
pub struct SegmentedLayoutData {
    pub segment_count: i32,
    pub segments: Vec<VGMStream>,
    pub current_segment: i32,
    pub buffer: Vec<i16>,
    pub input_channels: i32,     /* internal buffer channels */
    pub output_channels: i32,    /* resulting channels (after mixing, if applied) */
    pub mixed_channels: bool,    /* segments have different number of channels */
}

/* for files made of "parallel" layers, one per group of channels (using a complete sub-VGMSTREAM) */
#[derive(Default)]
pub struct LayeredLayoutData {
    pub layer_count: i32,
    pub layers: Vec<VGMStream>,
    pub buffer: Vec<u8>,
    pub input_channels: i32,     /* internal buffer channels */
    pub output_channels: i32,    /* resulting channels (after mixing, if applied) */
    pub external_looping: i32,   /* don't loop using per-layer loops, but layout's own looping */
    pub curr_layer: i32,         /* helper */
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct VGMStreamInfo {
    pub sample_rate: i32,
    pub channels: i32,
    pub mixing_info: MixingInfo,
    pub channel_layout: i32,
    pub loop_info: LoopInfo,
    pub num_samples: isize,
    pub encoding: &'static str,
    pub layout: &'static str,
    pub interleave_info: InterleaveInfo,
    pub frame_size: i32,
    pub metadata: &'static str,
    pub bitrate: i32,
    pub stream_info: StreamInfo,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MixingInfo {
    pub input_channels: i32,
    pub output_channels: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct LoopInfo {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct InterleaveInfo {
    pub value: i32,
    pub first_block: i32,
    pub last_block: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct StreamInfo {
    pub current: i32,
    pub total: i32,
    pub name: &'static str,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum CodingType {
    #[default]
    coding_SILENCE,         /* generates silence */
    coding_PCM16LE,         /* little endian 16-bit PCM */
    coding_PCM16BE,         /* big endian 16-bit PCM */
    coding_PCM16_int,       /* 16-bit PCM with sample-level interleave (for blocks) */
    coding_PCM8,            /* 8-bit PCM */
    coding_PCM8_int,        /* 8-bit PCM with sample-level interleave (for blocks) */
    coding_PCM8_U,          /* 8-bit PCM, unsigned (0x80 = 0) */
    coding_PCM8_U_int,      /* 8-bit PCM, unsigned (0x80 = 0) with sample-level interleave (for blocks) */
    coding_PCM8_SB,         /* 8-bit PCM, sign bit (others are 2's complement) */
    coding_PCM4,            /* 4-bit PCM, signed */
    coding_PCM4_U,          /* 4-bit PCM, unsigned */
    coding_ULAW,            /* 8-bit u-Law (non-linear PCM) */
    coding_ULAW_int,        /* 8-bit u-Law (non-linear PCM) with sample-level interleave (for blocks) */
    coding_ALAW,            /* 8-bit a-Law (non-linear PCM) */
    coding_PCMFLOAT,        /* 32-bit float PCM */
    coding_PCM24LE,         /* little endian 24-bit PCM */
    coding_PCM24BE,         /* big endian 24-bit PCM */
    coding_PCM32LE,         /* little endian 32-bit PCM */
    coding_CRI_ADX,         /* CRI ADX */
    coding_CRI_ADX_fixed,   /* CRI ADX, encoding type 2 with fixed coefficients */
    coding_CRI_ADX_exp,     /* CRI ADX, encoding type 4 with exponential scale */
    coding_CRI_ADX_enc_8,   /* CRI ADX, type 8 encryption (God Hand) */
    coding_CRI_ADX_enc_9,   /* CRI ADX, type 9 encryption (PSO2) */
    coding_NGC_DSP,         /* Nintendo DSP ADPCM */
    coding_NGC_DSP_subint,  /* Nintendo DSP ADPCM with frame subinterframe */
    coding_NGC_DTK,         /* Nintendo DTK ADPCM (hardware disc), also called TRK or ADP */
    coding_NGC_AFC,         /* Nintendo AFC ADPCM */
    coding_VADPCM,          /* Silicon Graphics VADPCM */
    coding_G721,            /* CCITT G.721 */
    coding_XA,              /* CD-ROM XA 4-bit */
    coding_XA8,             /* CD-ROM XA 8-bit */
    coding_XA_EA,           /* EA's Saturn XA (not to be confused with EA-XA) */
    coding_PSX,             /* Sony PS ADPCM (VAG) */
    coding_PSX_badflags,    /* Sony PS ADPCM with custom flag byte */
    coding_PSX_cfg,         /* Sony PS ADPCM with configurable frame size (int math) */
    coding_PSX_pivotal,     /* Sony PS ADPCM with configurable frame size (float math) */
    coding_HEVAG,           /* Sony PSVita ADPCM */
    coding_EA_XA,           /* Electronic Arts EA-XA ADPCM v1 (stereo) aka "EA ADPCM" */
    coding_EA_XA_int,       /* Electronic Arts EA-XA ADPCM v1 (mono/interleave) */
    coding_EA_XA_V2,        /* Electronic Arts EA-XA ADPCM v2 */
    coding_MAXIS_XA,        /* Maxis EA-XA ADPCM */
    coding_EA_XAS_V0,       /* Electronic Arts EA-XAS ADPCM v0 */
    coding_EA_XAS_V1,       /* Electronic Arts EA-XAS ADPCM v1 */
    coding_IMA,             /* IMA ADPCM (stereo or mono, low nibble first) */
    coding_IMA_int,         /* IMA ADPCM (mono/interleave, low nibble first) */
    coding_DVI_IMA,         /* DVI IMA ADPCM (stereo or mono, high nibble first) */
    coding_DVI_IMA_int,     /* DVI IMA ADPCM (mono/interleave, high nibble first) */
    coding_NW_IMA,
    coding_SNDS_IMA,        /* Heavy Iron Studios .snds IMA ADPCM */
    coding_QD_IMA,
    coding_WV6_IMA,         /* Gorilla Systems WV6 4-bit IMA ADPCM */
    coding_HV_IMA,          /* High Voltage 4-bit IMA ADPCM */
    coding_FFTA2_IMA,       /* Final Fantasy Tactics A2 4-bit IMA ADPCM */
    coding_BLITZ_IMA,       /* Blitz Games 4-bit IMA ADPCM */
    coding_MS_IMA,          /* Microsoft IMA ADPCM */
    coding_MS_IMA_mono,     /* Microsoft IMA ADPCM (mono/interleave) */
    coding_XBOX_IMA,        /* XBOX IMA ADPCM */
    coding_XBOX_IMA_mch,    /* XBOX IMA ADPCM (multichannel) */
    coding_XBOX_IMA_int,    /* XBOX IMA ADPCM (mono/interleave) */
    coding_NDS_IMA,         /* IMA ADPCM w/ NDS layout */
    coding_DAT4_IMA,        /* Eurocom 'DAT4' IMA ADPCM */
    coding_RAD_IMA,         /* Radical IMA ADPCM */
    coding_RAD_IMA_mono,    /* Radical IMA ADPCM (mono/interleave) */
    coding_APPLE_IMA4,      /* Apple Quicktime IMA4 */
    coding_FSB_IMA,         /* FMOD's FSB multichannel IMA ADPCM */
    coding_WWISE_IMA,       /* Audiokinetic Wwise IMA ADPCM */
    coding_REF_IMA,         /* Reflections IMA ADPCM */
    coding_AWC_IMA,         /* Rockstar AWC IMA ADPCM */
    coding_UBI_IMA,         /* Ubisoft IMA ADPCM */
    coding_UBI_SCE_IMA,     /* Ubisoft SCE IMA ADPCM */
    coding_H4M_IMA,         /* H4M IMA ADPCM (stereo or mono, high nibble first) */
    coding_MTF_IMA,         /* Capcom MT Framework IMA ADPCM */
    coding_CD_IMA,          /* Crystal Dynamics IMA ADPCM */
    coding_MSADPCM,         /* Microsoft ADPCM (stereo/mono) */
    coding_MSADPCM_int,     /* Microsoft ADPCM (mono) */
    coding_MSADPCM_ck,      /* Microsoft ADPCM (Cricket Audio variation) */
    coding_WS,              /* Westwood Studios VBR ADPCM */
    coding_AICA,            /* Yamaha AICA ADPCM (stereo) */
    coding_AICA_int,        /* Yamaha AICA ADPCM (mono/interleave) */
    coding_CP_YM,           /* Capcom's Yamaha ADPCM (stereo/mono) */
    coding_ASKA,            /* Aska ADPCM */
    coding_NXAP,            /* NXAP ADPCM */
    coding_TGC,             /* Tiger Game.com 4-bit ADPCM */
    coding_NDS_PROCYON,     /* Procyon Studio ADPCM */
    coding_L5_555,          /* Level-5 0x555 ADPCM */
    coding_LSF,             /* lsf ADPCM (Fastlane Street Racing iPhone)*/
    coding_MTAF,            /* Konami MTAF ADPCM */
    coding_MTA2,            /* Konami MTA2 ADPCM */
    coding_MC3,             /* Paradigm MC3 3-bit ADPCM */
    coding_FADPCM,          /* FMOD FADPCM 4-bit ADPCM */
    coding_ASF,             /* Argonaut ASF 4-bit ADPCM */
    coding_DSA,             /* Ocean DSA 4-bit ADPCM */
    coding_XMD,             /* Konami XMD 4-bit ADPCM */
    coding_TANTALUS,        /* Tantalus 4-bit ADPCM */
    coding_PCFX,            /* PC-FX 4-bit ADPCM */
    coding_OKI16,           /* OKI 4-bit ADPCM with 16-bit output and modified expand */
    coding_OKI4S,           /* OKI 4-bit ADPCM with 16-bit output and cuadruple step */
    coding_PTADPCM,         /* Platinum 4-bit ADPCM */
    coding_IMUSE,           /* LucasArts iMUSE Variable ADPCM */
    coding_COMPRESSWAVE,    /* CompressWave Huffman ADPCM */
    coding_SDX2,            /* SDX2 2:1 Squareroot-Delta-Exact compression DPCM */
    coding_SDX2_int,        /* SDX2 2:1 Squareroot-Delta-Exact compression with sample-level interleave */
    coding_CBD2,            /* CBD2 2:1 Cuberoot-Delta-Exact compression DPCM */
    coding_CBD2_int,        /* CBD2 2:1 Cuberoot-Delta-Exact compression, with sample-level interleave */
    coding_SASSC,           /* Activision EXAKT SASSC 8-bit DPCM */
    coding_DERF,            /* DERF 8-bit DPCM */
    coding_WADY,            /* WADY 8-bit DPCM */
    coding_NWA,             /* VisualArt's NWA DPCM */
    coding_ACM,             /* InterPlay ACM */
    coding_CIRCUS_ADPCM,    /* Circus 8-bit ADPCM */
    coding_UBI_ADPCM,       /* Ubisoft 4/6-bit ADPCM */
    coding_EA_MT,           /* Electronic Arts MicroTalk (linear-predictive speech codec) */
    coding_CIRCUS_VQ,       /* Circus VQ */
    coding_RELIC,           /* Relic Codec (DCT-based) */
    coding_CRI_HCA,         /* CRI High Compression Audio (MDCT-based) */
    coding_TAC,             /* tri-Ace Codec (MDCT-based) */
    coding_ICE_RANGE,       /* Inti Creates "range" codec */
    coding_ICE_DCT,         /* Inti Creates "DCT" codec */
    coding_OGG_VORBIS,      /* Xiph Vorbis with Ogg layer (MDCT-based) */
    coding_VORBIS_custom,   /* Xiph Vorbis with custom layer (MDCT-based) */
    coding_MPEG_custom,     /* MPEG audio with custom features (MDCT-based) */
    coding_MPEG_ealayer3,   /* EALayer3, custom MPEG frames */
    coding_MPEG_layer1,     /* MP1 MPEG audio (MDCT-based) */
    coding_MPEG_layer2,     /* MP2 MPEG audio (MDCT-based) */
    coding_MPEG_layer3,     /* MP3 MPEG audio (MDCT-based) */
    coding_G7221C,          /* ITU G.722.1 annex C (Polycom Siren 14) */
    coding_G719,            /* ITU G.719 annex B (Polycom Siren 22) */
    coding_MP4_AAC,         /* AAC (MDCT-based) */
    coding_ATRAC9,          /* Sony ATRAC9 (MDCT-based) */
    coding_CELT_FSB,        /* Custom Xiph CELT (MDCT-based) */
    coding_SPEEX,           /* Custom Speex (CELP-based) */
    coding_FFmpeg,          /* Formats handled by FFmpeg (ATRAC3, XMA, AC3, etc) */
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum LayoutType {
    #[default]
    layout_none,            /* straight data */
    layout_interleave,      /* equal interleave throughout the stream */
    layout_blocked_ast,
    layout_blocked_halpst,
    layout_blocked_xa,
    layout_blocked_ea_schl,
    layout_blocked_ea_1snh,
    layout_blocked_caf,
    layout_blocked_wsi,
    layout_blocked_str_snds,
    layout_blocked_ws_aud,
    layout_blocked_dec,
    layout_blocked_xvas,
    layout_blocked_vs,
    layout_blocked_mul,
    layout_blocked_gsb,
    layout_blocked_thp,
    layout_blocked_filp,
    layout_blocked_ea_swvr,
    layout_blocked_adm,
    layout_blocked_mxch,
    layout_blocked_ivaud,   /* GTA IV .ivaud blocks */
    layout_blocked_ps2_iab,
    layout_blocked_vs_str,
    layout_blocked_rws,
    layout_blocked_hwas,
    layout_blocked_ea_sns,  /* newest Electronic Arts blocks, found in SNS/SNU/SPS/etc formats */
    layout_blocked_awc,     /* Rockstar AWC */
    layout_blocked_vgs,     /* Guitar Hero II (PS2) */
    layout_blocked_xwav,
    layout_blocked_xvag_subsong, /* XVAG subsongs [God of War III (PS4)] */
    layout_blocked_ea_wve_au00, /* EA WVE au00 blocks */
    layout_blocked_ea_wve_ad10, /* EA WVE Ad10 blocks */
    layout_blocked_sthd, /* Dream Factory STHD */
    layout_blocked_h4m, /* H4M video */
    layout_blocked_xa_aiff, /* XA in AIFF files [Crusader: No Remorse (SAT), Road Rash (3DO)] */
    layout_blocked_vs_square,
    layout_blocked_vid1,
    layout_blocked_ubi_sce,
    layout_blocked_tt_ad,
    layout_segmented,       /* song divided in segments (song sections) */
    layout_layered,         /* song divided in layers (song channels) */
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum MetaType {
    #[default]
    meta_SILENCE,

    meta_DSP_STD,           /* Nintendo standard GC ADPCM (DSP) header */
    meta_DSP_CSTR,          /* Star Fox Assault "Cstr" */
    meta_DSP_RS03,          /* Retro: Metroid Prime 2 "RS03" */
    meta_DSP_STM,           /* Paper Mario 2 STM */
    meta_AGSC,              /* Retro: Metroid Prime 2 title */
    meta_CSMP,              /* Retro: Metroid Prime 3 (Wii), Donkey Kong Country Returns (Wii) */
    meta_RFRM,              /* Retro: Donkey Kong Country Tropical Freeze (Wii U) */
    meta_DSP_MPDSP,         /* Monopoly Party single header stereo */
    meta_DSP_JETTERS,       /* Bomberman Jetters .dsp */
    meta_DSP_MSS,           /* Free Radical GC games */
    meta_DSP_GCM,           /* some of Traveller's Tales games */
    meta_DSP_STR,           /* Conan .str files */
    meta_DSP_SADB,          /* .sad */
    meta_DSP_WSI,           /* .wsi */
    meta_IDSP_TT,           /* Traveller's Tales games */
    meta_MUS_KROME,
    meta_DSP_WII_WSD,       /* Phantom Brave (WII) */
    meta_WII_NDP,           /* Vertigo (Wii) */
    meta_DSP_YGO,           /* Konami: Yu-Gi-Oh! The Falsebound Kingdom (NGC), Hikaru no Go 3 (NGC) */

    meta_STRM,              /* Nintendo STRM */
    meta_RSTM,              /* Nintendo RSTM (Revolution Stream, similar to STRM) */
    meta_AFC,               /* AFC */
    meta_AST,               /* AST */
    meta_RWSD,              /* single-stream RWSD */
    meta_RWAR,              /* single-stream RWAR */
    meta_RWAV,              /* contents of RWAR */
    meta_CWAV,              /* contents of CWAR */
    meta_FWAV,              /* contents of FWAR */
    meta_THP,               /* THP movie files */
    meta_SWAV,
    meta_NDS_RRDS,          /* Ridge Racer DS */
    meta_BNS,
    meta_WIIU_BTSND,        /* Wii U Boot Sound */

    meta_ADX_03,            /* CRI ADX "type 03" */
    meta_ADX_04,            /* CRI ADX "type 04" */
    meta_ADX_05,            /* CRI ADX "type 05" */
    meta_AIX,               /* CRI AIX */
    meta_AAX,               /* CRI AAX */
    meta_UTF_DSP,           /* CRI ADPCM_WII, like AAX with DSP */

    meta_DTK,
    meta_RSF,
    meta_HALPST,            /* HAL Labs HALPST */
    meta_GCSW,              /* GCSW (PCM) */
    meta_CAF,               /* tri-Crescendo CAF */
    meta_MYSPD,             /* U-Sing .myspd */
    meta_HIS,               /* Her Ineractive .his */
    meta_BNSF,              /* Bandai Namco Sound Format */

    meta_XA,                /* CD-ROM XA */
    meta_ADS,
    meta_NPS,
    meta_RXWS,
    meta_RAW_INT,
    meta_EXST,
    meta_SVAG_KCET,
    meta_PS_HEADERLESS,     /* headerless PS-ADPCM */
    meta_MIB_MIH,
    meta_PS2_MIC,           /* KOEI MIC File */
    meta_VAG,
    meta_VAG_custom,
    meta_AAAP,
    meta_SEB,
    meta_STR_WAV,           /* Blitz Games STR+WAV files */
    meta_ILD,
    meta_PWB,
    meta_VPK,               /* VPK Audio File */
    meta_PS2_BMDX,          /* Beatmania thing */
    meta_PS2_IVB,           /* Langrisser 3 IVB */
    meta_PS2_SND,           /* some Might & Magics SSND header */
    meta_SVS,               /* Square SVS */
    meta_XSS,               /* Dino Crisis 3 */
    meta_SL3,               /* Test Drive Unlimited */
    meta_HGC1,              /* Knights of the Temple 2 */
    meta_AUS,               /* Various Capcom games */
    meta_RWS,               /* RenderWare games (only when using RW Audio middleware) */
    meta_FSB1,              /* FMOD Sample Bank, version 1 */
    meta_FSB2,              /* FMOD Sample Bank, version 2 */
    meta_FSB3,              /* FMOD Sample Bank, version 3.0/3.1 */
    meta_FSB4,              /* FMOD Sample Bank, version 4 */
    meta_FSB5,              /* FMOD Sample Bank, version 5 */
    meta_RWAX,
    meta_XWB,               /* Microsoft XACT framework (Xbox, X360, Windows) */
    meta_PS2_XA30,          /* Driver - Parallel Lines (PS2) */
    meta_MUSC,              /* Krome PS2 games */
    meta_MUSX,
    meta_FILP,              /* Resident Evil - Dead Aim */
    meta_IKM,
    meta_STER,
    meta_BG00,              /* Ibara, Mushihimesama */
    meta_RSTM_ROCKSTAR,
    meta_PS2_KCES,          /* Dance Dance Revolution */
    meta_HXD,
    meta_VSV,
    meta_SCD_PCM,           /* Lunar - Eternal Blue */
    meta_PS2_PCM,           /* Konami KCEJ East: Ephemeral Fantasia, Yu-Gi-Oh! The Duelists of the Roses, 7 Blades */
    meta_PS2_RKV,           /* Legacy of Kain - Blood Omen 2 (PS2) */
    meta_PS2_VAS,           /* Pro Baseball Spirits 5 */
    meta_LP_AP_LEP,
    meta_SDT,               /* Baldur's Gate - Dark Alliance */
    meta_DC_STR,            /* SEGA Stream Asset Builder */
    meta_DC_STR_V2,         /* variant of SEGA Stream Asset Builder */
    meta_SAP,
    meta_DC_IDVI,           /* Eldorado Gate */
    meta_KRAW,              /* Geometry Wars - Galaxies */
    meta_OMU,
    meta_XA2_ACCLAIM,
    meta_NUB,
    meta_IDSP_NL,           /* Mario Strikers Charged (Wii) */
    meta_IDSP_IE,           /* Defencer (GC) */
    meta_SPT_SPD,           /* Various (SPT+SPT DSP) */
    meta_ISH_ISD,           /* Various (ISH+ISD DSP) */
    meta_GSND,
    meta_YDSP,              /* WWE Day of Reckoning */
    meta_FFCC_STR,          /* Final Fantasy: Crystal Chronicles */
    meta_UBI_JADE,          /* Beyond Good & Evil, Rayman Raving Rabbids */
    meta_GCA,               /* Metal Slug Anthology */
    meta_NGC_SSM,           /* Golden Gashbell Full Power */
    meta_PS2_JOE,           /* Wall-E / Pixar games */
    meta_YMF,
    meta_SADL,
    meta_FAG,               /* Jackie Chan - Stuntmaster */
    meta_PS2_MIHB,          /* Merged MIH+MIB */
    meta_NGC_PDT,           /* Mario Party 6 */
    meta_DC_ASD,            /* Miss Moonligh */
    meta_SPSD,
    meta_RSD,
    meta_PS2_ASS,           /* ASS */
    meta_SEG,               /* Eragon */
    meta_NDS_STRM_FFTA2,    /* Final Fantasy Tactics A2 */
    meta_KNON,
    meta_ZWDSP,             /* Zack and Wiki */
    meta_VGS,               /* Guitar Hero Encore - Rocks the 80s */
    meta_DCS_WAV,
    meta_SMP,
    meta_WII_SNG,           /* Excite Trucks */
    meta_MUL,
    meta_SAT_BAKA,          /* Crypt Killer */
    meta_VSF,
    meta_PS2_VSF_TTA,       /* Tiny Toon Adventures: Defenders of the Universe */
    meta_ADS_MIDWAY,
    meta_PS2_SPS,           /* Ape Escape 2 */
    meta_NGC_DSP_KONAMI,    /* Konami DSP header, found in various games */
    meta_UBI_CKD,           /* Ubisoft CKD RIFF header (Rayman Origins Wii) */
    meta_RAW_WAVM,
    meta_WVS,
    meta_XMU,
    meta_XVAS,
    meta_EA_SCHL,           /* Electronic Arts SCHl with variable header */
    meta_EA_SCHL_fixed,     /* Electronic Arts SCHl with fixed header */
    meta_EA_BNK,            /* Electronic Arts BNK */
    meta_EA_1SNH,           /* Electronic Arts 1SNh/EACS */
    meta_EA_EACS,
    meta_RAW_PCM,
    meta_GENH,              /* generic header */
    meta_AIFC,              /* Audio Interchange File Format AIFF-C */
    meta_AIFF,              /* Audio Interchange File Format */
    meta_STR_SNDS,          /* .str with SNDS blocks and SHDR header */
    meta_WS_AUD,
    meta_RIFF_WAVE,         /* RIFF, for WAVs */
    meta_RIFF_WAVE_POS,     /* .wav + .pos for looping (Ys Complete PC) */
    meta_RIFF_WAVE_labl,    /* RIFF w/ loop Markers in LIST-adtl-labl */
    meta_RIFF_WAVE_smpl,    /* RIFF w/ loop data in smpl chunk */
    meta_RIFF_WAVE_wsmp,    /* RIFF w/ loop data in wsmp chunk */
    meta_RIFF_WAVE_MWV,     /* .mwv RIFF w/ loop data in ctrl chunk pflt */
    meta_RIFX_WAVE,         /* RIFX, for big-endian WAVs */
    meta_RIFX_WAVE_smpl,    /* RIFX w/ loop data in smpl chunk */
    meta_XNB,               /* XNA Game Studio 4.0 */
    meta_PC_MXST,           /* Lego Island MxSt */
    meta_SAB,               /* Worms 4 Mayhem SAB+SOB file */
    meta_NWA,               /* Visual Art's NWA */
    meta_NWA_NWAINFOINI,    /* Visual Art's NWA w/ NWAINFO.INI for looping */
    meta_NWA_GAMEEXEINI,    /* Visual Art's NWA w/ Gameexe.ini for looping */
    meta_SAT_DVI,           /* Konami KCE Nagoya DVI (SAT games) */
    meta_DC_KCEY,           /* Konami KCE Yokohama KCEYCOMP (DC games) */
    meta_ACM,               /* InterPlay ACM header */
    meta_MUS_ACM,           /* MUS playlist of InterPlay ACM files */
    meta_DEC,               /* Falcom PC games (Xanadu Next, Gurumin) */
    meta_VS,                /* Men in Black .vs */
    meta_FFXI_BGW,          /* FFXI (PC) BGW */
    meta_FFXI_SPW,          /* FFXI (PC) SPW */
    meta_STS,
    meta_PS2_P2BT,          /* Pop'n'Music 7 Audio File */
    meta_PS2_GBTS,          /* Pop'n'Music 9 Audio File */
    meta_NGC_DSP_IADP,      /* Gamecube Interleave DSP */
    meta_PS2_MCG,           /* Gunvari MCG Files (was name .GCM on disk) */
    meta_ZSD,               /* Dragon Booster ZSD */
    meta_REDSPARK,          /* "RedSpark" RSD (MadWorld) */
    meta_IVAUD,             /* .ivaud GTA IV */
    meta_NDS_HWAS,          /* Spider-Man 3, Tony Hawk's Downhill Jam, possibly more... */
    meta_NGC_LPS,           /* Rave Master (Groove Adventure Rave)(GC) */
    meta_NAOMI_ADPCM,       /* NAOMI/NAOMI2 ARcade games */
    meta_SD9,               /* beatmaniaIIDX16 - EMPRESS (Arcade) */
    meta_2DX9,              /* beatmaniaIIDX16 - EMPRESS (Arcade) */
    meta_PS2_VGV,           /* Rune: Viking Warlord */
    meta_GCUB,
    meta_MAXIS_XA,          /* Sim City 3000 (PC) */
    meta_NGC_SCK_DSP,       /* Scorpion King (NGC) */
    meta_CAFF,              /* iPhone .caf */
    meta_EXAKT_SC,          /* Activision EXAKT .SC (PS2) */
    meta_WII_WAS,           /* DiRT 2 (WII) */
    meta_PONA_3DO,          /* Policenauts (3DO) */
    meta_PONA_PSX,          /* Policenauts (PSX) */
    meta_XBOX_HLWAV,        /* Half Life 2 (XBOX) */
    meta_AST_MV,
    meta_AST_MMV,
    meta_DMSG,              /* Nightcaster II - Equinox (XBOX) */
    meta_NGC_DSP_AAAP,      /* Turok: Evolution (NGC), Vexx (NGC) */
    meta_WB,
    meta_S14,               /* raw Siren 14, 24kbit mono */
    meta_SSS,               /* raw Siren 14, 48kbit stereo */
    meta_PS2_GCM,           /* NamCollection */
    meta_SMPL,
    meta_MSA,
    meta_VOI,
    meta_P3D,               /* Prototype P3D */
    meta_NGC_RKV,           /* Legacy of Kain - Blood Omen 2 (GC) */
    meta_DSP_DDSP,          /* Various (2 dsp files stuck together */
    meta_NGC_DSP_MPDS,      /* Big Air Freestyle, Terminator 3 */
    meta_DSP_STR_IG,        /* Micro Machines, Superman Superman: Shadow of Apokolis */
    meta_EA_SWVR,           /* Future Cop L.A.P.D., Freekstyle */
    meta_PS2_B1S,           /* 7 Wonders of the ancient world */
    meta_DSP_XIII,          /* XIII, possibly more (Ubisoft header???) */
    meta_DSP_CABELAS,       /* Cabelas games */
    meta_PS2_ADM,           /* Dragon Quest V (PS2) */
    meta_LPCM_SHADE,
    meta_PS2_VMS,           /* Autobahn Raser - Police Madness */
    meta_XAU,               /* XPEC Entertainment (Beat Down (PS2 Xbox), Spectral Force Chronicle (PS2)) */
    meta_GH3_BAR,           /* Guitar Hero III Mobile .bar */
    meta_FFW,               /* Freedom Fighters [NGC] */
    meta_DSP_DSPW,          /* Sengoku Basara 3 [WII] */
    meta_PS2_JSTM,          /* Tantei Jinguji Saburo - Kind of Blue (PS2) */
    meta_SQEX_SCD,          /* Square-Enix SCD */
    meta_NGC_NST_DSP,       /* Animaniacs [NGC] */
    meta_BAF,               /* Bizarre Creations (Blur, James Bond) */
    meta_XVAG,              /* Ratchet & Clank Future: Quest for Booty (PS3) */
    meta_CPS,
    meta_MSF,
    meta_PS3_PAST,          /* Bakugan Battle Brawlers (PS3) */
    meta_SGXD,              /* Sony: Folklore, Genji, Tokyo Jungle (PS3), Brave Story, Kurohyo (PSP) */
    meta_WII_RAS,           /* Donkey Kong Country Returns (Wii) */
    meta_SPM,
    meta_VGS_PS,
    meta_PS2_IAB,           /* Ueki no Housoku - Taosu ze Robert Juudan!! (PS2) */
    meta_VS_STR,            /* The Bouncer */
    meta_LSF_N1NJ4N,        /* .lsf n1nj4n Fastlane Street Racing (iPhone) */
    meta_XWAV,
    meta_RAW_SNDS,
    meta_PS2_WMUS,          /* The Warriors (PS2) */
    meta_HYPERSCAN_KVAG,    /* Hyperscan KVAG/BVG */
    meta_IOS_PSND,          /* Crash Bandicoot Nitro Kart 2 (iOS) */
    meta_ADP_WILDFIRE,
    meta_QD_ADP,
    meta_EB_SFX,            /* Excitebots .sfx */
    meta_EB_SF0,            /* Excitebots .sf0 */
    meta_MTAF,
    meta_ALP,
    meta_WPD,               /* Shuffle! (PC) */
    meta_MN_STR,            /* Mini Ninjas (PC/PS3/WII) */
    meta_MSS,               /* Guerilla: ShellShock Nam '67 (PS2/Xbox), Killzone (PS2) */
    meta_PS2_HSF,           /* Lowrider (PS2) */
    meta_IVAG,
    meta_PS2_2PFS,          /* Konami: Mahoromatic: Moetto - KiraKira Maid-San, GANTZ (PS2) */
    meta_PS2_VBK,           /* Disney's Stitch - Experiment 626 */
    meta_OTM,               /* Otomedius (Arcade) */
    meta_CSTM,              /* Nintendo 3DS CSTM (Century Stream) */
    meta_FSTM,              /* Nintendo Wii U FSTM (caFe? Stream) */
    meta_IDSP_NAMCO,
    meta_KT_WIIBGM,         /* Koei Tecmo WiiBGM */
    meta_KTSS,              /* Koei Tecmo Nintendo Stream (KNS) */
    meta_MCA,               /* Capcom MCA "MADP" */
    meta_ADX_MONSTER,
    meta_HCA,               /* CRI HCA */
    meta_SVAG_SNK,
    meta_PS2_VDS_VDM,       /* Graffiti Kingdom */
    meta_FFMPEG,
    meta_FFMPEG_faulty,
    meta_CXS,
    meta_AKB,
    meta_PASX,
    meta_XMA_RIFF,
    meta_ASTB,
    meta_WWISE_RIFF,        /* Audiokinetic Wwise RIFF/RIFX */
    meta_UBI_RAKI,          /* Ubisoft RAKI header (Rayman Legends, Just Dance 2017) */
    meta_SNDX,
    meta_OGL,               /* Shin'en Wii/WiiU (Jett Rocket (Wii), FAST Racing NEO (WiiU)) */
    meta_MC3,               /* Paradigm games (T3 PS2, MX Rider PS2, MI: Operation Surma PS2) */
    meta_GHS,
    meta_AAC_TRIACE,
    meta_MTA2,
    meta_XA_XA30,
    meta_XA_04SW,
    meta_TXTH,
    meta_SK_AUD,            /* Silicon Knights .AUD (Eternal Darkness GC) */
    meta_AHX,
    meta_STMA,
    meta_BINK,              /* RAD Game Tools BINK audio/video */
    meta_EA_SNU,            /* Electronic Arts SNU (Dead Space) */
    meta_AWC,               /* Rockstar AWC (GTA5, RDR) */
    meta_OPUS,              /* Nintendo Opus [Lego City Undercover (Switch)] */
    meta_PC_AST,            /* Dead Rising (PC) */
    meta_NAAC,              /* Namco AAC (3DS) */
    meta_UBI_SB,            /* Ubisoft banks */
    meta_EZW,               /* EZ2DJ (Arcade) EZWAV */
    meta_VXN,               /* Gameloft mobile games */
    meta_EA_SNR_SNS,        /* Electronic Arts SNR+SNS (Burnout Paradise) */
    meta_EA_SPS,            /* Electronic Arts SPS (Burnout Crash) */
    meta_VID1,
    meta_PC_FLX,            /* Ultima IX PC */
    meta_MOGG,              /* Harmonix Music Systems MOGG Vorbis */
    meta_OGG_VORBIS,        /* Ogg Vorbis */
    meta_OGG_SLI,           /* Ogg Vorbis file w/ companion .sli for looping */
    meta_OPUS_SLI,          /* Ogg Opus file w/ companion .sli for looping */
    meta_OGG_SFL,           /* Ogg Vorbis file w/ .sfl (RIFF SFPL) for looping */
    meta_OGG_KOVS,          /* Ogg Vorbis with header and encryption (Koei Tecmo Games) */
    meta_OGG_encrypted,     /* Ogg Vorbis with encryption */
    meta_KMA9,              /* Koei Tecmo [Nobunaga no Yabou - Souzou (Vita)] */
    meta_XWC,               /* Starbreeze games */
    meta_SQEX_SAB,          /* Square-Enix newest middleware (sound) */
    meta_SQEX_MAB,          /* Square-Enix newest middleware (music) */
    meta_WAF,               /* KID WAF [Ever 17 (PC)] */
    meta_WAVE,              /* EngineBlack games [Mighty Switch Force! (3DS)] */
    meta_WAVE_segmented,    /* EngineBlack games, segmented [Shantae and the Pirate's Curse (PC)] */
    meta_SMV,
    meta_NXAP,              /* Nex Entertainment games [Time Crisis 4 (PS3), Time Crisis Razing Storm (PS3)] */
    meta_EA_WVE_AU00,       /* Electronic Arts PS movies [Future Cop - L.A.P.D. (PS), Supercross 2000 (PS)] */
    meta_EA_WVE_AD10,       /* Electronic Arts PS movies [Wing Commander 3/4 (PS)] */
    meta_STHD,              /* STHD .stx [Kakuto Chojin (Xbox)] */
    meta_MP4,               /* MP4/AAC */
    meta_PCM_SRE,           /* .PCM+SRE [Viewtiful Joe (PS2)] */
    meta_DSP_MCADPCM,       /* Skyrim (Switch) */
    meta_UBI_LYN,           /* Ubisoft LyN engine [The Adventures of Tintin (multi)] */
    meta_MSB_MSH,           /* sfx companion of MIH+MIB */
    meta_TXTP,
    meta_SMC_SMH,           /* Wangan Midnight (System 246) */
    meta_PPST,              /* PPST [Parappa the Rapper (PSP)] */
    meta_SPS_N1,
    meta_UBI_BAO,
    meta_DSP_SWITCH_AUDIO,  /* Gal Gun 2 (Switch) */
    meta_H4M,               /* Hudson HVQM4 video [Resident Evil 0 (GC), Tales of Symphonia (GC)] */
    meta_ASF,               /* Argonaut ASF [Croc 2 (PC)] */
    meta_XMD,               /* Konami XMD [Silent Hill 4 (Xbox), Castlevania: Curse of Darkness (Xbox)] */
    meta_CKS,               /* Cricket Audio stream [Part Time UFO (Android), Mega Man 1-6 (Android)] */
    meta_CKB,               /* Cricket Audio bank [Fire Emblem Heroes (Android), Mega Man 1-6 (Android)] */
    meta_WV6,               /* Gorilla Systems PC games */
    meta_WAVEBATCH,         /* Firebrand Games */
    meta_HD3_BD3,           /* Sony PS3 bank */
    meta_BNK_SONY,          /* Sony Scream Tool bank */
    meta_SSCF,
    meta_DSP_VAG,           /* Penny-Punching Princess (Switch) sfx */
    meta_DSP_ITL,           /* Charinko Hero (GC) */
    meta_A2M,               /* Scooby-Doo! Unmasked (PS2) */
    meta_AHV,               /* Headhunter (PS2) */
    meta_MSV,
    meta_SDF,
    meta_SVG,               /* Hunter - The Reckoning - Wayward (PS2) */
    meta_VIS,               /* AirForce Delta Strike (PS2) */
    meta_VAI,               /* Ratatouille (GC) */
    meta_AIF_ASOBO,         /* Ratatouille (PC) */
    meta_AO,                /* Cloudphobia (PC) */
    meta_APC,               /* MegaRace 3 (PC) */
    meta_WV2,               /* Slave Zero (PC) */
    meta_XAU_KONAMI,        /* Yu-Gi-Oh - The Dawn of Destiny (Xbox) */
    meta_DERF,              /* Stupid Invaders (PC) */
    meta_SADF,
    meta_UTK,
    meta_NXA,
    meta_ADPCM_CAPCOM,
    meta_UE4OPUS,
    meta_XWMA,
    meta_VA3,               /* DDR Supernova 2 AC */
    meta_XOPUS,
    meta_VS_SQUARE,
    meta_NWAV,
    meta_XPCM,
    meta_MSF_TAMASOFT,
    meta_XPS_DAT,
    meta_ZSND,
    meta_DSP_ADPY,
    meta_DSP_ADPX,
    meta_OGG_OPUS,
    meta_IMC,
    meta_GIN,
    meta_DSF,
    meta_208,
    meta_DSP_DS2,
    meta_MUS_VC,
    meta_STRM_ABYLIGHT,
    meta_MSF_KONAMI,
    meta_XWMA_KONAMI,
    meta_9TAV,
    meta_BWAV,
    meta_RAD,
    meta_SMACKER,
    meta_MZRT,
    meta_XAVS,
    meta_PSF,
    meta_DSP_ITL_i,
    meta_IMA,
    meta_XWV_VALVE,
    meta_UBI_HX,
    meta_BMP_KONAMI,
    meta_ISB,
    meta_XSSB,
    meta_XMA_UE3,
    meta_FWSE,
    meta_FDA,
    meta_TGC,
    meta_KWB,
    meta_LRMD,
    meta_WWISE_FX,
    meta_DIVA,
    meta_IMUSE,
    meta_KTSR,
    meta_KAT,
    meta_PCM_SUCCESS,
    meta_ADP_KONAMI,
    meta_SDRH,
    meta_WADY,
    meta_DSP_SQEX,
    meta_DSP_WIIVOICE,
    meta_SBK,
    meta_DSP_WIIADPCM,
    meta_DSP_CWAC,
    meta_COMPRESSWAVE,
    meta_KTAC,
    meta_MJB_MJH,
    meta_BSNF,
    meta_TAC,
    meta_IDSP_TOSE,
    meta_DSP_KWA,
    meta_OGV_3RDEYE,
    meta_PIFF_TPCM,
    meta_WXD_WXH,
    meta_BNK_RELIC,
    meta_XSH_XSD_XSS,
    meta_PSB,
    meta_LOPU_FB,
    meta_LPCM_FB,
    meta_WBK,
    meta_WBK_NSLB,
    meta_DSP_APEX,
    meta_MPEG,
    meta_SSPF,
    meta_S3V,
    meta_ESF,
    meta_ADM,
    meta_TT_AD,
    meta_SNDZ,
    meta_VAB,
    meta_BIGRP,
    meta_DIC1,
    meta_AWD,
    meta_SQUEAKSTREAM,
    meta_SQUEAKSAMPLE,
    meta_SNDS,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct G72xState {
    pub yl: i64,    /* Locked or steady state step size multiplier. */
    pub yu: i16,    /* Unlocked or non-steady state step size multiplier. */
    pub dms: i16,   /* Short term energy estimate. */
    pub dml: i16,   /* Long term energy estimate. */
    pub ap: i16,    /* Linear weighting coefficient of 'yl' and 'yu'. */

    pub a: [i16;2], /* Coefficients of pole portion of prediction filter. */
    pub b: [i16;6], /* Coefficients of zero portion of prediction filter. */
    pub pk: [i16;2],    /*
             * Signs of previous two samples of a partially
             * reconstructed signal.
             */
    pub dq: [i16;6],    /*
             * Previous 6 samples of the quantized difference
             * signal represented in an internal floating point
             * format.
             */
    pub sr: [i16;2],    /*
             * Previous 2 samples of the quantized difference
             * signal represented in an internal floating point
             * format.
             */
    pub td: char,   /* delayed tone detect, new in 1988 version */
}
