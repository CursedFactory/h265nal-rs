use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawPpsMultilayerExtensionFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PpsMultilayerExtensionFields {
    pub poc_reset_info_present_flag: u32,
    pub pps_infer_scaling_list_flag: u32,
    pub num_ref_loc_offsets: u32,
    pub colour_mapping_enabled_flag: u32,
}

pub fn pps_multilayer_extension_parse(data: &[u8]) -> Result<PpsMultilayerExtensionFields, Error> {
    let mut raw = RawPpsMultilayerExtensionFields {
        poc_reset_info_present_flag: 0,
        pps_infer_scaling_list_flag: 0,
        num_ref_loc_offsets: 0,
        colour_mapping_enabled_flag: 0,
    };
    let status =
        unsafe { ffi::h265nal_pps_multilayer_extension_parse(data.as_ptr(), data.len(), &mut raw) };
    status_to_result(status)?;

    Ok(PpsMultilayerExtensionFields {
        poc_reset_info_present_flag: raw.poc_reset_info_present_flag,
        pps_infer_scaling_list_flag: raw.pps_infer_scaling_list_flag,
        num_ref_loc_offsets: raw.num_ref_loc_offsets,
        colour_mapping_enabled_flag: raw.colour_mapping_enabled_flag,
    })
}
