use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawSpsRangeExtensionFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpsRangeExtensionFields {
    pub transform_skip_rotation_enabled_flag: u32,
    pub transform_skip_context_enabled_flag: u32,
    pub implicit_rdpcm_enabled_flag: u32,
    pub explicit_rdpcm_enabled_flag: u32,
    pub extended_precision_processing_flag: u32,
    pub intra_smoothing_disabled_flag: u32,
    pub high_precision_offsets_enabled_flag: u32,
    pub persistent_rice_adaptation_enabled_flag: u32,
    pub cabac_bypass_alignment_enabled_flag: u32,
}

pub fn sps_range_extension_parse(data: &[u8]) -> Result<SpsRangeExtensionFields, Error> {
    let mut raw = RawSpsRangeExtensionFields {
        transform_skip_rotation_enabled_flag: 0,
        transform_skip_context_enabled_flag: 0,
        implicit_rdpcm_enabled_flag: 0,
        explicit_rdpcm_enabled_flag: 0,
        extended_precision_processing_flag: 0,
        intra_smoothing_disabled_flag: 0,
        high_precision_offsets_enabled_flag: 0,
        persistent_rice_adaptation_enabled_flag: 0,
        cabac_bypass_alignment_enabled_flag: 0,
    };
    let status =
        unsafe { ffi::h265nal_sps_range_extension_parse(data.as_ptr(), data.len(), &mut raw) };
    status_to_result(status)?;

    Ok(SpsRangeExtensionFields {
        transform_skip_rotation_enabled_flag: raw.transform_skip_rotation_enabled_flag,
        transform_skip_context_enabled_flag: raw.transform_skip_context_enabled_flag,
        implicit_rdpcm_enabled_flag: raw.implicit_rdpcm_enabled_flag,
        explicit_rdpcm_enabled_flag: raw.explicit_rdpcm_enabled_flag,
        extended_precision_processing_flag: raw.extended_precision_processing_flag,
        intra_smoothing_disabled_flag: raw.intra_smoothing_disabled_flag,
        high_precision_offsets_enabled_flag: raw.high_precision_offsets_enabled_flag,
        persistent_rice_adaptation_enabled_flag: raw.persistent_rice_adaptation_enabled_flag,
        cabac_bypass_alignment_enabled_flag: raw.cabac_bypass_alignment_enabled_flag,
    })
}
