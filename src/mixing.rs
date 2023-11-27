use crate::vgmstream::VGMStream;

// pub fn mixing_info(vgmstream: &mut VGMStream, p_input_channels: &mut i32, p_output_channels: &mut i32) {
//     let mut data = vgmstream.mixing_data;
//     int input_channels, output_channels;

//     if (!data) goto fail;

//     output_channels = data->output_channels;
//     if (data->output_channels > vgmstream->channels)
//         input_channels = data->output_channels;
//     else
//         input_channels = vgmstream->channels;

//     if (p_input_channels)  *p_input_channels = input_channels;
//     if (p_output_channels) *p_output_channels = output_channels;

//     //;VGM_LOG("MIX: channels %i, in=%i, out=%i, mix=%i\n", vgmstream->channels, input_channels, output_channels, data->mixing_channels);
//     return;
// fail:
//     if (p_input_channels)  *p_input_channels = vgmstream->channels;
//     if (p_output_channels) *p_output_channels = vgmstream->channels;
//     return;
// }