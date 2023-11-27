use crate::vgmstream::{VGMStream, LayoutType};
use crate::layout::blocked::render_vgmstream_blocked;
use crate::layout::interleave::render_vgmstream_interleave;
use crate::layout::flat::render_vgmstream_flat;
use crate::layout::segmented::render_vgmstream_segmented;

pub fn render_vgmstream(buffer: &mut Vec<i16>, sample_count: i32, vgmstream: &mut VGMStream) -> i32 {
    let samples_to_do = sample_count;
    // let mut buf: Vec<i16> = vec![0; samples_to_do as usize * vgmstream.channels as usize * 2];
    if !vgmstream.config_enabled {
        render_layout(buffer, samples_to_do, vgmstream);
        // buf = mix_vgmstream(samples_to_do, vgmstream);
    }
    
    // TODO: config
    
    return samples_to_do;
}

pub fn render_layout(buffer: &mut Vec<i16>, sample_count: i32, vgmstream: &mut VGMStream) {
    /* current_sample goes between loop points (if looped) or up to max samples,
     * must detect beyond that decoders would encounter garbage data */

    /* not ">=" to allow layouts to loop in some cases when == happens */
    if vgmstream.current_sample > vgmstream.num_samples as isize{
        // memset(buf, 0, sample_count * sizeof(sample_t) * vgmstream.channels);
        for i in 0..sample_count * vgmstream.channels {
            buffer[i as usize] = 0;
        }
        return;
    }

    match vgmstream.layout_type {
        LayoutType::layout_interleave => {
            render_vgmstream_interleave(buffer, sample_count, vgmstream);
        }
        LayoutType::layout_none => {
            render_vgmstream_flat(buffer, sample_count, vgmstream);
        }
        LayoutType::layout_blocked_mxch |
        LayoutType::layout_blocked_ast |
        LayoutType::layout_blocked_halpst |
        LayoutType::layout_blocked_xa |
        LayoutType::layout_blocked_ea_schl |
        LayoutType::layout_blocked_ea_1snh |
        LayoutType::layout_blocked_caf |
        LayoutType::layout_blocked_wsi |
        LayoutType::layout_blocked_str_snds |
        LayoutType::layout_blocked_ws_aud |
        LayoutType::layout_blocked_dec |
        LayoutType::layout_blocked_vs |
        LayoutType::layout_blocked_mul |
        LayoutType::layout_blocked_gsb |
        LayoutType::layout_blocked_xvas |
        LayoutType::layout_blocked_thp |
        LayoutType::layout_blocked_filp |
        LayoutType::layout_blocked_ivaud |
        LayoutType::layout_blocked_ea_swvr |
        LayoutType::layout_blocked_adm |
        LayoutType::layout_blocked_ps2_iab |
        LayoutType::layout_blocked_vs_str |
        LayoutType::layout_blocked_rws |
        LayoutType::layout_blocked_hwas |
        LayoutType::layout_blocked_ea_sns |
        LayoutType::layout_blocked_awc |
        LayoutType::layout_blocked_vgs |
        LayoutType::layout_blocked_xwav |
        LayoutType::layout_blocked_xvag_subsong |
        LayoutType::layout_blocked_ea_wve_au00 |
        LayoutType::layout_blocked_ea_wve_ad10 |
        LayoutType::layout_blocked_sthd |
        LayoutType::layout_blocked_h4m |
        LayoutType::layout_blocked_xa_aiff |
        LayoutType::layout_blocked_vs_square |
        LayoutType::layout_blocked_vid1 |
        LayoutType::layout_blocked_ubi_sce |
        LayoutType::layout_blocked_tt_ad => {
            // render_vgmstream_blocked(buffer, sample_count, vgmstream);
        }
        LayoutType::layout_segmented => {
            // render_vgmstream_segmented(buffer, sample_count,vgmstream);
        }
        // LayoutType::layout_layered => {
        //     buf = render_vgmstream_layered(sample_count, vgmstream);
        // }
        _ => {}
    }
}