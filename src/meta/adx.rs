use crate::streamfile::*;
use crate::vgmstream::*;
use libm::*;

pub fn init_vgmstream_adx(sf: &mut Streamfile) -> Option<VGMStream> {
    return init_vgmstream_adx_subkey(sf, 0);
}

fn init_vgmstream_adx_subkey(sf: &mut Streamfile, subkey: u16) -> Option<VGMStream> {
    let mut vgmstream = VGMStream::default();
    
    if read_u16be(sf, 0x0) != 0x8000 {
        return None;
    }

    if !check_extensions(sf, vec!["adx", "adp"]) {
        return None;
    }

    let start_offset = read_u16be(sf, 0x02) + 0x04;
    if read_u16be(sf, (start_offset - 0x06).into()) != 0x2863 ||     /* "(c" */
        read_u32be(sf, (start_offset - 0x04).into()) != 0x29435249   /* ")CRI" */
    {
        return None;
    }
    let encoding_type = read_u8(sf, 0x04);
    let coding_type: CodingType;
    match encoding_type {
        0x02 => {
            coding_type = CodingType::coding_CRI_ADX_fixed;
        }
        0x03 => {
            coding_type = CodingType::coding_CRI_ADX;
        }
        0x04 => {
            coding_type = CodingType::coding_CRI_ADX_exp;
        }
        _ => { /* 0x10 is AHX for DC, 0x11 is AHX */
            return None;
        }
    }

    /* ADX encoders can't set this value, but is honored by ADXPlay if changed and multiple of 0x12,
     * though output is unusual and may not be fully supported (works in mono so not an interleave)
     * Later versions of the decode just use constant 0x12 ignoring it, though. */

    let frame_size = read_u8(sf, 0x05) as isize;

    if read_u8(sf, 0x06) != 4 { /* bits per sample */
        return None;
    }

    let channels = read_u8(sf, 0x07);
    let sample_rate = read_s32be(sf, 0x08);
    let num_samples = read_s32be(sf, 0x0c);
    let cutoff = read_u16be(sf, 0x10); /* high-pass cutoff frequency, always 500 */
    let version = read_u16be(sf, 0x12); /* version + revision, originally read as separate */

    // TODO: ADX encryption

    // if version == 0x0408 {

    //     if (!find_adx_key(sf, 8, &xor_start, &xor_mult, &xor_add, 0)) {
    //         vgm_logi("ADX: decryption keystring not found\n");
    //     }
    //     coding_type = CodingType::coding_CRI_ADX_enc_8;
    //     version = 0x0400;
    // }
    // else if version == 0x0409 {
    //     if (!find_adx_key(sf, 9, &xor_start, &xor_mult, &xor_add, subkey)) {
    //         vgm_logi("ADX: decryption keycode not found\n");
    //     }
    //     coding_type = CodingType::coding_CRI_ADX_enc_9;
    //     version = 0x0400;
    // }

    let header_type: MetaType;

    let mut loop_flag = false;
    let mut loop_start_sample = 0;
    let mut loop_end_sample = 0;
    let mut hist_offset: usize = 0;

    if version == 0x0300 {
        let base_size: usize = 0x14;
        let loops_size: usize = 0x18;

        header_type = MetaType::meta_ADX_03;

        if start_offset - 0x06 >= (base_size + loops_size) as u16 { /* enough space for loop info? */
            let loops_offset = base_size;

            /* 0x00 (2): initial loop padding (the encoder adds a few blank samples so loop start is block-aligned; max 31)
             *  ex. loop_start=12: enc_start=32, padding=20 (32-20=12); loop_start=35: enc_start=64, padding=29 (64-29=35)
             * 0x02 (2): loop flag? (always 1) */
            loop_flag           = read_s32be(sf, loops_offset+0x04) != 0; /* loop count + loop type? (always 1) */
            loop_start_sample   = read_s32be(sf, loops_offset+0x08);
            loop_end_sample     = read_s32be(sf, loops_offset+0x10);
        }
    }
    else if version == 0x0400 {  /* common */
        let base_size = 0x18;
        let mut hist_size = 0;
        let mut ainf_size = 0;
        let loops_size = 0x18;
        let ainf_offset: usize;

        header_type = MetaType::meta_ADX_04;

        hist_offset = base_size; /* always present but often blank */
        // hist_size = (channels > 1 ? 0x04 * channels : 0x04 + 0x04); /* min is 0x8, even in 1ch files */
        if channels > 1 {
            hist_size = 0x04 * channels;
        }
        else {
            hist_size = 0x04 + 0x04;
        }

        ainf_offset = base_size + hist_size as usize + 0x04; /* not seen with >2ch though */
        if is_id32be(sf, ainf_offset+0x00, "AINF") {
            ainf_size = read_u32be(sf, ainf_offset+0x04);
        }

        if (start_offset as u32 - ainf_size - 0x06) as usize >= (hist_offset + hist_size as usize + loops_size) as usize {  /* enough space for loop info? */
            let loops_offset = base_size + hist_size as usize;

            /* 0x00 (2): initial loop padding (the encoder adds a few blank samples so loop start is block-aligned; max 31)
             *  ex. loop_start=12: enc_start=32, padding=20 (32-20=12); loop_start=35: enc_start=64, padding=29 (64-29=35)
             * 0x02 (2): loop flag? (always 1) */
            loop_flag           = read_s32be(sf, loops_offset+0x04) != 0; /* loop count + loop type? (always 1) */
            loop_start_sample   = read_s32be(sf, loops_offset+0x08);
            loop_end_sample     = read_s32be(sf, loops_offset+0x10);
        }

        /* AINF header info (may be inserted by CRI's tools but is rarely used)
         *  Can also start right after the loop points (base_size + hist_size + loops_size)
         * 0x00 (4): "AINF"
         * 0x04 (4): size
         * 0x08 (10): str_id
         * 0x18 (2): volume (0=base/max?, negative=reduce)
         * 0x1c (2): pan l
         * 0x1e (2): pan r (0=base, max +-128) */

        /* CINF header info (very rare, found after loops) [Sakura Taisen 3 (PS2)]
         * 0x00 (4): "CINF"
         * 0x04 (4): size
         * 0x08 (4): "ASO ", unknown
         * 0x28 (4): "SND ", unknown
         * 0x48 (-): file name, null terminated
         */
    }
    else if version == 0x0500 {  /* found in some SFD: Buggy Heat, appears to have no loop */
        header_type = MetaType::meta_ADX_05;
    }
    else { /* not a known/supported version signature */
        return None;
    }

    vgmstream.sample_rate = sample_rate;
    vgmstream.num_samples = num_samples;
    vgmstream.loop_flag = loop_flag;
    vgmstream.loop_start_sample = loop_start_sample;
    vgmstream.loop_end_sample = loop_end_sample;

    vgmstream.codec_config = version as i32;
    vgmstream.coding_type = coding_type;
    vgmstream.layout_type = LayoutType::layout_interleave;
    vgmstream.interleave_block_size = frame_size;
    vgmstream.meta_type = header_type;

    vgmstream.channels = channels as i32;
    vgmstream.ch = vec![VGMStreamChannel::default(); channels as usize];
    
    if coding_type == CodingType::coding_CRI_ADX_fixed {
        /* standard XA coefs * (2<<11) */
        // TODO: this feels... wrong...
        #[allow(overflowing_literals)]
        for ch in &mut vgmstream.ch {
            ch.adpcm_coef[0] = 0x0000;
            ch.adpcm_coef[1] = 0x0000;
            ch.adpcm_coef[2] = 0x0F00;
            ch.adpcm_coef[3] = 0x0000;
            ch.adpcm_coef[4] = 0x1CC0;
            ch.adpcm_coef[5] = 0xF300;
            ch.adpcm_coef[6] = 0x1880;
            ch.adpcm_coef[7] = 0xF240;
        }
    }
    else {
        /* coefs from cutoff frequency (some info from decomps, uses floats but no diffs if using doubles due to rounding) */
        let x: f32 = cutoff as f32;
        let y: f32 = sample_rate as f32;
        let z = cosf(2.0 * std::f32::consts::PI * x / y); /* 2.0 * M_PI: 6.28318548202515f (decomp) */

        let a = std::f32::consts::SQRT_2 - z;    /* M_SQRT2: 1.41421353816986f (decomp) */
        let b = std::f32::consts::SQRT_2 - 1.0;  /* M_SQRT2 - 1: 0.414213538169861f (decomp) */
        let c = (a - sqrtf((a + b) * (a - b))) / b; /* this seems calculated with a custom algorithm */

        let coef1: i16 = (c * 8192.0) as i16;
        let coef2: i16 = (c * c * -4096.0) as i16;

        for ch in &mut vgmstream.ch {
            ch.adpcm_coef[0] = coef1;
            ch.adpcm_coef[1] = coef2;
        }
    }


    for i in 0..vgmstream.ch.len() {
        if hist_offset != 0 {
            vgmstream.ch[i].adpcm_history1_32 = read_s16be(sf, hist_offset + i*4 + 0x00) as i32;
            vgmstream.ch[i].adpcm_history2_32 = read_s16be(sf, hist_offset + i*4 + 0x02) as i32;
        }

        // if coding_type == CodingType::coding_CRI_ADX_enc_8 || coding_type == CodingType::coding_CRI_ADX_enc_9 {
        //     vgmstream.ch[i].adx_channels = channels as i32;
        //     vgmstream.ch[i].adx_xor = xor_start;
        //     vgmstream.ch[i].adx_mult = xor_mult;
        //     vgmstream.ch[i].adx_add = xor_add;

        //     for j in 0..i {
        //         adx_next_key(&vgmstream.ch[i]);
        //     }
        // }
    }

    vgmstream.open_stream(sf, start_offset as isize);

    return Some(vgmstream);
}
