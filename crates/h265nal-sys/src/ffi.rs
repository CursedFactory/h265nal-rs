use crate::state::RawBitstreamParserState;

pub(crate) type NalUnitTypePredicate = unsafe extern "C" fn(u32, *mut u32) -> i32;

#[repr(C)]
pub(crate) struct RawProfileInfoFields {
    pub profile_space: u32,
    pub tier_flag: u32,
    pub profile_idc: u32,
    pub profile_compatibility_flag: [u32; 32],
    pub progressive_source_flag: u32,
    pub interlaced_source_flag: u32,
    pub non_packed_constraint_flag: u32,
    pub frame_only_constraint_flag: u32,
    pub max_12bit_constraint_flag: u32,
    pub max_10bit_constraint_flag: u32,
    pub max_8bit_constraint_flag: u32,
    pub max_422chroma_constraint_flag: u32,
    pub max_420chroma_constraint_flag: u32,
    pub max_monochrome_constraint_flag: u32,
    pub intra_constraint_flag: u32,
    pub one_picture_only_constraint_flag: u32,
    pub lower_bit_rate_constraint_flag: u32,
    pub max_14bit_constraint_flag: u32,
    pub reserved_zero_33bits: u64,
    pub reserved_zero_34bits: u64,
    pub reserved_zero_43bits: u64,
    pub inbld_flag: u32,
    pub reserved_zero_bit: u32,
}

#[repr(C)]
pub(crate) struct RawProfileTierLevelFields {
    pub general: RawProfileInfoFields,
    pub general_level_idc: u32,
    pub sub_layer_profile_present_flag_size: usize,
    pub sub_layer_level_present_flag_size: usize,
    pub reserved_zero_2bits_size: usize,
    pub sub_layer_size: usize,
}

#[repr(C)]
pub(crate) struct RawPpsFields {
    pub pps_pic_parameter_set_id: u32,
    pub pps_seq_parameter_set_id: u32,
    pub dependent_slice_segments_enabled_flag: u32,
    pub output_flag_present_flag: u32,
    pub num_extra_slice_header_bits: u32,
    pub sign_data_hiding_enabled_flag: u32,
    pub cabac_init_present_flag: u32,
    pub num_ref_idx_l0_default_active_minus1: u32,
    pub num_ref_idx_l1_default_active_minus1: u32,
    pub init_qp_minus26: i32,
    pub constrained_intra_pred_flag: u32,
    pub transform_skip_enabled_flag: u32,
    pub cu_qp_delta_enabled_flag: u32,
    pub diff_cu_qp_delta_depth: u32,
    pub pps_cb_qp_offset: i32,
    pub pps_cr_qp_offset: i32,
    pub pps_slice_chroma_qp_offsets_present_flag: u32,
    pub weighted_pred_flag: u32,
    pub weighted_bipred_flag: u32,
    pub transquant_bypass_enabled_flag: u32,
    pub tiles_enabled_flag: u32,
    pub entropy_coding_sync_enabled_flag: u32,
    pub pps_loop_filter_across_slices_enabled_flag: u32,
    pub deblocking_filter_control_present_flag: u32,
    pub pps_scaling_list_data_present_flag: u32,
    pub lists_modification_present_flag: u32,
    pub log2_parallel_merge_level_minus2: u32,
    pub slice_segment_header_extension_present_flag: u32,
    pub pps_extension_present_flag: u32,
}

#[repr(C)]
pub(crate) struct RawNalUnitFields {
    pub parsed_length: usize,
    pub forbidden_zero_bit: u32,
    pub nal_unit_type: u32,
    pub nuh_layer_id: u32,
    pub nuh_temporal_id_plus1: u32,
}

unsafe extern "C" {
    pub(crate) fn h265nal_annexb_count_nalus(
        data: *const u8,
        len: usize,
        out_count: *mut usize,
    ) -> i32;
    pub(crate) fn h265nal_annexb_dump(data: *const u8, len: usize, dump_flags: u32) -> i32;

    pub(crate) fn h265nal_bitstream_parser_state_create(
        out_state: *mut *mut RawBitstreamParserState,
    ) -> i32;
    pub(crate) fn h265nal_bitstream_parser_state_free(state: *mut RawBitstreamParserState) -> i32;

    pub(crate) fn h265nal_common_is_slice_segment(
        nal_unit_type: u32,
        out_is_slice_segment: *mut u32,
    ) -> i32;
    pub(crate) fn h265nal_common_is_nal_unit_type_vcl(
        nal_unit_type: u32,
        out_is_vcl: *mut u32,
    ) -> i32;
    pub(crate) fn h265nal_common_is_nal_unit_type_non_vcl(
        nal_unit_type: u32,
        out_is_non_vcl: *mut u32,
    ) -> i32;
    pub(crate) fn h265nal_common_is_nal_unit_type_unspecified(
        nal_unit_type: u32,
        out_is_unspecified: *mut u32,
    ) -> i32;
    pub(crate) fn h265nal_common_more_rbsp_data(
        data: *const u8,
        len: usize,
        byte_offset: usize,
        bit_offset: usize,
        out_has_more: *mut u32,
        out_byte_offset: *mut usize,
        out_bit_offset: *mut usize,
    ) -> i32;

    pub(crate) fn h265nal_aud_parse_pic_type(
        data: *const u8,
        len: usize,
        out_pic_type: *mut u32,
    ) -> i32;

    pub(crate) fn h265nal_utils_get_slice_qp_y(
        data: *const u8,
        len: usize,
        state: *mut RawBitstreamParserState,
        out_values: *mut i32,
        out_capacity: usize,
        out_count: *mut usize,
    ) -> i32;

    pub(crate) fn h265nal_nal_unit_header_get_nal_unit_type(
        data: *const u8,
        len: usize,
        out_nal_unit_type: *mut u32,
    ) -> i32;

    pub(crate) fn h265nal_nal_unit_parse(
        data: *const u8,
        len: usize,
        state: *mut RawBitstreamParserState,
        add_checksum: u32,
        out_nal_unit: *mut RawNalUnitFields,
    ) -> i32;

    pub(crate) fn h265nal_profile_tier_level_parse(
        data: *const u8,
        len: usize,
        profile_present_flag: u32,
        max_num_sub_layers_minus1: u32,
        out_profile_tier_level: *mut RawProfileTierLevelFields,
    ) -> i32;

    pub(crate) fn h265nal_pps_parse(data: *const u8, len: usize, out_pps: *mut RawPpsFields)
        -> i32;

    pub(crate) fn h265nal_abi_version() -> u32;
}
