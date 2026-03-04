use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawConfigurationBoxFields};
use crate::state::BitstreamParserState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationBoxFields {
    pub configuration_version: u32,
    pub general_profile_space: u32,
    pub general_tier_flag: u32,
    pub general_profile_idc: u32,
    pub general_profile_compatibility_flag: [u32; 32],
    pub general_constraint_indicator_flags: u64,
    pub general_level_idc: u32,
    pub min_spatial_segmentation_idc: u32,
    pub parallelism_type: u32,
    pub chroma_format_idc: u32,
    pub bit_depth_luma_minus8: u32,
    pub bit_depth_chroma_minus8: u32,
    pub avg_frame_rate: u32,
    pub constant_frame_rate: u32,
    pub num_temporal_layers: u32,
    pub temporal_id_nested: u32,
    pub length_size_minus_one: u32,
    pub num_of_arrays: u32,
    pub array_completeness_size: usize,
    pub array_completeness: [u32; 8],
    pub nal_unit_type_size: usize,
    pub nal_unit_type: [u32; 8],
    pub num_nalus_size: usize,
    pub num_nalus: [u32; 8],
    pub first_nal_unit_length_size: usize,
    pub first_nal_unit_length: [u32; 8],
}

/// Parses an hvcC configuration box and returns flattened scalar fields.
///
/// Native references:
/// - Declaration: `include/h265_configuration_box_parser.h:71` (`ParseConfigurationBox`)
/// - Unit test: `test/h265_configuration_box_parser_unittest.cc:21`
pub fn configuration_box_parse(
    data: &[u8],
    state: &mut BitstreamParserState,
) -> Result<ConfigurationBoxFields, Error> {
    let mut raw = RawConfigurationBoxFields {
        configuration_version: 0,
        general_profile_space: 0,
        general_tier_flag: 0,
        general_profile_idc: 0,
        general_profile_compatibility_flag: [0; 32],
        general_constraint_indicator_flags: 0,
        general_level_idc: 0,
        min_spatial_segmentation_idc: 0,
        parallelism_type: 0,
        chroma_format_idc: 0,
        bit_depth_luma_minus8: 0,
        bit_depth_chroma_minus8: 0,
        avg_frame_rate: 0,
        constant_frame_rate: 0,
        num_temporal_layers: 0,
        temporal_id_nested: 0,
        length_size_minus_one: 0,
        num_of_arrays: 0,
        array_completeness_size: 0,
        array_completeness: [0; 8],
        nal_unit_type_size: 0,
        nal_unit_type: [0; 8],
        num_nalus_size: 0,
        num_nalus: [0; 8],
        first_nal_unit_length_size: 0,
        first_nal_unit_length: [0; 8],
    };
    let status = unsafe {
        ffi::h265nal_configuration_box_parse(
            data.as_ptr(),
            data.len(),
            state.as_mut_ptr(),
            &mut raw,
        )
    };
    status_to_result(status)?;
    Ok(ConfigurationBoxFields {
        configuration_version: raw.configuration_version,
        general_profile_space: raw.general_profile_space,
        general_tier_flag: raw.general_tier_flag,
        general_profile_idc: raw.general_profile_idc,
        general_profile_compatibility_flag: raw.general_profile_compatibility_flag,
        general_constraint_indicator_flags: raw.general_constraint_indicator_flags,
        general_level_idc: raw.general_level_idc,
        min_spatial_segmentation_idc: raw.min_spatial_segmentation_idc,
        parallelism_type: raw.parallelism_type,
        chroma_format_idc: raw.chroma_format_idc,
        bit_depth_luma_minus8: raw.bit_depth_luma_minus8,
        bit_depth_chroma_minus8: raw.bit_depth_chroma_minus8,
        avg_frame_rate: raw.avg_frame_rate,
        constant_frame_rate: raw.constant_frame_rate,
        num_temporal_layers: raw.num_temporal_layers,
        temporal_id_nested: raw.temporal_id_nested,
        length_size_minus_one: raw.length_size_minus_one,
        num_of_arrays: raw.num_of_arrays,
        array_completeness_size: raw.array_completeness_size,
        array_completeness: raw.array_completeness,
        nal_unit_type_size: raw.nal_unit_type_size,
        nal_unit_type: raw.nal_unit_type,
        num_nalus_size: raw.num_nalus_size,
        num_nalus: raw.num_nalus,
        first_nal_unit_length_size: raw.first_nal_unit_length_size,
        first_nal_unit_length: raw.first_nal_unit_length,
    })
}
