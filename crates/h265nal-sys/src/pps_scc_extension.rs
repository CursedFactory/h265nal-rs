use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawPpsSccExtensionFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PpsSccExtensionFields {
    pub pps_curr_pic_ref_enabled_flag: u32,
    pub residual_adaptive_colour_transform_enabled_flag: u32,
    pub pps_palette_predictor_initializer_present_flag: u32,
    pub pps_palette_predictor_initializers_size: usize,
}

pub fn pps_scc_extension_parse(data: &[u8]) -> Result<PpsSccExtensionFields, Error> {
    let mut raw = RawPpsSccExtensionFields {
        pps_curr_pic_ref_enabled_flag: 0,
        residual_adaptive_colour_transform_enabled_flag: 0,
        pps_palette_predictor_initializer_present_flag: 0,
        pps_palette_predictor_initializers_size: 0,
    };
    let status =
        unsafe { ffi::h265nal_pps_scc_extension_parse(data.as_ptr(), data.len(), &mut raw) };
    status_to_result(status)?;

    Ok(PpsSccExtensionFields {
        pps_curr_pic_ref_enabled_flag: raw.pps_curr_pic_ref_enabled_flag,
        residual_adaptive_colour_transform_enabled_flag: raw
            .residual_adaptive_colour_transform_enabled_flag,
        pps_palette_predictor_initializer_present_flag: raw
            .pps_palette_predictor_initializer_present_flag,
        pps_palette_predictor_initializers_size: raw.pps_palette_predictor_initializers_size,
    })
}
