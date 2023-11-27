use crate::vgmstream_types::*;
use crate::vgmstream_types::LayoutType::{layout_layered, layout_segmented};

pub const EXTENSION_LIST: [&str; 625] = [
    "208",
    "2dx9",
    "2pfs",
    "3do",
    "3ds", //txth/reserved [F1 2011 (3DS)]
    "4", //for Game.com audio
    "8", //txth/reserved [Gungage (PS1)]
    "800",
    "9tav",
    "a3c", //txth/reserved [Puyo Puyo 20th Anniversary (PSP)]
    "aa3", //FFmpeg/not parsed (ATRAC3/ATRAC3PLUS/MP3/LPCM/WMA)
    "aax",
    "abc", //txth/reserved [Find My Own Way (PS2) tech demo]
    "abk",
    "acb",
    "acm",
    "acx",
    "ad", //txth/reserved [Xenosaga Freaks (PS2)]
    "adc", //txth/reserved [Tomb Raider The Last Revelation (DC), Tomb Raider Chronicles (DC)]
    "adm",
    "adp",
    "adpcm",
    "adpcmx",
    "ads",
    "adw",
    "adx",
    "afc",
    "afs2",
    "agsc",
    "ahx",
    "ahv",
    "ai",
    "aifc", //common?
    "aix",
    "akb",
    "al", //txth/raw [Dominions 3 - The Awakening (PC)]
    "al2", //txth/raw [Conquest of Elysium 3 (PC)]
    "ams", //txth/reserved [Super Dragon Ball Z (PS2) ELF names]
    "an2",
    "ao",
    "ap",
    "apc",
    "as4",
    "asbin",
    "asd",
    "asf",
    "asr",
    "ass",
    "ast",
    "at3",
    "at9",
    "atsl",
    "atsl3",
    "atsl4",
    "atslx",
    "atx",
    "aud",
    "audio", //txth/reserved [Grimm Echoes (Android)]
    "audio_data",
    "aus",
    "awa", //txth/reserved [Missing Parts Side A (PS2)]
    "awb",
    "awc",
    "awd",
    "b1s",
    "baf",
    "baka",
    "bank",
    "bar",
    "bcstm",
    "bcwav",
    "bcv", //txth/reserved [The Bigs (PSP)]
    "bfstm",
    "bfwav",
    "bg00",
    "bgm",
    "bgw",
    "bigrp",
    "bik",
    "bika", //fake extension for .bik (to be removed)
    "bik2",
    "binka", //FFmpeg/not parsed (BINK AUDIO)
    "bk2",
    "bkr",  //txth/reserved [P.N.03 (GC), Viewtiful Joe (GC)]
    "blk",
    "bmdx", //fake extension (to be removed?)
    "bms",
    "bnk",
    "bnm",
    "bns",
    "bnsf",
    "bo2",
    "brstm",
    "brstmspm",
    "brwav",
    "brwsd", //fake extension for RWSD (non-format)
    "bsnd",
    "btsnd",
    "bvg",
    "bwav",
    "cads",
    "caf",
    "cbd2",
    "cd",
    "cfn", //fake extension for CAF (renamed, to be removed?)
    "chd", //txth/reserved [Donkey Konga (GC), Star Fox Assault (GC)]
    "chk",
    "ckb",
    "ckd",
    "cks",
    "cnk",
    "cpk",
    "cps",
    "csa", //txth/reserved [LEGO Racers 2 (PS2)]
    "csb",
    "csmp",
    "cvs", //txth/reserved [Aladdin in Nasira's Revenge (PS1)]
    "cwav",
    "cxs",
    "d2", //txth/reserved [Dodonpachi Dai-Ou-Jou (PS2)]
    "da",
    "data",
    "dax",
    "dbm",
    "dct",
    "dcs",
    "ddsp",
    "de2",
    "dec",
    "dic",
    "diva",
    "dmsg", //fake extension/header id for .sgt (to be removed)
    "ds2", //txth/reserved [Star Wars Bounty Hunter (GC)]
    "dsb",
    "dsf",
    "dsp",
    "dspw",
    "dtk",
    "dvi",
    "dyx", //txth/reserved [Shrek 4 (iOS)]
    "e4x",
    "eam",
    "eas",
    "eda", //txth/reserved [Project Eden (PS2)]
    "emff", //fake extension for .mul (to be removed)
    "enm",
    "eno",
    "ens",
    "esf",
    "exa",
    "ezw",
    "fag",
    "fcb", //FFmpeg/not parsed (BINK AUDIO)
    "fda",
    "ffw",
    "filp",
    "flx",
    "fsb",
    "fsv",
    "fwav",
    "fwse",
    "g1l",
    "gbts",
    "gca",
    "gcm",
    "gcub",
    "gcw",
    "genh",
    "gin",
    "gmd",  //txth/semi [High Voltage games: Charlie and the Chocolate Factory (GC), Zathura (GC)]
    "gms",
    "grn",
    "gsf",
    "gsp",
    "gtd",
    "gwm",
    "h4m",
    "hab",
    "hca",
    "hd3",
    "hdr",
    "hdt",
    "his",
    "hps",
    "hsf",
    "hvqm",
    "hwx", //txth/reserved [Star Wars Episode III (Xbox)]
    "hx2",
    "hx3",
    "hxc",
    "hxd",
    "hxg",
    "hxx",
    "hwas",
    "hwb",
    "hwd",
    "iab",
    "iadp",
    "idmsf",
    "idsp",
    "idvi", //fake extension/header id for .pcm (renamed, to be removed)
    "idwav",
    "idx",
    "idxma",
    "ifs",
    "ikm",
    "ild",
    "ilf", //txth/reserved [Madden NFL 98 (PS1)]
    "ilv", //txth/reserved [Star Wars Episode III (PS2)]
    "ima",
    "imc",
    "imf",
    "imx",
    "int",
    "is14",
    "isb",
    "isd",
    "isws",
    "itl",
    "ivaud",
    "ivag",
    "ivb",
    "ivs", //txth/reserved [Burnout 2 (PS2)]
    "joe",
    "jstm",
    "kat",
    "kces",
    "kcey", //fake extension/header id for .pcm (renamed, to be removed)
    "km9",
    "kma",  //txth/reserved [Dynasty Warriors 7: Empires (PS3)]
    "kmx",
    "kovs", //fake extension/header id for .kvs
    "kno",
    "kns",
    "koe",
    "kraw",
    "ktac",
    "ktsl2asbin",
    "ktss", //fake extension/header id for .kns
    "kvs",
    "kwa",
    "l",
    "l00", //txth/reserved [Disney's Dinosaur (PS2)]
    "laac", //fake extension for .aac (tri-Ace)
    "ladpcm", //not fake
    "laif", //fake extension for .aif (various)
    "laiff", //fake extension for .aiff
    "laifc", //fake extension for .aifc
    "lac3", //fake extension for .ac3, FFmpeg/not parsed
    "lasf", //fake extension for .asf (various)
    "lbin", //fake extension for .bin (various)
    "ldat", //fake extension for .dat
    "ldt",
    "leg",
    "lep",
    "lflac", //fake extension for .flac, FFmpeg/not parsed
    "lin",
    "lm0",
    "lm1",
    "lm2",
    "lm3",
    "lm4",
    "lm5",
    "lm6",
    "lm7",
    "lmp2", //fake extension for .mp2, FFmpeg/not parsed
    "lmp3", //fake extension for .mp3, FFmpeg/not parsed
    "lmp4", //fake extension for .mp4
    "lmpc", //fake extension for .mpc, FFmpeg/not parsed
    "logg", //fake extension for .ogg
    "lopus", //fake extension for .opus, used by LOPU too
    "lp",
    "lpcm",
    "lpk",
    "lps",
    "lrmh",
    "lse",
    "lsf",
    "lstm", //fake extension for .stm
    "lwav", //fake extension for .wav
    "lwd",
    "lwma", //fake extension for .wma, FFmpeg/not parsed
    "mab",
    "mad",
    "map",
    "mc3",
    "mca",
    "mcadpcm",
    "mcg",
    "mds",
    "mdsp",
    "med",
    "mjb",
    "mi4", //fake extension for .mib (renamed, to be removed)
    "mib",
    "mic",
    "mihb",
    "mnstr",
    "mogg",
    "mpdsp",
    "mpds",
    "mpf",
    "mps", //txth/reserved [Scandal (PS2)]
    "ms",
    "msa",
    "msb",
    "msd",
    "mse",
    "msf",
    "mss",
    "msv",
    "msvp", //fake extension/header id for .msv
    "mta2",
    "mtaf",
    "mul",
    "mups",
    "mus",
    "musc",
    "musx",
    "mvb", //txth/reserved [Porsche Challenge (PS1)]
    "mwa", //txth/reserved [Fatal Frame (Xbox)]
    "mwv",
    "mxst",
    "myspd",
    "n64",
    "naac",
    "nds",
    "ndp", //fake extension/header id for .nds
    "nlsd",
    "nop",
    "nps",
    "npsf", //fake extension/header id for .nps (in bigfiles)
    "nsa",
    "nsopus",
    "nub",
    "nub2",
    "nus3audio",
    "nus3bank",
    "nwa",
    "nwav",
    "nxa",
    "ogg_",
    "ogl",
    "ogv",
    "oma", //FFmpeg/not parsed (ATRAC3/ATRAC3PLUS/MP3/LPCM/WMA)
    "omu",
    "opu",
    "opusx",
    "otm",
    "oto", //txth/reserved [Vampire Savior (SAT)]
    "ovb", //txth/semi [namCollection: Tekken (PS2), Tekken 5: Tekken 1-3 (PS2)]
    "p04", //txth/reserved [Psychic Force 2012 (DC), Skies of Arcadia (DC)]
    "p16", //txth/reserved [Astal (SAT)]
    "p1d", //txth/reserved [Farming Simulator 18 (3DS)]
    "p2a", //txth/reserved [Thunderhawk Operation Phoenix (PS2)]
    "p2bt",
    "p3d",
    "past",
    "pcm",
    "pdt",
    "pk",
    "pona",
    "pos",
    "ps3",
    "psb",
    "psf",
    "psh", //fake extension for .vsv (to be removed)
    "psnd",
    "pwb",
    "r",
    "rac", //txth/reserved [Manhunt (Xbox)]
    "rad",
    "rak",
    "ras",
    "raw", //txth/reserved [Madden NHL 97 (PC)-pcm8u]
    "rda", //FFmpeg/reserved [Rhythm Destruction (PC)]
    "res", //txth/reserved [Spider-Man: Web of Shadows (PSP)]
    "rkv",
    "rnd",
    "rof",
    "rpgmvo",
    "rrds",
    "rsd",
    "rsf",
    "rsm",
    "rsnd", //txth/reserved [Birushana: Ichijuu no Kaze (Switch)]
    "rsp",
    "rstm", //fake extension/header id for .rstm (in bigfiles)
    "rvws",
    "rwar",
    "rwav",
    "rws",
    "rwsd",
    "rwx",
    "rxx", //txth/reserved [Full Auto (X360)]
    "s14",
    "s3s", //txth/reserved [DT Racer (PS2)]
    "s3v", //Sound Voltex (AC)
    "sab",
    "sad",
    "saf",
    "sag",
    "sam", //txth/reserved [Lost Kingdoms 2 (GC)]
    "sap",
    "sb0",
    "sb1",
    "sb2",
    "sb3",
    "sb4",
    "sb5",
    "sb6",
    "sb7",
    "sbk",
    "sbin",
    "sbr",
    "sbv",
    "sig",
    "sm0",
    "sm1",
    "sm2",
    "sm3",
    "sm4",
    "sm5",
    "sm6",
    "sm7",
    "sc",
    "scd",
    "sch",
    "sd9",
    "sdp", //txth/reserved [Metal Gear Arcade (AC)]
    "sdf",
    "sdt",
    "seb",
    "sed",
    "seg",
    "sem", //txth/reserved [Oretachi Game Center Zoku: Sonic Wings (PS2)]
    "sf0",
    "sfl",
    "sfs",
    "sfx",
    "sgb",
    "sgd",
    "sgt",
    "slb", //txth/reserved [THE Nekomura no Hitobito (PS2)]
    "sli",
    "smc",
    "smk",
    "smp",
    "smv",
    "snb",
    "snd",
    "snds",
    "sng",
    "sngw",
    "snr",
    "sns",
    "snu",
    "snz", //txth/reserved [Killzone HD (PS3)]
    "sod",
    "son",
    "spd",
    "spm",
    "sps",
    "spsd",
    "spw",
    "ss2",
    "ssd", //txth/reserved [Zack & Wiki (Wii)]
    "ssm",
    "sspr",
    "ssp",
    "sss",
    "ster",
    "sth",
    "stm",
    "str",
    "stream",
    "strm",
    "sts",
    "sts_cp3",
    "stx",
    "svag",
    "svs",
    "svg",
    "swag",
    "swav",
    "swd",
    "switch", //txth/reserved (.m4a-x.switch) [Ikinari Maou (Switch)]
    "switch_audio",
    "sx",
    "sxd",
    "sxd2",
    "sxd3",
    "szd",
    "szd1",
    "szd3",
    "tad",
    "tgq",
    "tgv",
    "thp",
    "tmx",
    "tra",
    "trk",
    "trs", //txth/semi [Kamiwaza (PS2), Shinobido (PS2)]
    "tun",
    "txth",
    "txtp",
    "u0",
    "ue4opus",
    "ulw", //txth/raw [Burnout (GC)]
    "um3",
    "utk",
    "uv",
    "v0",
    "va3",
    "vab",
    "vag",
    "vai",
    "vam", //txth/reserved [Rocket Power: Beach Bandits (PS2)]
    "vas",
    "vawx",
    "vb", //txth/reserved [Tantei Jinguji Saburo: Mikan no Rupo (PS1)]
    "vbk",
    "vbx", //txth/reserved [THE Taxi 2 (PS2)]
    "vca", //txth/reserved [Pac-Man World (PS1)]
    "vcb", //txth/reserved [Pac-Man World (PS1)]
    "vds",
    "vdm",
    "vgi", //txth/reserved [Time Crisis II (PS2)]
    "vgm", //txth/reserved [Maximo (PS2)]
    "vgs",
    "vgv",
    "vh",
    "vid",
    "vig",
    "vis",
    "vm4", //txth/reserved [Elder Gate (PS1)]
    "vms",
    "vmu", //txth/reserved [Red Faction (PS2)]
    "voi",
    "vp6",
    "vpk",
    "vs",
    "vsf",
    "vsv",
    "vxn",
    "w",
    "waa",
    "wac",
    "wad",
    "waf",
    "wam",
    "was",
    "wavc",
    "wave",
    "wavebatch",
    "wavm",
    "wavx", //txth/reserved [LEGO Star Wars (Xbox)]
    "way",
    "wb",
    "wb2",
    "wbd",
    "wbk",
    "wd",
    "wem",
    "wii",
    "wic", //txth/reserved [Road Rash (SAT)-videos]
    "wip", //txth/reserved [Colin McRae DiRT (PC)]
    "wlv", //txth/reserved [ToeJam & Earl III: Mission to Earth (DC)]
    "wmus", //fake extension (to be removed)
    "wp2",
    "wpd",
    "wsd",
    "wsi",
    "wst", //txth/reserved [3jigen Shoujo o Hogo Shimashita (PC)]
    "wua",
    "wv2",
    "wv6",
    "wvd", //txth/reserved [Donkey Kong Barrel Blast (Wii)]
    "wve",
    "wvs",
    "wvx",
    "wxd",
    "x",
    "x360audio", //fake extension for Unreal Engine 3 XMA (real extension unknown)
    "xa",
    "xa2",
    "xa30",
    "xai",
    "xag", //txth/reserved [Tamsoft's PS2 games]
    "xau",
    "xav",
    "xb", //txth/reserved [Scooby-Doo! Unmasked (Xbox)]
    "xen",
    "xma",
    "xma2",
    "xms",
    "xmu",
    "xmv",
    "xnb",
    "xsh",
    "xsf",
    "xse",
    "xsew",
    "xss",
    "xvag",
    "xvas",
    "xwav", //fake extension for .wav (renamed, to be removed)
    "xwb",
    "xmd",
    "xopus",
    "xps",
    "xwc",
    "xwm",
    "xwma",
    "xws",
    "xwv",
    "ydsp",
    "ymf",
    "zic",
    "zsd",
    "zsm",
    "zss",
    "zwdsp",
    "zwv",
    "vgmstream" /* fake extension, catch-all for FFmpeg/txth/etc */
];

pub const COMMON_EXTENSION_LIST: [&str; 19] = [
    "aac", //common
    "ac3", //common, FFmpeg/not parsed (AC3)
    "aif", //common
    "aiff", //common
    "bin", //common
    "dat", //common
    "flac", //common
    "m4a", //common
    "m4v", //common
    "mov", //common
    "mp+", //common [Moonshine Runners (PC)]
    "mp2", //common
    "mp3", //common
    "mp4", //common
    "mpc", //common
    "ogg", //common
    "opus", //common
    "wav", //common
    "wma", //common
];

// pub fn vgmstream_get_formats(size: usize) -> Vec<&'static str> {
//     let mut vec = Vec::new();
//     for i in 0..size {
//         vec.push(EXTENSION_LIST[i]);
//     }
//     vec
// }
//
// pub fn vgmstream_get_common_formats(size: usize) -> Vec<&'static str> {
//     let mut vec = Vec::new();
//     for i in 0..size {
//         vec.push(COMMON_EXTENSION_LIST[i]);
//     }
//     vec
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CodingInfo {
    pub coding_type: CodingType,
    pub description: &'static str,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LayoutInfo {
    pub layout_type: LayoutType,
    pub description: &'static str,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MetaInfo {
    pub meta_type: MetaType,
    pub description: &'static str,
}

pub const CODING_INFO_LIST: [CodingInfo; 132] = [
    CodingInfo{ coding_type: CodingType::coding_SILENCE,           description: "Silence"},
    CodingInfo{ coding_type: CodingType::coding_PCM16LE,           description: "Little Endian 16-bit PCM"},
    CodingInfo{ coding_type: CodingType::coding_PCM16BE,           description: "Big Endian 16-bit PCM"},
    CodingInfo{ coding_type: CodingType::coding_PCM16_int,         description: "16-bit PCM with 2 byte interleave (block)"},
    CodingInfo{ coding_type: CodingType::coding_PCM8,              description: "8-bit signed PCM"},
    CodingInfo{ coding_type: CodingType::coding_PCM8_int,          description: "8-bit signed PCM with 1 byte interleave (block)"},
    CodingInfo{ coding_type: CodingType::coding_PCM8_U,            description: "8-bit unsigned PCM"},
    CodingInfo{ coding_type: CodingType::coding_PCM8_U_int,        description: "8-bit unsigned PCM with 1 byte interleave (block)"},
    CodingInfo{ coding_type: CodingType::coding_PCM8_SB,           description: "8-bit PCM with sign bit"},
    CodingInfo{ coding_type: CodingType::coding_PCM4,              description: "4-bit signed PCM"},
    CodingInfo{ coding_type: CodingType::coding_PCM4_U,            description: "4-bit unsigned PCM"},
    CodingInfo{ coding_type: CodingType::coding_ULAW,              description: "8-bit u-Law"},
    CodingInfo{ coding_type: CodingType::coding_ULAW_int,          description: "8-bit u-Law with 1 byte interleave (block)"},
    CodingInfo{ coding_type: CodingType::coding_ALAW,              description: "8-bit a-Law"},
    CodingInfo{ coding_type: CodingType::coding_PCMFLOAT,          description: "32-bit float PCM"},
    CodingInfo{ coding_type: CodingType::coding_PCM24LE,           description: "24-bit Little Endian PCM"},
    CodingInfo{ coding_type: CodingType::coding_PCM24BE,           description: "24-bit Big Endian PCM"},
    CodingInfo{ coding_type: CodingType::coding_PCM32LE,           description: "32-bit Little Endian PCM"},
    CodingInfo{ coding_type: CodingType::coding_CRI_ADX,           description: "CRI ADX 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_CRI_ADX_fixed,     description: "CRI ADX 4-bit ADPCM (fixed coefficients)"},
    CodingInfo{ coding_type: CodingType::coding_CRI_ADX_exp,       description: "CRI ADX 4-bit ADPCM (exponential scale)"},
    CodingInfo{ coding_type: CodingType::coding_CRI_ADX_enc_8,     description: "CRI ADX 4-bit ADPCM (type 8 encryption)"},
    CodingInfo{ coding_type: CodingType::coding_CRI_ADX_enc_9,     description: "CRI ADX 4-bit ADPCM (type 9 encryption)"},
    CodingInfo{ coding_type: CodingType::coding_NGC_DSP,           description: "Nintendo DSP 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_NGC_DSP_subint,    description: "Nintendo DSP 4-bit ADPCM (subinterleave)"},
    CodingInfo{ coding_type: CodingType::coding_NGC_DTK,           description: "Nintendo DTK 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_NGC_AFC,           description: "Nintendo AFC 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_VADPCM,            description: "Silicon Graphics VADPCM 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_G721,              description: "CCITT G.721 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_XA,                description: "CD-ROM XA 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_XA8,               description: "CD-ROM XA 8-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_XA_EA,             description: "Electronic Arts XA 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_PSX,               description: "Playstation 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_PSX_badflags,      description: "Playstation 4-bit ADPCM (bad flags)"},
    CodingInfo{ coding_type: CodingType::coding_PSX_cfg,           description: "Playstation 4-bit ADPCM (configurable)"},
    CodingInfo{ coding_type: CodingType::coding_PSX_pivotal,       description: "Playstation 4-bit ADPCM (Pivotal)"},
    CodingInfo{ coding_type: CodingType::coding_HEVAG,             description: "Sony HEVAG 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_EA_XA,             description: "Electronic Arts EA-XA 4-bit ADPCM v1"},
    CodingInfo{ coding_type: CodingType::coding_EA_XA_int,         description: "Electronic Arts EA-XA 4-bit ADPCM v1 (mono/interleave)"},
    CodingInfo{ coding_type: CodingType::coding_EA_XA_V2,          description: "Electronic Arts EA-XA 4-bit ADPCM v2"},
    CodingInfo{ coding_type: CodingType::coding_MAXIS_XA,          description: "Maxis EA-XA 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_EA_XAS_V0,         description: "Electronic Arts EA-XAS 4-bit ADPCM v0"},
    CodingInfo{ coding_type: CodingType::coding_EA_XAS_V1,         description: "Electronic Arts EA-XAS 4-bit ADPCM v1"},
    CodingInfo{ coding_type: CodingType::coding_IMA,               description: "IMA 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_IMA_int,           description: "IMA 4-bit ADPCM (mono/interleave)"},
    CodingInfo{ coding_type: CodingType::coding_DVI_IMA,           description: "Intel DVI 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_DVI_IMA_int,       description: "Intel DVI 4-bit IMA ADPCM (mono/interleave)"},
    CodingInfo{ coding_type: CodingType::coding_NW_IMA,            description: "NintendoWare IMA 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_SNDS_IMA,          description: "Heavy Iron .snds 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_QD_IMA,            description: "Quantic Dream 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_WV6_IMA,           description: "Gorilla Systems WV6 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_HV_IMA,            description: "High Voltage 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_FFTA2_IMA,         description: "Final Fantasy Tactics A2 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_BLITZ_IMA,         description: "Blitz Games 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_MTF_IMA,           description: "MT Framework 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_MS_IMA,            description: "Microsoft 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_MS_IMA_mono,       description: "Microsoft 4-bit IMA ADPCM (mono/interleave)"},
    CodingInfo{ coding_type: CodingType::coding_XBOX_IMA,          description: "XBOX 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_XBOX_IMA_mch,      description: "XBOX 4-bit IMA ADPCM (multichannel)"},
    CodingInfo{ coding_type: CodingType::coding_XBOX_IMA_int,      description: "XBOX 4-bit IMA ADPCM (mono/interleave)"},
    CodingInfo{ coding_type: CodingType::coding_NDS_IMA,           description: "NDS-style 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_DAT4_IMA,          description: "Eurocom DAT4 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_RAD_IMA,           description: "Radical 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_RAD_IMA_mono,      description: "Radical 4-bit IMA ADPCM (mono/interleave)"},
    CodingInfo{ coding_type: CodingType::coding_APPLE_IMA4,        description: "Apple Quicktime 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_FSB_IMA,           description: "FSB 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_WWISE_IMA,         description: "Audiokinetic Wwise 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_REF_IMA,           description: "Reflections 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_AWC_IMA,           description: "Rockstar AWC 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_UBI_IMA,           description: "Ubisoft 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_UBI_SCE_IMA,       description: "Ubisoft 4-bit SCE IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_H4M_IMA,           description: "Hudson HVQM4 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_CD_IMA,            description: "Crystal Dynamics 4-bit IMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_MSADPCM,           description: "Microsoft 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_MSADPCM_int,       description: "Microsoft 4-bit ADPCM (mono/interleave)"},
    CodingInfo{ coding_type: CodingType::coding_MSADPCM_ck,        description: "Microsoft 4-bit ADPCM (Cricket Audio)"},
    CodingInfo{ coding_type: CodingType::coding_WS,                description: "Westwood Studios VBR ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_AICA,              description: "Yamaha AICA 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_AICA_int,          description: "Yamaha AICA 4-bit ADPCM (mono/interleave)"},
    CodingInfo{ coding_type: CodingType::coding_CP_YM,             description: "Capcom Yamaha 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_ASKA,              description: "tri-Ace Aska 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_NXAP,              description: "Nex NXAP 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_TGC,               description: "Tiger Game.com 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_NDS_PROCYON,       description: "Procyon Studio Digital Sound Elements NDS 4-bit APDCM"},
    CodingInfo{ coding_type: CodingType::coding_L5_555,            description: "Level-5 0x555 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_LSF,               description: "Gizmondo Studios Helsingborg LSF 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_MTAF,              description: "Konami MTAF 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_MTA2,              description: "Konami MTA2 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_MC3,               description: "Paradigm MC3 3-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_FADPCM,            description: "FMOD FADPCM 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_ASF,               description: "Argonaut ASF 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_TANTALUS,          description: "Tantalus 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_DSA,               description: "Ocean DSA 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_XMD,               description: "Konami XMD 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_PCFX,              description: "PC-FX 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_OKI16,             description: "OKI 4-bit ADPCM (16-bit output)"},
    CodingInfo{ coding_type: CodingType::coding_OKI4S,             description: "OKI 4-bit ADPCM (4-shift)"},
    CodingInfo{ coding_type: CodingType::coding_PTADPCM,           description: "Platinum 4-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_IMUSE,             description: "LucasArts iMUSE VIMA ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_COMPRESSWAVE,      description: "CompressWave Huffman ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_SDX2,              description: "Squareroot-delta-exact (SDX2) 8-bit DPCM"},
    CodingInfo{ coding_type: CodingType::coding_SDX2_int,          description: "Squareroot-delta-exact (SDX2) 8-bit DPCM with 1 byte interleave"},
    CodingInfo{ coding_type: CodingType::coding_CBD2,              description: "Cuberoot-delta-exact (CBD2) 8-bit DPCM"},
    CodingInfo{ coding_type: CodingType::coding_CBD2_int,          description: "Cuberoot-delta-exact (CBD2) 8-bit DPCM with 1 byte interleave"},
    CodingInfo{ coding_type: CodingType::coding_SASSC,             description: "Activision / EXAKT SASSC 8-bit DPCM"},
    CodingInfo{ coding_type: CodingType::coding_DERF,              description: "Xilam DERF 8-bit DPCM"},
    CodingInfo{ coding_type: CodingType::coding_WADY,              description: "Marble WADY 8-bit DPCM"},
    CodingInfo{ coding_type: CodingType::coding_NWA,               description: "VisualArt's NWA DPCM"},
    CodingInfo{ coding_type: CodingType::coding_ACM,               description: "InterPlay ACM"},
    CodingInfo{ coding_type: CodingType::coding_CIRCUS_ADPCM,      description: "Circus 8-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_UBI_ADPCM,         description: "Ubisoft 4/6-bit ADPCM"},
    CodingInfo{ coding_type: CodingType::coding_EA_MT,             description: "Electronic Arts MicroTalk"},
    CodingInfo{ coding_type: CodingType::coding_CIRCUS_VQ,         description: "Circus VQ"},
    CodingInfo{ coding_type: CodingType::coding_RELIC,             description: "Relic Codec"},
    CodingInfo{ coding_type: CodingType::coding_CRI_HCA,           description: "CRI HCA"},
    CodingInfo{ coding_type: CodingType::coding_TAC,               description: "tri-Ace Codec"},
    CodingInfo{ coding_type: CodingType::coding_ICE_RANGE,         description: "Inti Creates Range Codec"},
    CodingInfo{ coding_type: CodingType::coding_ICE_DCT,           description: "Inti Creates DCT Codec"},
    CodingInfo{ coding_type: CodingType::coding_OGG_VORBIS,        description: "Ogg Vorbis"},
    CodingInfo{ coding_type: CodingType::coding_VORBIS_custom,     description: "Custom Vorbis"},
    CodingInfo{ coding_type: CodingType::coding_MPEG_custom,       description: "Custom MPEG Audio"},
    CodingInfo{ coding_type: CodingType::coding_MPEG_ealayer3,     description: "EALayer3"},
    CodingInfo{ coding_type: CodingType::coding_MPEG_layer1,       description: "MPEG Layer I Audio (MP1)"},
    CodingInfo{ coding_type: CodingType::coding_MPEG_layer2,       description: "MPEG Layer II Audio (MP2)"},
    CodingInfo{ coding_type: CodingType::coding_MPEG_layer3,       description: "MPEG Layer III Audio (MP3)"},
    CodingInfo{ coding_type: CodingType::coding_G7221C,            description: "ITU G.722.1 annex C (Polycom Siren 14)"},
    CodingInfo{ coding_type: CodingType::coding_G719,              description: "ITU G.719 annex B (Polycom Siren 22)"},
    CodingInfo{ coding_type: CodingType::coding_ATRAC9,            description: "ATRAC9"},
    CodingInfo{ coding_type: CodingType::coding_CELT_FSB,          description: "Custom CELT"},
    CodingInfo{ coding_type: CodingType::coding_SPEEX,             description: "Custom Speex"},
    CodingInfo{ coding_type: CodingType::coding_FFmpeg,            description: "FFmpeg"},
    CodingInfo{ coding_type: CodingType::coding_MP4_AAC,           description: "MPEG-4 AAC"},
];

pub const LAYOUT_INFO_LIST: [LayoutInfo; 42] = [
    LayoutInfo{ layout_type: LayoutType::layout_none,                  description: "flat"},
    LayoutInfo{ layout_type: LayoutType::layout_interleave,            description: "interleave"},
    LayoutInfo{ layout_type: LayoutType::layout_segmented,             description: "segmented"},
    LayoutInfo{ layout_type: LayoutType::layout_layered,               description: "layered"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_mxch,          description: "blocked (MxCh)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ast,           description: "blocked (AST)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_halpst,        description: "blocked (HALPST)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_xa,            description: "blocked (XA)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ea_schl,       description: "blocked (EA SCHl)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ea_1snh,       description: "blocked (EA 1SNh)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_caf,           description: "blocked (CAF)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_wsi,           description: "blocked (WSI)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_xvas,          description: "blocked (.xvas)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_str_snds,      description: "blocked (.str SNDS)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ws_aud,        description: "blocked (Westwood Studios .aud)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_dec,           description: "blocked (DEC)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_vs,            description: "blocked (Melbourne House VS)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_mul,           description: "blocked (MUL)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_gsb,           description: "blocked (GSB)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_thp,           description: "blocked (THP)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_filp,          description: "blocked (FILP)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ea_swvr,       description: "blocked (EA SWVR)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_adm,           description: "blocked (ADM)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ivaud,         description: "blocked (IVAUD)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ps2_iab,       description: "blocked (IAB)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_vs_str,        description: "blocked (STR VS)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_rws,           description: "blocked (RWS)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_hwas,          description: "blocked (HWAS)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ea_sns,        description: "blocked (EA SNS)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_awc,           description: "blocked (AWC)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_vgs,           description: "blocked (VGS)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_xwav,          description: "blocked (XWAV)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_xvag_subsong,  description: "blocked (XVAG subsong)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ea_wve_au00,   description: "blocked (EA WVE au00)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ea_wve_ad10,   description: "blocked (EA WVE Ad10)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_sthd,          description: "blocked (STHD)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_h4m,           description: "blocked (H4M)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_xa_aiff,       description: "blocked (XA AIFF)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_vs_square,     description: "blocked (Square VS)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_vid1,          description: "blocked (VID1)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_ubi_sce,       description: "blocked (Ubi SCE)"},
    LayoutInfo{ layout_type: LayoutType::layout_blocked_tt_ad,         description: "blocked (TT AD)"},
];

pub const META_INFO_LIST: [MetaInfo; 453] = [
    MetaInfo{ meta_type: MetaType::meta_SILENCE,             description: "Silence"},
    MetaInfo{ meta_type: MetaType::meta_RSTM,                description: "Nintendo RSTM header"},
    MetaInfo{ meta_type: MetaType::meta_STRM,                description: "Nintendo STRM header"},
    MetaInfo{ meta_type: MetaType::meta_ADX_03,              description: "CRI ADX header (type 03)"},
    MetaInfo{ meta_type: MetaType::meta_ADX_04,              description: "CRI ADX header (type 04)"},
    MetaInfo{ meta_type: MetaType::meta_ADX_05,              description: "CRI ADX header (type 05)"},
    MetaInfo{ meta_type: MetaType::meta_AIX,                 description: "CRI AIX header"},
    MetaInfo{ meta_type: MetaType::meta_AAX,                 description: "CRI AAX header"},
    MetaInfo{ meta_type: MetaType::meta_UTF_DSP,             description: "CRI ADPCM_WII header"},
    MetaInfo{ meta_type: MetaType::meta_AGSC,                description: "Retro Studios AGSC header"},
    MetaInfo{ meta_type: MetaType::meta_CSMP,                description: "Retro Studios CSMP header"},
    MetaInfo{ meta_type: MetaType::meta_RFRM,                description: "Retro Studios RFRM header"},
    MetaInfo{ meta_type: MetaType::meta_DTK,                 description: "Nintendo .DTK raw header"},
    MetaInfo{ meta_type: MetaType::meta_RSF,                 description: "Retro Studios .RSF raw header"},
    MetaInfo{ meta_type: MetaType::meta_AFC,                 description: "Nintendo .AFC header"},
    MetaInfo{ meta_type: MetaType::meta_AST,                 description: "Nintendo .AST header"},
    MetaInfo{ meta_type: MetaType::meta_HALPST,              description: "HAL Laboratory HALPST header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_RS03,            description: "Retro Studios RS03 header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_STD,             description: "Nintendo DSP header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_CSTR,            description: "Namco Cstr header"},
    MetaInfo{ meta_type: MetaType::meta_GCSW,                description: "MileStone GCSW header"},
    MetaInfo{ meta_type: MetaType::meta_ADS,                 description: "Sony ADS header"},
    MetaInfo{ meta_type: MetaType::meta_NPS,                 description: "Namco NPSF header"},
    MetaInfo{ meta_type: MetaType::meta_RWSD,                description: "Nintendo RWSD header (single stream)"},
    MetaInfo{ meta_type: MetaType::meta_RWAR,                description: "Nintendo RWAR header (single stream)"},
    MetaInfo{ meta_type: MetaType::meta_RWAV,                description: "Nintendo RWAV header"},
    MetaInfo{ meta_type: MetaType::meta_CWAV,                description: "Nintendo CWAV header"},
    MetaInfo{ meta_type: MetaType::meta_FWAV,                description: "Nintendo FWAV header"},
    MetaInfo{ meta_type: MetaType::meta_XA,                  description: "Sony XA header"},
    MetaInfo{ meta_type: MetaType::meta_RXWS,                description: "Sony RXWS header"},
    MetaInfo{ meta_type: MetaType::meta_RAW_INT,             description: "PS2 .int raw header"},
    MetaInfo{ meta_type: MetaType::meta_OMU,                 description: "Outrage OMU Header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_STM,             description: "Intelligent Systems STM header"},
    MetaInfo{ meta_type: MetaType::meta_EXST,                description: "Sony EXST header"},
    MetaInfo{ meta_type: MetaType::meta_SVAG_KCET,           description: "Konami SVAG header"},
    MetaInfo{ meta_type: MetaType::meta_PS_HEADERLESS,       description: "Headerless PS-ADPCM raw header"},
    MetaInfo{ meta_type: MetaType::meta_MIB_MIH,             description: "Sony MultiStream MIH+MIB header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_MPDSP,           description: "Single DSP header stereo by .mpdsp extension"},
    MetaInfo{ meta_type: MetaType::meta_PS2_MIC,             description: "KOEI .MIC header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_JETTERS,         description: "Double DSP header stereo by _lr.dsp extension"},
    MetaInfo{ meta_type: MetaType::meta_DSP_MSS,             description: "Double DSP header stereo by .mss extension"},
    MetaInfo{ meta_type: MetaType::meta_DSP_GCM,             description: "Double DSP header stereo by .gcm extension"},
    MetaInfo{ meta_type: MetaType::meta_IDSP_TT,             description: "Traveller's Tales IDSP header"},
    MetaInfo{ meta_type: MetaType::meta_RAW_PCM,             description: "PC .raw raw header"},
    MetaInfo{ meta_type: MetaType::meta_VAG,                 description: "Sony VAG header"},
    MetaInfo{ meta_type: MetaType::meta_VAG_custom,          description: "Sony VAG header (custom)"},
    MetaInfo{ meta_type: MetaType::meta_AAAP,                description: "Acclaim Austin AAAp header"},
    MetaInfo{ meta_type: MetaType::meta_SEB,                 description: "Game Arts .SEB header"},
    MetaInfo{ meta_type: MetaType::meta_STR_WAV,             description: "Blitz Games .STR+WAV header"},
    MetaInfo{ meta_type: MetaType::meta_ILD,                 description: "Tose ILD header"},
    MetaInfo{ meta_type: MetaType::meta_PWB,                 description: "Double Fine WB header"},
    MetaInfo{ meta_type: MetaType::meta_RAW_WAVM,            description: "Xbox .wavm raw header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_STR,             description: "Cauldron .STR header"},
    MetaInfo{ meta_type: MetaType::meta_EA_SCHL,             description: "Electronic Arts SCHl header"},
    MetaInfo{ meta_type: MetaType::meta_EA_SCHL_fixed,       description: "Electronic Arts SCHl header (fixed)"},
    MetaInfo{ meta_type: MetaType::meta_CAF,                 description: "tri-Crescendo CAF Header"},
    MetaInfo{ meta_type: MetaType::meta_VPK,                 description: "SCE America VPK Header"},
    MetaInfo{ meta_type: MetaType::meta_GENH,                description: "GENH generic header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_SADB,            description: "Procyon Studio SADB header"},
    MetaInfo{ meta_type: MetaType::meta_SADL,                description: "Procyon Studio SADL header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_BMDX,            description: "Beatmania .bmdx header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_WSI,             description: "Alone in the Dark .WSI header"},
    MetaInfo{ meta_type: MetaType::meta_AIFC,                description: "Apple AIFF-C header"},
    MetaInfo{ meta_type: MetaType::meta_AIFF,                description: "Apple AIFF header"},
    MetaInfo{ meta_type: MetaType::meta_STR_SNDS,            description: "3DO SNDS header"},
    MetaInfo{ meta_type: MetaType::meta_WS_AUD,              description: "Westwood Studios .AUD header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_IVB,             description: "IVB/BVII header"},
    MetaInfo{ meta_type: MetaType::meta_SVS,                 description: "Square SVS header"},
    MetaInfo{ meta_type: MetaType::meta_RIFF_WAVE,           description: "RIFF WAVE header"},
    MetaInfo{ meta_type: MetaType::meta_RIFF_WAVE_POS,       description: "RIFF WAVE header (.pos looping)"},
    MetaInfo{ meta_type: MetaType::meta_NWA,                 description: "VisualArt's NWA header"},
    MetaInfo{ meta_type: MetaType::meta_NWA_NWAINFOINI,      description: "VisualArt's NWA header (NWAINFO.INI looping)"},
    MetaInfo{ meta_type: MetaType::meta_NWA_GAMEEXEINI,      description: "VisualArt's NWA header (Gameexe.ini looping)"},
    MetaInfo{ meta_type: MetaType::meta_XSS,                 description: "Dino Crisis 3 XSS File"},
    MetaInfo{ meta_type: MetaType::meta_HGC1,                description: "Cauldron HGC1 header"},
    MetaInfo{ meta_type: MetaType::meta_AUS,                 description: "Capcom AUS Header"},
    MetaInfo{ meta_type: MetaType::meta_RWS,                 description: "RenderWare RWS header"},
    MetaInfo{ meta_type: MetaType::meta_EA_1SNH,             description: "Electronic Arts 1SNh header"},
    MetaInfo{ meta_type: MetaType::meta_EA_EACS,             description: "Electronic Arts EACS header"},
    MetaInfo{ meta_type: MetaType::meta_SL3,                 description: "Atari Melbourne House SL3 header"},
    MetaInfo{ meta_type: MetaType::meta_FSB1,                description: "FMOD FSB1 header"},
    MetaInfo{ meta_type: MetaType::meta_FSB2,                description: "FMOD FSB2 header"},
    MetaInfo{ meta_type: MetaType::meta_FSB3,                description: "FMOD FSB3 header"},
    MetaInfo{ meta_type: MetaType::meta_FSB4,                description: "FMOD FSB4 header"},
    MetaInfo{ meta_type: MetaType::meta_FSB5,                description: "FMOD FSB5 header"},
    MetaInfo{ meta_type: MetaType::meta_RWAX,                description: "Konami RWAX header"},
    MetaInfo{ meta_type: MetaType::meta_XWB,                 description: "Microsoft XWB header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_XA30,            description: "Reflections XA30 PS2 header"},
    MetaInfo{ meta_type: MetaType::meta_MUSC,                description: "Krome MUSC header"},
    MetaInfo{ meta_type: MetaType::meta_MUSX,                description: "Eurocom MUSX header"},
    MetaInfo{ meta_type: MetaType::meta_FILP,                description: "cavia FILp Header"},
    MetaInfo{ meta_type: MetaType::meta_IKM,                 description: "MiCROViSiON IKM header"},
    MetaInfo{ meta_type: MetaType::meta_STER,                description: "ALCHEMY STER header"},
    MetaInfo{ meta_type: MetaType::meta_SAT_DVI,             description: "Konami DVI. header"},
    MetaInfo{ meta_type: MetaType::meta_DC_KCEY,             description: "Konami KCEY header"},
    MetaInfo{ meta_type: MetaType::meta_BG00,                description: "Cave BG00 header"},
    MetaInfo{ meta_type: MetaType::meta_RSTM_ROCKSTAR,       description: "Rockstar Games RSTM Header"},
    MetaInfo{ meta_type: MetaType::meta_ACM,                 description: "InterPlay ACM Header"},
    MetaInfo{ meta_type: MetaType::meta_MUS_ACM,             description: "InterPlay MUS ACM header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_KCES,            description: "Konami KCES Header"},
    MetaInfo{ meta_type: MetaType::meta_HXD,                 description: "Tecmo HXD Header"},
    MetaInfo{ meta_type: MetaType::meta_VSV,                 description: "Square Enix .vsv Header"},
    MetaInfo{ meta_type: MetaType::meta_RIFF_WAVE_labl,      description: "RIFF WAVE header (labl looping)"},
    MetaInfo{ meta_type: MetaType::meta_RIFF_WAVE_smpl,      description: "RIFF WAVE header (smpl looping)"},
    MetaInfo{ meta_type: MetaType::meta_RIFF_WAVE_wsmp,      description: "RIFF WAVE header (wsmp looping)"},
    MetaInfo{ meta_type: MetaType::meta_RIFX_WAVE,           description: "RIFX WAVE header"},
    MetaInfo{ meta_type: MetaType::meta_RIFX_WAVE_smpl,      description: "RIFX WAVE header (smpl looping)"},
    MetaInfo{ meta_type: MetaType::meta_XNB,                 description: "Microsoft XNA Game Studio header"},
    MetaInfo{ meta_type: MetaType::meta_SCD_PCM,             description: "Lunar: Eternal Blue .PCM header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_PCM,             description: "Konami .PCM header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_RKV,             description: "Legacy of Kain - Blood Omen 2 RKV PS2 header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_VAS,             description: "Konami .VAS header"},
    MetaInfo{ meta_type: MetaType::meta_LP_AP_LEP,           description: "Konami LP/AP/LEP header"},
    MetaInfo{ meta_type: MetaType::meta_SDT,                 description: "High Voltage .sdt header"},
    MetaInfo{ meta_type: MetaType::meta_WVS,                 description: "Swingin' Ape .WVS header"},
    MetaInfo{ meta_type: MetaType::meta_DEC,                 description: "Falcom .DEC RIFF header"},
    MetaInfo{ meta_type: MetaType::meta_VS,                  description: "Melbourne House .VS header"},
    MetaInfo{ meta_type: MetaType::meta_DC_STR,              description: "Sega Stream Asset Builder header"},
    MetaInfo{ meta_type: MetaType::meta_DC_STR_V2,           description: "variant of Sega Stream Asset Builder header"},
    MetaInfo{ meta_type: MetaType::meta_XMU,                 description: "Outrage XMU header"},
    MetaInfo{ meta_type: MetaType::meta_XVAS,                description: "Konami .XVAS header"},
    MetaInfo{ meta_type: MetaType::meta_XA2_ACCLAIM,         description: "Acclaim .XA2 Header"},
    MetaInfo{ meta_type: MetaType::meta_SAP,                 description: "VING .SAP header"},
    MetaInfo{ meta_type: MetaType::meta_DC_IDVI,             description: "Capcom IDVI header"},
    MetaInfo{ meta_type: MetaType::meta_KRAW,                description: "Geometry Wars: Galaxies KRAW header"},
    MetaInfo{ meta_type: MetaType::meta_YMF,                 description: "Yuke's .YMF Header"},
    MetaInfo{ meta_type: MetaType::meta_FAG,                 description: "Radical .FAG Header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_MIHB,            description: "Sony MultiStream MIC header"},
    MetaInfo{ meta_type: MetaType::meta_MUS_KROME,           description: "Krome .MUS header"},
    MetaInfo{ meta_type: MetaType::meta_WII_SNG,             description: "SNG DSP Header"},
    MetaInfo{ meta_type: MetaType::meta_RSD,                 description: "Radical RSD header"},
    MetaInfo{ meta_type: MetaType::meta_DC_ASD,              description: "ASD Header"},
    MetaInfo{ meta_type: MetaType::meta_SPSD,                description: "Sega Naomi SPSD header"},
    MetaInfo{ meta_type: MetaType::meta_FFXI_BGW,            description: "Square Enix .BGW header"},
    MetaInfo{ meta_type: MetaType::meta_FFXI_SPW,            description: "Square Enix .SPW header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_ASS,             description: "SystemSoft .ASS header"},
    MetaInfo{ meta_type: MetaType::meta_NUB,                 description: "Namco NUB header"},
    MetaInfo{ meta_type: MetaType::meta_IDSP_NL,             description: "Next Level IDSP header"},
    MetaInfo{ meta_type: MetaType::meta_IDSP_IE,             description: "Inevitable Entertainment IDSP Header"},
    MetaInfo{ meta_type: MetaType::meta_UBI_JADE,            description: "Ubisoft Jade RIFF header"},
    MetaInfo{ meta_type: MetaType::meta_SEG,                 description: "Stormfront SEG header"},
    MetaInfo{ meta_type: MetaType::meta_NDS_STRM_FFTA2,      description: "Final Fantasy Tactics A2 RIFF Header"},
    MetaInfo{ meta_type: MetaType::meta_KNON,                description: "Paon KNON header"},
    MetaInfo{ meta_type: MetaType::meta_ZWDSP,               description: "Zack and Wiki custom DSP Header"},
    MetaInfo{ meta_type: MetaType::meta_GCA,                 description: "GCA DSP Header"},
    MetaInfo{ meta_type: MetaType::meta_SPT_SPD,             description: "SPT+SPD DSP Header"},
    MetaInfo{ meta_type: MetaType::meta_ISH_ISD,             description: "ISH+ISD DSP Header"},
    MetaInfo{ meta_type: MetaType::meta_GSND,                description: "Tecmo GSND Header"},
    MetaInfo{ meta_type: MetaType::meta_YDSP,                description: "Yuke's YDSP Header"},
    MetaInfo{ meta_type: MetaType::meta_NGC_SSM,             description: "SSM DSP Header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_JOE,             description: "Asobo Studio .JOE header"},
    MetaInfo{ meta_type: MetaType::meta_VGS,                 description: "Guitar Hero VGS Header"},
    MetaInfo{ meta_type: MetaType::meta_DCS_WAV,             description: "In Utero DCS+WAV header"},
    MetaInfo{ meta_type: MetaType::meta_SMP,                 description: "Infernal Engine .smp header"},
    MetaInfo{ meta_type: MetaType::meta_MUL,                 description: "Crystal Dynamics .MUL header"},
    MetaInfo{ meta_type: MetaType::meta_THP,                 description: "Nintendo THP header"},
    MetaInfo{ meta_type: MetaType::meta_STS,                 description: "Alfa System .STS header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_P2BT,            description: "Pop'n'Music 7 Header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_GBTS,            description: "Pop'n'Music 9 Header"},
    MetaInfo{ meta_type: MetaType::meta_NGC_DSP_IADP,        description: "IADP Header"},
    MetaInfo{ meta_type: MetaType::meta_RIFF_WAVE_MWV,       description: "RIFF WAVE header (ctrl looping)"},
    MetaInfo{ meta_type: MetaType::meta_FFCC_STR,            description: "Final Fantasy: Crystal Chronicles STR header"},
    MetaInfo{ meta_type: MetaType::meta_SAT_BAKA,            description: "Konami BAKA header"},
    MetaInfo{ meta_type: MetaType::meta_SWAV,                description: "Nintendo SWAV header"},
    MetaInfo{ meta_type: MetaType::meta_VSF,                 description: "Square Enix VSF header"},
    MetaInfo{ meta_type: MetaType::meta_NDS_RRDS,            description: "Ridger Racer DS Header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_SND,             description: "Might and Magic SSND Header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_VSF_TTA,         description: "VSF with SMSS Header"},
    MetaInfo{ meta_type: MetaType::meta_ADS_MIDWAY,          description: "Midway ADS header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_MCG,             description: "Gunvari MCG Header"},
    MetaInfo{ meta_type: MetaType::meta_ZSD,                 description: "Konami ZSD header"},
    MetaInfo{ meta_type: MetaType::meta_REDSPARK,            description: "RedSpark Header"},
    MetaInfo{ meta_type: MetaType::meta_IVAUD,               description: "Rockstar .ivaud header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_WII_WSD,         description: ".WSD header"},
    MetaInfo{ meta_type: MetaType::meta_WII_NDP,             description: "Icon Games NDP header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_SPS,             description: "Ape Escape 2 SPS Header"},
    MetaInfo{ meta_type: MetaType::meta_NDS_HWAS,            description: "Vicarious Visions HWAS header"},
    MetaInfo{ meta_type: MetaType::meta_NGC_LPS,             description: "Rave Master LPS Header"},
    MetaInfo{ meta_type: MetaType::meta_NAOMI_ADPCM,         description: "NAOMI/NAOMI2 Arcade games ADPCM header"},
    MetaInfo{ meta_type: MetaType::meta_SD9,                 description: "beatmania IIDX SD9 header"},
    MetaInfo{ meta_type: MetaType::meta_2DX9,                description: "beatmania IIDX 2DX9 header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_YGO,             description: "Konami custom DSP Header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_VGV,             description: "Rune: Viking Warlord VGV Header"},
    MetaInfo{ meta_type: MetaType::meta_GCUB,                description: "Sega GCub header"},
    MetaInfo{ meta_type: MetaType::meta_NGC_SCK_DSP,         description: "The Scorpion King SCK Header"},
    MetaInfo{ meta_type: MetaType::meta_CAFF,                description: "Apple Core Audio Format File header"},
    MetaInfo{ meta_type: MetaType::meta_PC_MXST,             description: "Lego Island MxSt Header"},
    MetaInfo{ meta_type: MetaType::meta_SAB,                 description: "Sensaura SAB header"},
    MetaInfo{ meta_type: MetaType::meta_MAXIS_XA,            description: "Maxis XA Header"},
    MetaInfo{ meta_type: MetaType::meta_EXAKT_SC,            description: "assumed Activision / EXAKT SC by extension"},
    MetaInfo{ meta_type: MetaType::meta_BNS,                 description: "Nintendo BNS header"},
    MetaInfo{ meta_type: MetaType::meta_WII_WAS,             description: "Sumo Digital iSWS header"},
    MetaInfo{ meta_type: MetaType::meta_XBOX_HLWAV,          description: "Half-Life 2 .WAV header"},
    MetaInfo{ meta_type: MetaType::meta_MYSPD,               description: "Punchers Impact .MYSPD header"},
    MetaInfo{ meta_type: MetaType::meta_HIS,                 description: "Her Interactive HIS header"},
    MetaInfo{ meta_type: MetaType::meta_AST_MV,              description: "MicroVision AST header"},
    MetaInfo{ meta_type: MetaType::meta_AST_MMV,             description: "Marvelous AST header"},
    MetaInfo{ meta_type: MetaType::meta_DMSG,                description: "Microsoft RIFF DMSG header"},
    MetaInfo{ meta_type: MetaType::meta_PONA_3DO,            description: "Policenauts BGM header"},
    MetaInfo{ meta_type: MetaType::meta_PONA_PSX,            description: "Policenauts BGM header"},
    MetaInfo{ meta_type: MetaType::meta_NGC_DSP_AAAP,        description: "Acclaim Austin AAAp DSP header"},
    MetaInfo{ meta_type: MetaType::meta_NGC_DSP_KONAMI,      description: "Konami DSP header"},
    MetaInfo{ meta_type: MetaType::meta_BNSF,                description: "Namco Bandai BNSF header"},
    MetaInfo{ meta_type: MetaType::meta_WB,                  description: "Triangle Service .WB header"},
    MetaInfo{ meta_type: MetaType::meta_S14,                 description: "Namco .S14 raw header"},
    MetaInfo{ meta_type: MetaType::meta_SSS,                 description: "Namco .SSS raw header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_GCM,             description: "Namco GCM header"},
    MetaInfo{ meta_type: MetaType::meta_SMPL,                description: "Skonec SMPL header"},
    MetaInfo{ meta_type: MetaType::meta_MSA,                 description: "Success .MSA header"},
    MetaInfo{ meta_type: MetaType::meta_VOI,                 description: "Irem .VOI header"},
    MetaInfo{ meta_type: MetaType::meta_NGC_PDT,             description: "Hudson .PDT header"},
    MetaInfo{ meta_type: MetaType::meta_NGC_RKV,             description: "Legacy of Kain - Blood Omen 2 RKV GC header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_DDSP,            description: ".DDSP header"},
    MetaInfo{ meta_type: MetaType::meta_P3D,                 description: "Radical P3D header"},
    MetaInfo{ meta_type: MetaType::meta_NGC_DSP_MPDS,        description: "MPDS DSP header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_STR_IG,          description: "Infogrames .DSP header"},
    MetaInfo{ meta_type: MetaType::meta_EA_SWVR,             description: "Electronic Arts SWVR header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_B1S,             description: "B1S header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_XIII,            description: "XIII dsp header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_CABELAS,         description: "Cabelas games .DSP header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_ADM,             description: "Dragon Quest V .ADM raw header"},
    MetaInfo{ meta_type: MetaType::meta_LPCM_SHADE,          description: "Shade LPCM header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_VMS,             description: "VMS Header"},
    MetaInfo{ meta_type: MetaType::meta_XAU,                 description: "XPEC XAU header"},
    MetaInfo{ meta_type: MetaType::meta_GH3_BAR,             description: "Guitar Hero III Mobile .bar"},
    MetaInfo{ meta_type: MetaType::meta_FFW,                 description: "Freedom Fighters BGM header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_DSPW,            description: "Capcom DSPW header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_JSTM,            description: "JSTM Header"},
    MetaInfo{ meta_type: MetaType::meta_XVAG,                description: "Sony XVAG header"},
    MetaInfo{ meta_type: MetaType::meta_CPS,                 description: "tri-Crescendo CPS Header"},
    MetaInfo{ meta_type: MetaType::meta_SQEX_SCD,            description: "Square Enix SCD header"},
    MetaInfo{ meta_type: MetaType::meta_NGC_NST_DSP,         description: "Animaniacs NST header"},
    MetaInfo{ meta_type: MetaType::meta_BAF,                 description: "Bizarre Creations .baf header"},
    MetaInfo{ meta_type: MetaType::meta_MSF,                 description: "Sony MSF header"},
    MetaInfo{ meta_type: MetaType::meta_PS3_PAST,            description: "SNDP header"},
    MetaInfo{ meta_type: MetaType::meta_SGXD,                description: "Sony SGXD header"},
    MetaInfo{ meta_type: MetaType::meta_WII_RAS,             description: "RAS header"},
    MetaInfo{ meta_type: MetaType::meta_SPM,                 description: "Square SPM header"},
    MetaInfo{ meta_type: MetaType::meta_VGS_PS,              description: "Princess Soft VGS header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_IAB,             description: "Runtime .IAB header"},
    MetaInfo{ meta_type: MetaType::meta_VS_STR,              description: "Square .VS STRx header"},
    MetaInfo{ meta_type: MetaType::meta_LSF_N1NJ4N,          description: "Gizmondo Studios Helsingborg LSF header"},
    MetaInfo{ meta_type: MetaType::meta_XWAV,                description: "feelplus XWAV header"},
    MetaInfo{ meta_type: MetaType::meta_RAW_SNDS,            description: "PC .snds raw header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_WMUS,            description: "assumed The Warriors Sony ADPCM by .wmus extension"},
    MetaInfo{ meta_type: MetaType::meta_HYPERSCAN_KVAG,      description: "Mattel Hyperscan KVAG"},
    MetaInfo{ meta_type: MetaType::meta_IOS_PSND,            description: "PSND Header"},
    MetaInfo{ meta_type: MetaType::meta_ADP_WILDFIRE,        description: "Wildfire ADP! header"},
    MetaInfo{ meta_type: MetaType::meta_QD_ADP,              description: "Quantic Dream .ADP header"},
    MetaInfo{ meta_type: MetaType::meta_EB_SFX,              description: "Excitebots .sfx header"},
    MetaInfo{ meta_type: MetaType::meta_EB_SF0,              description: "assumed Excitebots .sf0 by extension"},
    MetaInfo{ meta_type: MetaType::meta_MTAF,                description: "Konami MTAF header"},
    MetaInfo{ meta_type: MetaType::meta_ALP,                 description: "High Voltage ALP header"},
    MetaInfo{ meta_type: MetaType::meta_WPD,                 description: "WPD 'DPW' header"},
    MetaInfo{ meta_type: MetaType::meta_MN_STR,              description: "Mini Ninjas 'STR' header"},
    MetaInfo{ meta_type: MetaType::meta_MSS,                 description: "Guerilla MCSS header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_HSF,             description: "Lowrider 'HSF' header"},
    MetaInfo{ meta_type: MetaType::meta_IVAG,                description: "Namco IVAG header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_2PFS,            description: "Konami 2PFS header"},
    MetaInfo{ meta_type: MetaType::meta_UBI_CKD,             description: "Ubisoft CKD RIFF header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_VBK,             description: "PS2 VBK Header"},
    MetaInfo{ meta_type: MetaType::meta_OTM,                 description: "Otomedius OTM Header"},
    MetaInfo{ meta_type: MetaType::meta_CSTM,                description: "Nintendo CSTM Header"},
    MetaInfo{ meta_type: MetaType::meta_FSTM,                description: "Nintendo FSTM Header"},
    MetaInfo{ meta_type: MetaType::meta_KT_WIIBGM,           description: "Koei Tecmo WiiBGM Header"},
    MetaInfo{ meta_type: MetaType::meta_KTSS,                description: "Koei Tecmo KTSS header"},
    MetaInfo{ meta_type: MetaType::meta_IDSP_NAMCO,          description: "Namco IDSP header"},
    MetaInfo{ meta_type: MetaType::meta_WIIU_BTSND,          description: "Nintendo Wii U Menu Boot Sound"},
    MetaInfo{ meta_type: MetaType::meta_MCA,                 description: "Capcom MCA header"},
    MetaInfo{ meta_type: MetaType::meta_ADX_MONSTER,         description: "Monster Games .ADX header"},
    MetaInfo{ meta_type: MetaType::meta_HCA,                 description: "CRI HCA header"},
    MetaInfo{ meta_type: MetaType::meta_SVAG_SNK,            description: "SNK SVAG header"},
    MetaInfo{ meta_type: MetaType::meta_PS2_VDS_VDM,         description: "Procyon Studio VDS/VDM header"},
    MetaInfo{ meta_type: MetaType::meta_FFMPEG,              description: "FFmpeg supported format"},
    MetaInfo{ meta_type: MetaType::meta_FFMPEG_faulty,       description: "FFmpeg supported format (check log)"},
    MetaInfo{ meta_type: MetaType::meta_CXS,                 description: "tri-Crescendo CXS header"},
    MetaInfo{ meta_type: MetaType::meta_AKB,                 description: "Square Enix AKB header"},
    MetaInfo{ meta_type: MetaType::meta_PASX,                description: "Premium Agency PASX header"},
    MetaInfo{ meta_type: MetaType::meta_XMA_RIFF,            description: "Microsoft XMA RIFF header"},
    MetaInfo{ meta_type: MetaType::meta_ASTB,                description: "Capcom ASTB header"},
    MetaInfo{ meta_type: MetaType::meta_WWISE_RIFF,          description: "Audiokinetic Wwise RIFF header"},
    MetaInfo{ meta_type: MetaType::meta_UBI_RAKI,            description: "Ubisoft RAKI header"},
    MetaInfo{ meta_type: MetaType::meta_SNDX,                description: "Sony SNDX header"},
    MetaInfo{ meta_type: MetaType::meta_OGL,                 description: "Shin'en OGL header"},
    MetaInfo{ meta_type: MetaType::meta_MC3,                 description: "Paradigm MC3 header"},
    MetaInfo{ meta_type: MetaType::meta_GHS,                 description: "Hexadrive GHS/S_P_STH header"},
    MetaInfo{ meta_type: MetaType::meta_AAC_TRIACE,          description: "tri-Ace AAC header"},
    MetaInfo{ meta_type: MetaType::meta_MTA2,                description: "Konami MTA2 header"},
    MetaInfo{ meta_type: MetaType::meta_XA_XA30,             description: "Reflections XA30 header"},
    MetaInfo{ meta_type: MetaType::meta_XA_04SW,             description: "Reflections 04SW header"},
    MetaInfo{ meta_type: MetaType::meta_TXTH,                description: "TXTH generic header"},
    MetaInfo{ meta_type: MetaType::meta_EA_BNK,              description: "Electronic Arts BNK header"},
    MetaInfo{ meta_type: MetaType::meta_SK_AUD,              description: "Silicon Knights AUD header"},
    MetaInfo{ meta_type: MetaType::meta_AHX,                 description: "CRI AHX header"},
    MetaInfo{ meta_type: MetaType::meta_STMA,                description: "Angel Studios/Rockstar San Diego STMA header"},
    MetaInfo{ meta_type: MetaType::meta_BINK,                description: "RAD Game Tools Bink header"},
    MetaInfo{ meta_type: MetaType::meta_EA_SNU,              description: "Electronic Arts SNU header"},
    MetaInfo{ meta_type: MetaType::meta_AWC,                 description: "Rockstar AWC header"},
    MetaInfo{ meta_type: MetaType::meta_OPUS,                description: "Nintendo Switch OPUS header"},
    MetaInfo{ meta_type: MetaType::meta_PC_AST,              description: "Capcom AST (PC) header"},
    MetaInfo{ meta_type: MetaType::meta_UBI_SB,              description: "Ubisoft SBx header"},
    MetaInfo{ meta_type: MetaType::meta_NAAC,                description: "Namco NAAC header"},
    MetaInfo{ meta_type: MetaType::meta_EZW,                 description: "EZ2DJ EZWAVE header"},
    MetaInfo{ meta_type: MetaType::meta_VXN,                 description: "Gameloft VXN header"},
    MetaInfo{ meta_type: MetaType::meta_EA_SNR_SNS,          description: "Electronic Arts SNR+SNS header"},
    MetaInfo{ meta_type: MetaType::meta_EA_SPS,              description: "Electronic Arts SPS header"},
    MetaInfo{ meta_type: MetaType::meta_VID1,                description: "Factor 5 VID1 header"},
    MetaInfo{ meta_type: MetaType::meta_PC_FLX,              description: "Ultima IX .FLX header"},
    MetaInfo{ meta_type: MetaType::meta_MOGG,                description: "Harmonix Music Systems MOGG Vorbis"},
    MetaInfo{ meta_type: MetaType::meta_OGG_VORBIS,          description: "Ogg Vorbis header"},
    MetaInfo{ meta_type: MetaType::meta_OGG_SFL,             description: "Ogg Vorbis header (SFPL looping)"},
    MetaInfo{ meta_type: MetaType::meta_OGG_KOVS,            description: "Ogg Vorbis header (KOVS)"},
    MetaInfo{ meta_type: MetaType::meta_OGG_encrypted,       description: "Ogg Vorbis header (encrypted)"},
    MetaInfo{ meta_type: MetaType::meta_KMA9,                description: "Koei Tecmo KMA9 header"},
    MetaInfo{ meta_type: MetaType::meta_XWC,                 description: "Starbreeze XWC header"},
    MetaInfo{ meta_type: MetaType::meta_SQEX_SAB,            description: "Square Enix SAB header"},
    MetaInfo{ meta_type: MetaType::meta_SQEX_MAB,            description: "Square Enix MAB header"},
    MetaInfo{ meta_type: MetaType::meta_WAF,                 description: "KID WAF header"},
    MetaInfo{ meta_type: MetaType::meta_WAVE,                description: "EngineBlack .WAVE header"},
    MetaInfo{ meta_type: MetaType::meta_WAVE_segmented,      description: "EngineBlack .WAVE header (segmented)"},
    MetaInfo{ meta_type: MetaType::meta_SMV,                 description: "extreme .SMV header"},
    MetaInfo{ meta_type: MetaType::meta_NXAP,                description: "Nex NXAP header"},
    MetaInfo{ meta_type: MetaType::meta_EA_WVE_AU00,         description: "Electronic Arts WVE (au00) header"},
    MetaInfo{ meta_type: MetaType::meta_EA_WVE_AD10,         description: "Electronic Arts WVE (Ad10) header"},
    MetaInfo{ meta_type: MetaType::meta_STHD,                description: "Dream Factory STHD header"},
    MetaInfo{ meta_type: MetaType::meta_MP4,                 description: "MP4/AAC header"},
    MetaInfo{ meta_type: MetaType::meta_PCM_SRE,             description: "Capcom .PCM+SRE header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_MCADPCM,         description: "Bethesda .mcadpcm header"},
    MetaInfo{ meta_type: MetaType::meta_UBI_LYN,             description: "Ubisoft LyN RIFF header"},
    MetaInfo{ meta_type: MetaType::meta_MSB_MSH,             description: "Sony MultiStream MSH+MSB header"},
    MetaInfo{ meta_type: MetaType::meta_TXTP,                description: "TXTP generic header"},
    MetaInfo{ meta_type: MetaType::meta_SMC_SMH,             description: "Genki SMC+SMH header"},
    MetaInfo{ meta_type: MetaType::meta_PPST,                description: "Parappa PPST header"},
    MetaInfo{ meta_type: MetaType::meta_SPS_N1,              description: "Nippon Ichi .SPS header"},
    MetaInfo{ meta_type: MetaType::meta_UBI_BAO,             description: "Ubisoft BAO header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_SWITCH_AUDIO,    description: "UE4 Switch Audio header"},
    MetaInfo{ meta_type: MetaType::meta_SADF,                description: "Procyon Studio SADF header"},
    MetaInfo{ meta_type: MetaType::meta_H4M,                 description: "Hudson HVQM4 header"},
    MetaInfo{ meta_type: MetaType::meta_ASF,                 description: "Argonaut ASF header"},
    MetaInfo{ meta_type: MetaType::meta_XMD,                 description: "Konami XMD header"},
    MetaInfo{ meta_type: MetaType::meta_CKS,                 description: "Cricket Audio CKS header"},
    MetaInfo{ meta_type: MetaType::meta_CKB,                 description: "Cricket Audio CKB header"},
    MetaInfo{ meta_type: MetaType::meta_WV6,                 description: "Gorilla Systems WV6 header"},
    MetaInfo{ meta_type: MetaType::meta_WAVEBATCH,           description: "Firebrand Games WBAT header"},
    MetaInfo{ meta_type: MetaType::meta_HD3_BD3,             description: "Sony HD3+BD3 header"},
    MetaInfo{ meta_type: MetaType::meta_BNK_SONY,            description: "Sony BNK header"},
    MetaInfo{ meta_type: MetaType::meta_SSCF,                description: "Square Enix SSCF header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_VAG,             description: ".VAG DSP header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_ITL,             description: ".ITL DSP header"},
    MetaInfo{ meta_type: MetaType::meta_A2M,                 description: "Artificial Mind & Movement A2M header"},
    MetaInfo{ meta_type: MetaType::meta_AHV,                 description: "Amuze AHV header"},
    MetaInfo{ meta_type: MetaType::meta_MSV,                 description: "Sony MultiStream MSV header"},
    MetaInfo{ meta_type: MetaType::meta_SDF,                 description: "Beyond Reality SDF header"},
    MetaInfo{ meta_type: MetaType::meta_SVG,                 description: "High Voltage SVG header"},
    MetaInfo{ meta_type: MetaType::meta_VIS,                 description: "Konami VIS header"},
    MetaInfo{ meta_type: MetaType::meta_VAI,                 description: "Asobo Studio .VAI header"},
    MetaInfo{ meta_type: MetaType::meta_AIF_ASOBO,           description: "Asobo Studio .AIF header"},
    MetaInfo{ meta_type: MetaType::meta_AO,                  description: "AlphaOgg .AO header"},
    MetaInfo{ meta_type: MetaType::meta_APC,                 description: "Cryo APC header"},
    MetaInfo{ meta_type: MetaType::meta_WV2,                 description: "Infogrames North America WAV2 header"},
    MetaInfo{ meta_type: MetaType::meta_XAU_KONAMI,          description: "Konami XAU header"},
    MetaInfo{ meta_type: MetaType::meta_DERF,                description: "Xilam DERF header"},
    MetaInfo{ meta_type: MetaType::meta_UTK,                 description: "Maxis UTK header"},
    MetaInfo{ meta_type: MetaType::meta_NXA,                 description: "Entergram NXA header"},
    MetaInfo{ meta_type: MetaType::meta_ADPCM_CAPCOM,        description: "Capcom .ADPCM header"},
    MetaInfo{ meta_type: MetaType::meta_UE4OPUS,             description: "Epic Games UE4OPUS header"},
    MetaInfo{ meta_type: MetaType::meta_XWMA,                description: "Microsoft XWMA RIFF header"},
    MetaInfo{ meta_type: MetaType::meta_VA3,                 description: "Konami VA3 header"},
    MetaInfo{ meta_type: MetaType::meta_XOPUS,               description: "Exient XOPUS header"},
    MetaInfo{ meta_type: MetaType::meta_VS_SQUARE,           description: "Square VS header"},
    MetaInfo{ meta_type: MetaType::meta_NWAV,                description: "Chunsoft NWAV header"},
    MetaInfo{ meta_type: MetaType::meta_XPCM,                description: "Circus XPCM header"},
    MetaInfo{ meta_type: MetaType::meta_MSF_TAMASOFT,        description: "Tama-Soft MSF header"},
    MetaInfo{ meta_type: MetaType::meta_XPS_DAT,             description: "From Software .XPS+DAT header"},
    MetaInfo{ meta_type: MetaType::meta_ZSND,                description: "Z-Axis ZSND header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_ADPY,            description: "AQUASTYLE ADPY header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_ADPX,            description: "AQUASTYLE ADPX header"},
    MetaInfo{ meta_type: MetaType::meta_OGG_OPUS,            description: "Ogg Opus header"},
    MetaInfo{ meta_type: MetaType::meta_IMC,                 description: "iNiS .IMC header"},
    MetaInfo{ meta_type: MetaType::meta_GIN,                 description: "Electronic Arts Gnsu header"},
    MetaInfo{ meta_type: MetaType::meta_DSF,                 description: "Ocean DSF header"},
    MetaInfo{ meta_type: MetaType::meta_208,                 description: "Ocean .208 header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_DS2,             description: "LucasArts .DS2 header"},
    MetaInfo{ meta_type: MetaType::meta_MUS_VC,              description: "Vicious Cycle .MUS header"},
    MetaInfo{ meta_type: MetaType::meta_STRM_ABYLIGHT,       description: "Abylight STRM header"},
    MetaInfo{ meta_type: MetaType::meta_MSF_KONAMI,          description: "Konami MSF header"},
    MetaInfo{ meta_type: MetaType::meta_XWMA_KONAMI,         description: "Konami XWMA header"},
    MetaInfo{ meta_type: MetaType::meta_9TAV,                description: "Konami 9TAV header"},
    MetaInfo{ meta_type: MetaType::meta_BWAV,                description: "Nintendo BWAV header"},
    MetaInfo{ meta_type: MetaType::meta_RAD,                 description: "Traveller's Tales .RAD header"},
    MetaInfo{ meta_type: MetaType::meta_SMACKER,             description: "RAD Game Tools SMACKER header"},
    MetaInfo{ meta_type: MetaType::meta_MZRT,                description: "id Software MZRT header"},
    MetaInfo{ meta_type: MetaType::meta_XAVS,                description: "Reflections XAVS header"},
    MetaInfo{ meta_type: MetaType::meta_PSF,                 description: "Pivotal PSF header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_ITL_i,           description: "Infernal .ITL DSP header"},
    MetaInfo{ meta_type: MetaType::meta_IMA,                 description: "Blitz Games .IMA header"},
    MetaInfo{ meta_type: MetaType::meta_XWV_VALVE,           description: "Valve XWV header"},
    MetaInfo{ meta_type: MetaType::meta_UBI_HX,              description: "Ubisoft HXx header"},
    MetaInfo{ meta_type: MetaType::meta_BMP_KONAMI,          description: "Konami BMP header"},
    MetaInfo{ meta_type: MetaType::meta_ISB,                 description: "Creative ISACT header"},
    MetaInfo{ meta_type: MetaType::meta_XSSB,                description: "Artoon XSSB header"},
    MetaInfo{ meta_type: MetaType::meta_XMA_UE3,             description: "Unreal Engine XMA header"},
    MetaInfo{ meta_type: MetaType::meta_FWSE,                description: "MT Framework FWSE header"},
    MetaInfo{ meta_type: MetaType::meta_FDA,                 description: "Relic FDA header"},
    MetaInfo{ meta_type: MetaType::meta_TGC,                 description: "Tiger Game.com .4 header"},
    MetaInfo{ meta_type: MetaType::meta_KWB,                 description: "Koei Tecmo WaveBank header"},
    MetaInfo{ meta_type: MetaType::meta_LRMD,                description: "Sony LRMD header"},
    MetaInfo{ meta_type: MetaType::meta_WWISE_FX,            description: "Audiokinetic Wwise FX header"},
    MetaInfo{ meta_type: MetaType::meta_DIVA,                description: "Sega DIVA header"},
    MetaInfo{ meta_type: MetaType::meta_IMUSE,               description: "LucasArts iMUSE header"},
    MetaInfo{ meta_type: MetaType::meta_KTSR,                description: "Koei Tecmo KTSR header"},
    MetaInfo{ meta_type: MetaType::meta_KAT,                 description: "Sega KAT header"},
    MetaInfo{ meta_type: MetaType::meta_PCM_SUCCESS,         description: "Success PCM header"},
    MetaInfo{ meta_type: MetaType::meta_ADP_KONAMI,          description: "Konami ADP header"},
    MetaInfo{ meta_type: MetaType::meta_SDRH,                description: "feelplus SDRH header"},
    MetaInfo{ meta_type: MetaType::meta_WADY,                description: "Marble WADY header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_SQEX,            description: "Square Enix DSP header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_WIIVOICE,        description: "Koei Tecmo WiiVoice header"},
    MetaInfo{ meta_type: MetaType::meta_SBK,                 description: "Team17 SBK header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_WIIADPCM,        description: "Exient WIIADPCM header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_CWAC,            description: "CRI CWAC header"},
    MetaInfo{ meta_type: MetaType::meta_COMPRESSWAVE,        description: "CompressWave .cwav header"},
    MetaInfo{ meta_type: MetaType::meta_KTAC,                description: "Koei Tecmo KTAC header"},
    MetaInfo{ meta_type: MetaType::meta_MJB_MJH,             description: "Sony MultiStream MJH+MJB header"},
    MetaInfo{ meta_type: MetaType::meta_BSNF,                description: "id Software BSNF header"},
    MetaInfo{ meta_type: MetaType::meta_TAC,                 description: "tri-Ace Codec header"},
    MetaInfo{ meta_type: MetaType::meta_IDSP_TOSE,           description: "TOSE .IDSP header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_KWA,             description: "Kuju London .KWA header"},
    MetaInfo{ meta_type: MetaType::meta_OGV_3RDEYE,          description: "3rdEye .OGV header"},
    MetaInfo{ meta_type: MetaType::meta_PIFF_TPCM,           description: "Tantalus PIFF TPCM header"},
    MetaInfo{ meta_type: MetaType::meta_WXD_WXH,             description: "Relic WXD+WXH header"},
    MetaInfo{ meta_type: MetaType::meta_BNK_RELIC,           description: "Relic BNK header"},
    MetaInfo{ meta_type: MetaType::meta_XSH_XSD_XSS,         description: "Treyarch XSH+XSD/XSS header"},
    MetaInfo{ meta_type: MetaType::meta_PSB,                 description: "M2 PSB header"},
    MetaInfo{ meta_type: MetaType::meta_LOPU_FB,             description: "French-Bread LOPU header"},
    MetaInfo{ meta_type: MetaType::meta_LPCM_FB,             description: "French-Bread LPCM header"},
    MetaInfo{ meta_type: MetaType::meta_WBK,                 description: "Treyarch WBK header"},
    MetaInfo{ meta_type: MetaType::meta_WBK_NSLB,            description: "Treyarch NSLB header"},
    MetaInfo{ meta_type: MetaType::meta_DSP_APEX,            description: "Koei Tecmo APEX header"},
    MetaInfo{ meta_type: MetaType::meta_MPEG,                description: "MPEG header"},
    MetaInfo{ meta_type: MetaType::meta_SSPF,                description: "Konami SSPF header"},
    MetaInfo{ meta_type: MetaType::meta_S3V,                 description: "Konami S3V header"},
    MetaInfo{ meta_type: MetaType::meta_ESF,                 description: "Eurocom ESF header"},
    MetaInfo{ meta_type: MetaType::meta_ADM,                 description: "Crankcase ADMx header"},
    MetaInfo{ meta_type: MetaType::meta_TT_AD,               description: "Traveller's Tales AUDIO_DATA header"},
    MetaInfo{ meta_type: MetaType::meta_SNDZ,                description: "Sony SNDZ header"},
    MetaInfo{ meta_type: MetaType::meta_VAB,                 description: "Sony VAB header"},
    MetaInfo{ meta_type: MetaType::meta_BIGRP,               description: "Inti Creates .BIGRP header"},
    MetaInfo{ meta_type: MetaType::meta_DIC1,                description: "Codemasters DIC1 header"},
    MetaInfo{ meta_type: MetaType::meta_AWD,                 description: "RenderWare Audio Wave Dictionary header"},
    MetaInfo{ meta_type: MetaType::meta_SQUEAKSTREAM,        description: "Torus SqueakStream header"},
    MetaInfo{ meta_type: MetaType::meta_SQUEAKSAMPLE,        description: "Torus SqueakSample header"},
    MetaInfo{ meta_type: MetaType::meta_SNDS,                description: "Sony SNDS header"},
];

pub fn get_coding_description(stream: VGMStream) -> &'static str {
    let mut description = "CANNOT DECODE";
    for i in 0..CODING_INFO_LIST.len() {
        if CODING_INFO_LIST[i].coding_type == stream.coding_type {
            description = CODING_INFO_LIST[i].description;
        }
    }

    description
}

pub fn get_layout_name(layout: LayoutType) -> &'static str {
    let mut description = "";
    for i in 0..LAYOUT_INFO_LIST.len() {
        if LAYOUT_INFO_LIST[i].layout_type == layout {
            description = LAYOUT_INFO_LIST[i].description;
        }
    }

    description
}

pub fn has_sublayouts(streams: &Vec<VGMStream>) -> bool {
    for stream in streams {
        if stream.layout_type == layout_segmented || stream.layout_type == layout_layered {
            return true;
        }
    }
    return false;
}

/* Makes a mixed description, considering a segments/layers can contain segments/layers infinitely, like:
 *
 * "(L3[S2L2]S3)"        "(S3[L2[S2S2]])"
 *  L3                    S3
 *    S2                    L2
 *      file                  S2
 *      file                    file
 *    file                      file
 *    L2                      file
 *      file                file
 *      file                file
 *
 * ("mixed" is added externally)
 */

/*
pub fn get_layout_mixed_description(stream: &VGMStream) -> String {
    let mut final_str = String::new();
    let mut count: i32 = 0;
    let mut streams: Vec<VGMStream> = Vec::new();
    if stream.layout_type == layout_layered {
        count = stream.layered_layout_data.as_ref().expect("layered layout should not be empty").layer_count;
        streams = stream.layered_layout_data.as_ref().expect("layered layout should not be empty").layers.clone();
        final_str = format!("L{}", count);
    }
    else if stream.layout_type == layout_segmented {
        count = stream.segmented_layout_data.as_ref().expect("segmented layout should not be empty").segment_count;
        streams = stream.segmented_layout_data.as_ref().expect("segmented layout should not be empty").segments.clone();
        final_str = format!("S{}", count);
    }

    if streams.is_empty() {
        return final_str;
    }

    if !has_sublayouts(&streams) {
        return final_str;
    }

    final_str.push_str("[");
    for i in 0..count {
        final_str += get_layout_mixed_description(&streams[i as usize]).as_str();
    }

    final_str.push_str("]");

    return final_str;
}

pub fn get_vgmstream_layout_description(stream: VGMStream) -> String {
    let mut mixed = false;
    let mut description = get_layout_name(stream.layout_type).to_string();
    if description == "" {
        description = "INCONCEIVABLE".to_string();
    }

    if stream.layout_type == layout_layered {
        mixed = has_sublayouts(&stream.layered_layout_data.as_ref().expect("layered layout should not be empty").layers);
        if !mixed {
            description = format!("{} ({} layers)", description, stream.layered_layout_data.as_ref().expect("layered layout should not be empty").layer_count);
        }
    }
    else if stream.layout_type == layout_segmented {
        mixed = has_sublayouts(&stream.segmented_layout_data.as_ref().expect("segmented layout should not be empty").segments);
        if !mixed {
            description = format!("{} ({} segments)", description, stream.segmented_layout_data.as_ref().expect("segmented layout should not be empty").segment_count);
        }
    }
    else {
        return description;
    }

    if mixed {
        let mixed_description = get_layout_mixed_description(&stream.as_ref());
        return format!("mixed {}", mixed_description);
    }

    return description;
}

pub fn get_vgmstream_meta_description(stream: VGMStream) -> String {
    let mut description = "THEY SHOULD HAVE SENT A POET".to_string();

    for i in 0..META_INFO_LIST.len() {
        if META_INFO_LIST[i].meta_type == stream.meta_type {
            description = META_INFO_LIST[i].description.to_string();
        }
    }

    return description;
}
*/
