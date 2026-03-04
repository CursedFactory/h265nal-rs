use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawSpsSccExtensionFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpsSccExtensionFields {
    pub sps_curr_pic_ref_enabled_flag: u32,
    pub palette_mode_enabled_flag: u32,
    pub palette_max_size: u32,
    pub delta_palette_max_predictor_size: u32,
    pub sps_palette_predictor_initializers_present_flag: u32,
    pub sps_num_palette_predictor_initializers_minus1: u32,
    pub sps_palette_predictor_initializers_size: usize,
    pub motion_vector_resolution_control_idc: u32,
    pub intra_boundary_filtering_disabled_flag: u32,
}

pub fn sps_scc_extension_parse(
    data: &[u8],
    chroma_format_idc: u32,
    bit_depth_luma_minus8: u32,
    bit_depth_chroma_minus8: u32,
) -> Result<SpsSccExtensionFields, Error> {
    let mut raw = RawSpsSccExtensionFields {
        sps_curr_pic_ref_enabled_flag: 0,
        palette_mode_enabled_flag: 0,
        palette_max_size: 0,
        delta_palette_max_predictor_size: 0,
        sps_palette_predictor_initializers_present_flag: 0,
        sps_num_palette_predictor_initializers_minus1: 0,
        sps_palette_predictor_initializers_size: 0,
        motion_vector_resolution_control_idc: 0,
        intra_boundary_filtering_disabled_flag: 0,
    };
    let status = unsafe {
        ffi::h265nal_sps_scc_extension_parse(
            data.as_ptr(),
            data.len(),
            chroma_format_idc,
            bit_depth_luma_minus8,
            bit_depth_chroma_minus8,
            &mut raw,
        )
    };
    status_to_result(status)?;

    Ok(SpsSccExtensionFields {
        sps_curr_pic_ref_enabled_flag: raw.sps_curr_pic_ref_enabled_flag,
        palette_mode_enabled_flag: raw.palette_mode_enabled_flag,
        palette_max_size: raw.palette_max_size,
        delta_palette_max_predictor_size: raw.delta_palette_max_predictor_size,
        sps_palette_predictor_initializers_present_flag: raw
            .sps_palette_predictor_initializers_present_flag,
        sps_num_palette_predictor_initializers_minus1: raw
            .sps_num_palette_predictor_initializers_minus1,
        sps_palette_predictor_initializers_size: raw.sps_palette_predictor_initializers_size,
        motion_vector_resolution_control_idc: raw.motion_vector_resolution_control_idc,
        intra_boundary_filtering_disabled_flag: raw.intra_boundary_filtering_disabled_flag,
    })
}
