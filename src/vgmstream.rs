pub use crate::constants::*;
use crate::meta::adx;
use crate::meta::wwise;
pub use crate::streamfile::*;
pub use crate::vgmstream_types::*;

type InitVGMStream = fn(streamfile: &mut Streamfile) -> Option<VGMStream>;

pub const INIT_VGMSTREAM_FUNCTIONS: [InitVGMStream; 2] = [
    adx::init_vgmstream_adx,
    wwise::init_vgmstream_wwise,
];

impl VGMStream {
    pub fn init(filename: String) -> Self {
        println!("init({})", filename);
        let mut stream: VGMStream = VGMStream::default();
        // println!("{:?}", stream);
        let sf = Streamfile::open_stdio(filename);
        // TODO: check if streamfile is valid (options, duh)
        if sf.is_some() {
            println!("{:?}", sf);
            stream = Self::init_from_streamfile(&mut sf.unwrap()).unwrap();
            // sf.close();
        }
        return stream;
    }

    pub fn init_from_streamfile(sf: &mut Streamfile) -> Option<Self> {
        return Self::init_internal(sf);
    }

    fn init_internal(sf: &mut Streamfile) -> Option<Self> {
        for func in INIT_VGMSTREAM_FUNCTIONS {
            // let init_function = INIT_VGMSTREAM_FUNCTIONS[i];
            let stream = func(sf);
            if stream.is_none() {
                continue;
            }

            let mut stream = stream.unwrap();
            if stream.num_samples <= 0 || stream.num_samples >= VGMSTREAM_MAX_NUM_SAMPLES {
                println!("VGMSTREAM: wrong num samples {}", stream.num_samples);
                stream.close();
                continue;
            }

            if stream.sample_rate < VGMSTREAM_MIN_SAMPLE_RATE
                || stream.sample_rate > VGMSTREAM_MAX_SAMPLE_RATE
            {
                println!("VGMSTREAM: wrong sample rate {}", stream.sample_rate);
                stream.close();
                continue;
            }

            // TODO: loops

            if stream.channels == 1 && stream.allow_dual_stereo {
                // TODO: dual stereo
            }

            // TODO: loops (set to 0)

            // TODO: ffmpeg?

            if stream.channel_layout > 0 {
                let mut count = 0;
                for ch in 0..32 {
                    let bit = (stream.channel_layout >> ch) & 1;
                    if ch > 17 && bit == 1 {
                        println!(
                            "VGMSTREAM: wrong bit {} in channel_layout {:x}",
                            ch, stream.channel_layout
                        );
                        stream.channel_layout = 0;
                        break;
                    }
                    count += bit;
                }

                if count > stream.channels.try_into().unwrap() {
                    println!(
                        "VGMSTREAM: wrong totals {} in channel_layout {:x}",
                        count, stream.channel_layout
                    );
                    stream.channel_layout = 0;
                }
            }

            if stream.num_streams < 0 || stream.num_streams > VGMSTREAM_MAX_SUBSONGS {
                println!("VGMSTREAM: wrong num_streams (ns={})\n", stream.num_streams);
                stream.close();
                continue;
            }

            if stream.stream_index == 0 {
                stream.stream_index = sf.stream_index;
            }

            // stream.setup();

            return Some(stream);
        }

        return None;
    }

    // fn setup(&mut self) {
    //     self.start_ch = self.ch.clone();
    //     self.start_vgmstream = Some(Box::new(self.clone()));
    // }

    pub fn close(&mut self) {
        // TODO: close everything
    }

    pub fn open_stream(&mut self, sf: &mut Streamfile, start_offset: isize) -> bool {
        return self.open_stream_bf(sf, start_offset, false);
    }

    fn open_stream_bf(
        &mut self,
        sf: &mut Streamfile,
        start_offset: isize,
        force_multibuffer: bool,
    ) -> bool {
        if self.coding_type == CodingType::coding_SILENCE {
            return true;
        }

        if self.layout_type == LayoutType::layout_segmented
            || self.layout_type == LayoutType::layout_layered
        {
            return true;
        }

        if self.coding_type == CodingType::coding_NWA
            || self.coding_type == CodingType::coding_ACM
            || self.coding_type == CodingType::coding_CRI_HCA
        {
            return true;
        }

        if self.coding_type == CodingType::coding_OGG_VORBIS {
            return true;
        }

        if self.coding_type == CodingType::coding_FFmpeg {
            return true;
        }

        if (self.coding_type == CodingType::coding_CRI_ADX
            || self.coding_type == CodingType::coding_CRI_ADX_enc_8
            || self.coding_type == CodingType::coding_CRI_ADX_enc_9
            || self.coding_type == CodingType::coding_CRI_ADX_exp
            || self.coding_type == CodingType::coding_CRI_ADX_fixed)
            && (self.interleave_block_size == 0 || self.interleave_block_size > 0x12)
        {
            println!(
                "VGMSTREAM: ADX decoder with wrong frame size {:x}",
                self.interleave_block_size
            );
            return false;
        }

        if (self.coding_type == CodingType::coding_MSADPCM
            || self.coding_type == CodingType::coding_MSADPCM_ck
            || self.coding_type == CodingType::coding_MSADPCM_int
            || self.coding_type == CodingType::coding_MS_IMA
            || self.coding_type == CodingType::coding_MS_IMA_mono
            || self.coding_type == CodingType::coding_PSX_cfg
            || self.coding_type == CodingType::coding_PSX_pivotal)
            && self.frame_size == 0
        {
            self.frame_size = self.interleave_block_size;
        }

        if (self.coding_type == CodingType::coding_PSX_cfg
            || self.coding_type == CodingType::coding_PSX_pivotal)
            && (self.frame_size == 0 || self.frame_size > 0x50)
        {
            println!(
                "VGMSTREAM: PSX-cfg decoder with wrong frame size {:x}",
                self.frame_size
            );
            return true;
        }

        if (self.coding_type == CodingType::coding_MSADPCM
            || self.coding_type == CodingType::coding_MSADPCM_ck
            || self.coding_type == CodingType::coding_MSADPCM_int)
            && (self.frame_size == 0 || self.frame_size > MSADPCM_MAX_BLOCK_SIZE)
        {
            println!(
                "VGMSTREAM: MSADPCM decoder with wrong frame size {:x}",
                self.frame_size
            );
            return false;
        }

        /* big interleaved values for non-interleaved data may result in incorrect behavior,
         * quick fix for now since layouts are finicky, with 'interleave' left for meta info
         * (certain layouts+codecs combos results in funny output too, should rework the whole thing) */
        if self.layout_type == LayoutType::layout_interleave
            && self.channels == 1
            && self.interleave_block_size > 0
        {
            /* main codecs that use arbitrary interleaves but could happen for others too */
            match self.coding_type {
                CodingType::coding_NGC_DSP
                | CodingType::coding_NGC_DSP_subint
                | CodingType::coding_PSX
                | CodingType::coding_PSX_badflags => {
                    self.interleave_block_size = 0;
                }
                _ => {}
            }
        }

        let mut use_streamfile_per_channel = false;
        let mut use_same_offset_per_channel = false;
        let mut is_stereo_codec = false;

        /* if interleave is big enough keep a buffer per channel */
        if self.interleave_block_size  as usize * self.channels as usize >= STREAMFILE_DEFAULT_BUFFER_SIZE {
            use_streamfile_per_channel = true;
        }

        /* if blocked layout (implicit) use multiple streamfiles; using only one leads to
         * lots of buffer-trashing, with all the jumping around in the block layout
         * (this increases total of data read but still seems faster) */
        if self.layout_type != LayoutType::layout_none
            && self.layout_type != LayoutType::layout_interleave
        {
            use_streamfile_per_channel = true;
        }

        /* for hard-to-detect fixed offsets or full interleave */
        if force_multibuffer {
            use_streamfile_per_channel = true;
        }

        /* for mono or codecs like IMA (XBOX, MS IMA, MS ADPCM) where channels work with the same bytes */
        if self.layout_type == LayoutType::layout_none {
            use_same_offset_per_channel = true;
        }

        /* stereo codecs interleave in 2ch pairs (interleave size should still be: full_block_size / channels) */
        if self.layout_type == LayoutType::layout_interleave
            && (self.coding_type == CodingType::coding_XBOX_IMA
                || self.coding_type == CodingType::coding_MTAF)
        {
            is_stereo_codec = true;
        }

        if start_offset < 0 {
            println!("VGMSTREAM: buggy code (wrong start_offset)");
            return false;
        }

        if !use_streamfile_per_channel {
            // TODO: open???
        }

        for ch in 0..self.channels {
            let mut offset = 0;
            if use_same_offset_per_channel {
                offset = start_offset;
            } else if is_stereo_codec {
                let ch_mod = if ch & 1 == 1 { ch - 1 } else { ch }; /* adjust odd channels (ch 0,1,2,3,4,5 > ch 0,0,2,2,4,4) */
                offset = start_offset + self.interleave_block_size * ch_mod as isize;
            } else if self.interleave_first_block_size != 0 {
                offset = start_offset
                    + (self.interleave_first_block_size + self.interleave_first_skip) * ch as isize;
            } else {
                offset = start_offset + self.interleave_block_size * ch as isize;
            }

            if use_streamfile_per_channel {
                // TODO: open???
            }

            self.ch[ch as usize].streamfile = Some(sf.clone());
            self.ch[ch as usize].channel_start_offset = offset;
            self.ch[ch as usize].offset = offset;
        }

        // TODO: block updates
        // self.block_update(start_offset);

        if self.coding_type == CodingType::coding_EA_MT {
            // flush_ea_mt(vgmstream);
        }

        return true;
    }

    // fn block_update(&mut self, block_offset: isize) {
    //     match self.layout_type {
    //         LayoutType::layout_blocked_ast => {
    //             self.block_update_ast(block_offset);
    //         }
    //         LayoutType::layout_blocked_mxch => {
    //             self.block_update_mxch(block_offset);
    //         }
    //         LayoutType::layout_blocked_halpst => {
    //             self.block_update_halpst(block_offset);
    //         }
    //         LayoutType::layout_blocked_xa => {
    //             self.block_update_xa(block_offset);
    //         }
    //         LayoutType::layout_blocked_ea_schl => {
    //             self.block_update_ea_schl(block_offset);
    //         }
    //         LayoutType::layout_blocked_ea_1snh => {
    //             self.block_update_ea_1snh(block_offset);
    //         }
    //         LayoutType::layout_blocked_caf => {
    //             self.block_update_caf(block_offset);
    //         }
    //         LayoutType::layout_blocked_wsi => {
    //             self.block_update_wsi(block_offset);
    //         }
    //         LayoutType::layout_blocked_str_snds => {
    //             self.block_update_str_snds(block_offset);
    //         }
    //         LayoutType::layout_blocked_ws_aud => {
    //             self.block_update_ws_aud(block_offset);
    //         }
    //         LayoutType::layout_blocked_dec => {
    //             self.block_update_dec(block_offset);
    //         }
    //         LayoutType::layout_blocked_mul => {
    //             self.block_update_mul(block_offset);
    //         }
    //         LayoutType::layout_blocked_gsb => {
    //             self.block_update_gsb(block_offset);
    //         }
    //         LayoutType::layout_blocked_vs => {
    //             self.block_update_vs(block_offset);
    //         }
    //         LayoutType::layout_blocked_xvas => {
    //             self.block_update_xvas(block_offset);
    //         }
    //         LayoutType::layout_blocked_thp => {
    //             self.block_update_thp(block_offset);
    //         }
    //         LayoutType::layout_blocked_filp => {
    //             self.block_update_filp(block_offset);
    //         }
    //         LayoutType::layout_blocked_ivaud => {
    //             self.block_update_ivaud(block_offset);
    //         }
    //         LayoutType::layout_blocked_ea_swvr => {
    //             self.block_update_ea_swvr(block_offset);
    //         }
    //         LayoutType::layout_blocked_adm => {
    //             self.block_update_adm(block_offset);
    //         }
    //         LayoutType::layout_blocked_ps2_iab => {
    //             self.block_update_ps2_iab(block_offset);
    //         }
    //         LayoutType::layout_blocked_vs_str => {
    //             self.block_update_vs_str(block_offset);
    //         }
    //         LayoutType::layout_blocked_rws => {
    //             self.block_update_rws(block_offset);
    //         }
    //         LayoutType::layout_blocked_hwas => {
    //             self.block_update_hwas(block_offset);
    //         }
    //         LayoutType::layout_blocked_ea_sns => {
    //             self.block_update_ea_sns(block_offset);
    //         }
    //         LayoutType::layout_blocked_awc => {
    //             self.block_update_awc(block_offset);
    //         }
    //         LayoutType::layout_blocked_vgs => {
    //             self.block_update_vgs(block_offset);
    //         }
    //         LayoutType::layout_blocked_xwav => {
    //             self.block_update_xwav(block_offset);
    //         }
    //         LayoutType::layout_blocked_xvag_subsong => {
    //             self.block_update_xvag_subsong(block_offset);
    //         }
    //         LayoutType::layout_blocked_ea_wve_au00 => {
    //             self.block_update_ea_wve_au00(block_offset);
    //         }
    //         LayoutType::layout_blocked_ea_wve_ad10 => {
    //             self.block_update_ea_wve_ad10(block_offset);
    //         }
    //         LayoutType::layout_blocked_sthd => {
    //             self.block_update_sthd(block_offset);
    //         }
    //         LayoutType::layout_blocked_h4m => {
    //             self.block_update_h4m(block_offset);
    //         }
    //         LayoutType::layout_blocked_xa_aiff => {
    //             self.block_update_xa_aiff(block_offset);
    //         }
    //         LayoutType::layout_blocked_vs_square => {
    //             self.block_update_vs_square(block_offset);
    //         }
    //         LayoutType::layout_blocked_vid1 => {
    //             self.block_update_vid1(block_offset);
    //         }
    //         LayoutType::layout_blocked_ubi_sce => {
    //             self.block_update_ubi_sce(block_offset);
    //         }
    //         LayoutType::layout_blocked_tt_ad => {
    //             self.block_update_tt_ad(block_offset);
    //         }
    //         _ => {} /* not a blocked layout */
    //     }
    // }

    pub fn get_samples(&self) -> i32 {
        if !self.config_enabled || !self.config.config_set {
            return self.num_samples;
        }
        return self.pstate.play_duration;
    }

    pub fn decode_get_samples_to_do(&self, samples_this_block: i32, samples_per_frame: i32) -> i32 {
        let samples_left_this_block = samples_this_block - self.samples_into_block as i32;
        let mut samples_to_do = samples_left_this_block; /* by default decodes all samples left */

        /* fun loopy crap, why did I think this would be any simpler? */

        // TODO: handle looping

        // if vgmstream.loop_flag {
        //     int samples_after_decode = vgmstream->current_sample + samples_left_this_block;

        //     /* are we going to hit the loop end during this block? */
        //     if (samples_after_decode > vgmstream->loop_end_sample) {
        //         /* only do samples up to loop end */
        //         samples_to_do = vgmstream->loop_end_sample - vgmstream->current_sample;
        //     }

        //     /* are we going to hit the loop start during this block? (first time only) */
        //     if (samples_after_decode > vgmstream->loop_start_sample && !vgmstream->hit_loop) {
        //         /* only do samples up to loop start */
        //         samples_to_do = vgmstream->loop_start_sample - vgmstream->current_sample;
        //     }
        // }

        /* if it's a framed encoding don't do more than one frame */
        if samples_per_frame > 1 && (self.samples_into_block as i32 % samples_per_frame) + samples_to_do > samples_per_frame {
            samples_to_do = samples_per_frame - (self.samples_into_block as i32 % samples_per_frame);
        }

        return samples_to_do;
    }
}
