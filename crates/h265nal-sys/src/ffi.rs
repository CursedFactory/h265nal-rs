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
pub(crate) struct RawPpsMultilayerExtensionFields {
    pub poc_reset_info_present_flag: u32,
    pub pps_infer_scaling_list_flag: u32,
    pub num_ref_loc_offsets: u32,
    pub colour_mapping_enabled_flag: u32,
}

#[repr(C)]
pub(crate) struct RawPpsSccExtensionFields {
    pub pps_curr_pic_ref_enabled_flag: u32,
    pub residual_adaptive_colour_transform_enabled_flag: u32,
    pub pps_palette_predictor_initializer_present_flag: u32,
    pub pps_palette_predictor_initializers_size: usize,
}

#[repr(C)]
pub(crate) struct RawPredWeightTableFields {
    pub chroma_array_type: u32,
    pub num_ref_idx_l0_active_minus1: u32,
    pub luma_log2_weight_denom: u32,
    pub delta_chroma_log2_weight_denom: i32,
    pub luma_weight_l0_flag_size: usize,
    pub luma_weight_l0_flag: [u32; 16],
    pub chroma_weight_l0_flag_size: usize,
    pub chroma_weight_l0_flag: [u32; 16],
}

#[repr(C)]
pub(crate) struct RawNalUnitFields {
    pub checksum_size: usize,
    pub checksum: [u8; 4],
    pub parsed_length: usize,
    pub forbidden_zero_bit: u32,
    pub nal_unit_type: u32,
    pub nuh_layer_id: u32,
    pub nuh_temporal_id_plus1: u32,
}

#[repr(C)]
pub(crate) struct RawBitstreamNalFields {
    pub offset: usize,
    pub length: usize,
    pub parsed_length: usize,
    pub checksum_size: usize,
    pub checksum: [u8; 4],
    pub forbidden_zero_bit: u32,
    pub nal_unit_type: u32,
    pub nuh_layer_id: u32,
    pub nuh_temporal_id_plus1: u32,
}

#[repr(C)]
pub(crate) struct RawConfigurationBoxFields {
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

#[repr(C)]
pub(crate) struct RawSpsFields {
    pub sps_video_parameter_set_id: u32,
    pub sps_max_sub_layers_minus1: u32,
    pub sps_temporal_id_nesting_flag: u32,
    pub profile_tier_level: RawProfileTierLevelFields,
    pub sps_seq_parameter_set_id: u32,
    pub chroma_format_idc: u32,
    pub pic_width_in_luma_samples: u32,
    pub pic_height_in_luma_samples: u32,
    pub conformance_window_flag: u32,
    pub conf_win_left_offset: u32,
    pub conf_win_right_offset: u32,
    pub conf_win_top_offset: u32,
    pub conf_win_bottom_offset: u32,
    pub bit_depth_luma_minus8: u32,
    pub bit_depth_chroma_minus8: u32,
    pub log2_max_pic_order_cnt_lsb_minus4: u32,
    pub sps_sub_layer_ordering_info_present_flag: u32,
    pub sps_max_dec_pic_buffering_minus1_0: u32,
    pub sps_max_num_reorder_pics_0: u32,
    pub sps_max_latency_increase_plus1_0: u32,
    pub scaling_list_enabled_flag: u32,
    pub amp_enabled_flag: u32,
    pub sample_adaptive_offset_enabled_flag: u32,
    pub pcm_enabled_flag: u32,
    pub num_short_term_ref_pic_sets: u32,
    pub long_term_ref_pics_present_flag: u32,
    pub sps_temporal_mvp_enabled_flag: u32,
    pub strong_intra_smoothing_enabled_flag: u32,
    pub vui_parameters_present_flag: u32,
    pub vui_video_signal_type_present_flag: u32,
    pub vui_video_format: u32,
    pub vui_video_full_range_flag: u32,
    pub vui_colour_description_present_flag: u32,
    pub vui_colour_primaries: u32,
    pub vui_transfer_characteristics: u32,
    pub vui_matrix_coeffs: u32,
    pub vui_bitstream_restriction_flag: u32,
    pub vui_max_bytes_per_pic_denom: u32,
    pub vui_max_bits_per_min_cu_denom: u32,
    pub vui_log2_max_mv_length_horizontal: u32,
    pub vui_log2_max_mv_length_vertical: u32,
    pub sps_extension_present_flag: u32,
    pub sps_range_extension_flag: u32,
    pub sps_multilayer_extension_flag: u32,
    pub sps_3d_extension_flag: u32,
    pub sps_scc_extension_flag: u32,
    pub sps_extension_4bits: u32,
    pub pic_size_in_ctbs_y: u32,
}

#[repr(C)]
pub(crate) struct RawSliceSegmentLayerFields {
    pub has_slice_segment_header: u32,
    pub nal_unit_type: u32,
    pub first_slice_segment_in_pic_flag: u32,
    pub no_output_of_prior_pics_flag: u32,
    pub slice_pic_parameter_set_id: u32,
    pub slice_segment_address: u32,
    pub slice_type: u32,
    pub slice_sao_luma_flag: u32,
    pub slice_sao_chroma_flag: u32,
    pub slice_qp_delta: i32,
    pub num_entry_point_offsets: u32,
}

#[repr(C)]
pub(crate) struct RawRtpFields {
    pub forbidden_zero_bit: u32,
    pub nal_unit_type: u32,
    pub nuh_layer_id: u32,
    pub nuh_temporal_id_plus1: u32,
    pub packet_kind: u32,
    pub has_payload_sps: u32,
    pub payload_sps_seq_parameter_set_id: u32,
    pub payload_sps_pic_width_in_luma_samples: u32,
    pub payload_sps_pic_height_in_luma_samples: u32,
    pub has_payload_pps: u32,
    pub payload_pps_pic_parameter_set_id: u32,
    pub payload_pps_init_qp_minus26: i32,
    pub has_payload_slice_segment_header: u32,
    pub payload_slice_nal_unit_type: u32,
    pub payload_slice_first_slice_segment_in_pic_flag: u32,
    pub payload_slice_no_output_of_prior_pics_flag: u32,
    pub payload_slice_pic_parameter_set_id: u32,
    pub payload_slice_type: u32,
    pub payload_slice_qp_delta: i32,
    pub ap_nal_unit_sizes_count: usize,
    pub ap_nal_unit_headers_count: usize,
    pub ap_nal_unit_payloads_count: usize,
    pub ap_nal_unit_type_count: usize,
    pub ap_nal_unit_types: [u32; 8],
    pub fu_s_bit: u32,
    pub fu_e_bit: u32,
    pub fu_type: u32,
    pub fu_has_nal_unit_payload: u32,
}

#[repr(C)]
pub(crate) struct RawSeiMessageFields {
    pub payload_type: i32,
    pub payload_size: u32,
    pub has_user_data_registered_itu_t_t35: u32,
    pub user_data_registered_itu_t_t35_country_code: u8,
    pub user_data_registered_itu_t_t35_country_code_extension_byte: u8,
    pub has_user_data_unregistered: u32,
    pub user_data_unregistered_uuid_iso_iec_11578_1: u64,
    pub user_data_unregistered_uuid_iso_iec_11578_2: u64,
    pub has_alpha_channel_info: u32,
    pub alpha_channel_cancel_flag: u32,
    pub alpha_channel_use_idc: u32,
    pub alpha_channel_bit_depth_minus8: u32,
    pub alpha_transparent_value: u32,
    pub alpha_opaque_value: u32,
    pub alpha_channel_incr_flag: u32,
    pub alpha_channel_clip_flag: u32,
    pub alpha_channel_clip_type_flag: u32,
    pub has_mastering_display_colour_volume: u32,
    pub mastering_display_display_primaries_x: [u16; 3],
    pub mastering_display_display_primaries_y: [u16; 3],
    pub mastering_display_white_point_x: u16,
    pub mastering_display_white_point_y: u16,
    pub mastering_display_max_display_mastering_luminance: u32,
    pub mastering_display_min_display_mastering_luminance: u32,
    pub has_content_light_level_info: u32,
    pub content_light_level_max_content_light_level: u16,
    pub content_light_level_max_pic_average_light_level: u16,
}

#[repr(C)]
pub(crate) struct RawSpsMultilayerExtensionFields {
    pub inter_view_mv_vert_constraint_flag: u32,
}

#[repr(C)]
pub(crate) struct RawScalingListDataFields {
    pub size_id_count: u32,
    pub matrix_id_count: u32,
    pub scaling_list_pred_mode_flag: [u32; 24],
    pub scaling_list_pred_matrix_id_delta: [u32; 24],
    pub scaling_list_dc_coef_minus8: [i32; 24],
    pub scaling_list_size: [usize; 24],
    pub scaling_list_first_coef: [u32; 24],
    pub scaling_list_all_8: [u32; 24],
}

#[repr(C)]
pub(crate) struct RawSps3dExtensionFields {
    pub iv_di_mc_enabled_flag_size: usize,
    pub iv_di_mc_enabled_flag: [u32; 8],
    pub iv_mv_scal_enabled_flag_size: usize,
    pub iv_mv_scal_enabled_flag: [u32; 8],
    pub log2_ivmc_sub_pb_size_minus3: u32,
    pub iv_res_pred_enabled_flag: u32,
    pub depth_ref_enabled_flag: u32,
    pub vsp_mc_enabled_flag: u32,
    pub dbbp_enabled_flag: u32,
    pub tex_mc_enabled_flag: u32,
    pub log2_texmc_sub_pb_size_minus3: u32,
    pub intra_contour_enabled_flag: u32,
    pub intra_dc_only_wedge_enabled_flag: u32,
    pub cqt_cu_part_pred_enabled_flag: u32,
    pub inter_dc_only_enabled_flag: u32,
    pub skip_intra_enabled_flag: u32,
}

#[repr(C)]
pub(crate) struct RawSpsRangeExtensionFields {
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

#[repr(C)]
pub(crate) struct RawSpsSccExtensionFields {
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

#[repr(C)]
pub(crate) struct RawStRefPicSetFields {
    pub st_rps_idx: u32,
    pub num_short_term_ref_pic_sets: u32,
    pub max_num_pics: u32,
    pub num_negative_pics: u32,
    pub num_positive_pics: u32,
    pub delta_poc_s0_minus1_size: usize,
    pub delta_poc_s0_minus1: [u32; 16],
}

#[repr(C)]
pub(crate) struct RawSubLayerHrdParametersFields {
    pub sub_layer_id: u32,
    pub cpb_cnt: u32,
    pub sub_pic_hrd_params_present_flag: u32,
    pub bit_rate_value_minus1_size: usize,
    pub bit_rate_value_minus1: [u32; 32],
    pub cpb_size_value_minus1_size: usize,
    pub cpb_size_value_minus1: [u32; 32],
    pub cpb_size_du_value_minus1_size: usize,
    pub cpb_size_du_value_minus1: [u32; 32],
    pub bit_rate_du_value_minus1_size: usize,
    pub bit_rate_du_value_minus1: [u32; 32],
    pub cbr_flag_size: usize,
    pub cbr_flag: [u32; 32],
}

#[repr(C)]
pub(crate) struct RawHrdParametersFields {
    pub common_inf_present_flag: u32,
    pub max_num_sub_layers_minus1: u32,
    pub nal_hrd_parameters_present_flag: u32,
    pub vcl_hrd_parameters_present_flag: u32,
    pub sub_pic_hrd_params_present_flag: u32,
    pub tick_divisor_minus2: u32,
    pub du_cpb_removal_delay_increment_length_minus1: u32,
    pub sub_pic_cpb_params_in_pic_timing_sei_flag: u32,
    pub dpb_output_delay_du_length_minus1: u32,
    pub bit_rate_scale: u32,
    pub cpb_size_scale: u32,
    pub cpb_size_du_scale: u32,
    pub initial_cpb_removal_delay_length_minus1: u32,
    pub au_cpb_removal_delay_length_minus1: u32,
    pub dpb_output_delay_length_minus1: u32,
    pub fixed_pic_rate_general_flag_size: usize,
    pub fixed_pic_rate_general_flag: [u32; 8],
    pub fixed_pic_rate_within_cvs_flag_size: usize,
    pub fixed_pic_rate_within_cvs_flag: [u32; 8],
    pub elemental_duration_in_tc_minus1_size: usize,
    pub elemental_duration_in_tc_minus1: [u32; 8],
    pub low_delay_hrd_flag_size: usize,
    pub low_delay_hrd_flag: [u32; 8],
    pub cpb_cnt_minus1_size: usize,
    pub cpb_cnt_minus1: [u32; 8],
    pub sub_layer_hrd_parameters_vector_size: usize,
    pub sub_layer_hrd_parameters_0_present: u32,
    pub sub_layer_hrd_parameters_0: RawSubLayerHrdParametersFields,
}

#[repr(C)]
pub(crate) struct RawVpsFields {
    pub vps_video_parameter_set_id: u32,
    pub vps_base_layer_internal_flag: u32,
    pub vps_base_layer_available_flag: u32,
    pub vps_max_layers_minus1: u32,
    pub vps_max_sub_layers_minus1: u32,
    pub vps_temporal_id_nesting_flag: u32,
    pub vps_reserved_0xffff_16bits: u32,
    pub profile_tier_level: RawProfileTierLevelFields,
    pub vps_sub_layer_ordering_info_present_flag: u32,
    pub vps_max_dec_pic_buffering_minus1_size: usize,
    pub vps_max_dec_pic_buffering_minus1: [u32; 8],
    pub vps_max_num_reorder_pics_size: usize,
    pub vps_max_num_reorder_pics: [u32; 8],
    pub vps_max_latency_increase_plus1_size: usize,
    pub vps_max_latency_increase_plus1: [u32; 8],
    pub vps_max_layer_id: u32,
    pub vps_num_layer_sets_minus1: u32,
    pub layer_id_included_flag_size: usize,
    pub layer_id_included_flag_row_width: u32,
    pub layer_id_included_flag_row0_mask: u64,
    pub layer_id_included_flag_row1_mask: u64,
    pub vps_timing_info_present_flag: u32,
    pub vps_num_units_in_tick: u32,
    pub vps_time_scale: u32,
    pub vps_poc_proportional_to_timing_flag: u32,
    pub vps_num_ticks_poc_diff_one_minus1: u32,
    pub vps_num_hrd_parameters: u32,
    pub hrd_layer_set_idx_size: usize,
    pub cprms_present_flag_size: usize,
    pub vps_extension_flag: u32,
    pub vps_extension_data_flag: u32,
}

#[repr(C)]
pub(crate) struct RawVuiParametersFields {
    pub sps_max_sub_layers_minus1: u32,
    pub aspect_ratio_info_present_flag: u32,
    pub aspect_ratio_idc: u32,
    pub sar_width: u32,
    pub sar_height: u32,
    pub overscan_info_present_flag: u32,
    pub overscan_appropriate_flag: u32,
    pub video_signal_type_present_flag: u32,
    pub video_format: u32,
    pub video_full_range_flag: u32,
    pub colour_description_present_flag: u32,
    pub colour_primaries: u32,
    pub transfer_characteristics: u32,
    pub matrix_coeffs: u32,
    pub chroma_loc_info_present_flag: u32,
    pub chroma_sample_loc_type_top_field: u32,
    pub chroma_sample_loc_type_bottom_field: u32,
    pub neutral_chroma_indication_flag: u32,
    pub field_seq_flag: u32,
    pub frame_field_info_present_flag: u32,
    pub default_display_window_flag: u32,
    pub def_disp_win_left_offset: u32,
    pub def_disp_win_right_offset: u32,
    pub def_disp_win_top_offset: u32,
    pub def_disp_win_bottom_offset: u32,
    pub vui_timing_info_present_flag: u32,
    pub vui_num_units_in_tick: u32,
    pub vui_time_scale: u32,
    pub vui_poc_proportional_to_timing_flag: u32,
    pub vui_num_ticks_poc_diff_one_minus1: u32,
    pub vui_hrd_parameters_present_flag: u32,
    pub bitstream_restriction_flag: u32,
    pub tiles_fixed_structure_flag: u32,
    pub motion_vectors_over_pic_boundaries_flag: u32,
    pub restricted_ref_pic_lists_flag: u32,
    pub min_spatial_segmentation_idc: u32,
    pub max_bytes_per_pic_denom: u32,
    pub max_bits_per_min_cu_denom: u32,
    pub log2_max_mv_length_horizontal: u32,
    pub log2_max_mv_length_vertical: u32,
    pub has_hrd_parameters: u32,
    pub hrd_nal_hrd_parameters_present_flag: u32,
    pub hrd_vcl_hrd_parameters_present_flag: u32,
    pub hrd_sub_pic_hrd_params_present_flag: u32,
    pub hrd_tick_divisor_minus2: u32,
    pub hrd_du_cpb_removal_delay_increment_length_minus1: u32,
    pub hrd_sub_pic_cpb_params_in_pic_timing_sei_flag: u32,
    pub hrd_dpb_output_delay_du_length_minus1: u32,
    pub hrd_bit_rate_scale: u32,
    pub hrd_cpb_size_scale: u32,
    pub hrd_cpb_size_du_scale: u32,
    pub hrd_initial_cpb_removal_delay_length_minus1: u32,
    pub hrd_au_cpb_removal_delay_length_minus1: u32,
    pub hrd_dpb_output_delay_length_minus1: u32,
    pub hrd_fixed_pic_rate_general_flag_size: usize,
    pub hrd_fixed_pic_rate_general_flag: [u32; 8],
    pub hrd_fixed_pic_rate_within_cvs_flag_size: usize,
    pub hrd_fixed_pic_rate_within_cvs_flag: [u32; 8],
    pub hrd_elemental_duration_in_tc_minus1_size: usize,
    pub hrd_elemental_duration_in_tc_minus1: [u32; 8],
    pub hrd_low_delay_hrd_flag_size: usize,
    pub hrd_low_delay_hrd_flag: [u32; 8],
    pub hrd_cpb_cnt_minus1_size: usize,
    pub hrd_cpb_cnt_minus1: [u32; 8],
    pub hrd_sub_layer_hrd_parameters_vector_size: usize,
    pub hrd_sub_layer_hrd_parameters_0_present: u32,
    pub hrd_sub_layer_hrd_parameters_0: RawSubLayerHrdParametersFields,
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

    pub(crate) fn h265nal_bitstream_parse(
        data: *const u8,
        len: usize,
        state: *mut RawBitstreamParserState,
        add_checksum: u32,
        out_nals: *mut RawBitstreamNalFields,
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

    pub(crate) fn h265nal_configuration_box_parse(
        data: *const u8,
        len: usize,
        state: *mut RawBitstreamParserState,
        out_configuration_box: *mut RawConfigurationBoxFields,
    ) -> i32;

    pub(crate) fn h265nal_pps_multilayer_extension_parse(
        data: *const u8,
        len: usize,
        out_pps_multilayer_extension: *mut RawPpsMultilayerExtensionFields,
    ) -> i32;

    pub(crate) fn h265nal_pps_scc_extension_parse(
        data: *const u8,
        len: usize,
        out_pps_scc_extension: *mut RawPpsSccExtensionFields,
    ) -> i32;

    pub(crate) fn h265nal_pred_weight_table_parse(
        data: *const u8,
        len: usize,
        chroma_array_type: u32,
        num_ref_idx_l0_active_minus1: u32,
        out_pred_weight_table: *mut RawPredWeightTableFields,
    ) -> i32;

    pub(crate) fn h265nal_sps_parse(data: *const u8, len: usize, out_sps: *mut RawSpsFields)
        -> i32;

    pub(crate) fn h265nal_bitstream_parser_state_seed_vps(
        state: *mut RawBitstreamParserState,
        vps_id: u32,
    ) -> i32;

    pub(crate) fn h265nal_bitstream_parser_state_seed_sps(
        state: *mut RawBitstreamParserState,
        sps_id: u32,
        sample_adaptive_offset_enabled_flag: u32,
        chroma_format_idc: u32,
        log2_min_luma_coding_block_size_minus3: u32,
        log2_diff_max_min_luma_coding_block_size: u32,
        pic_width_in_luma_samples: u32,
        pic_height_in_luma_samples: u32,
    ) -> i32;

    pub(crate) fn h265nal_bitstream_parser_state_seed_pps(
        state: *mut RawBitstreamParserState,
        pps_id: u32,
    ) -> i32;

    pub(crate) fn h265nal_slice_segment_layer_parse(
        data: *const u8,
        len: usize,
        nal_unit_type: u32,
        state: *mut RawBitstreamParserState,
        out_slice_segment_layer: *mut RawSliceSegmentLayerFields,
    ) -> i32;

    pub(crate) fn h265nal_rtp_parse(
        data: *const u8,
        len: usize,
        state: *mut RawBitstreamParserState,
        out_rtp: *mut RawRtpFields,
    ) -> i32;

    pub(crate) fn h265nal_sei_parse(
        data: *const u8,
        len: usize,
        out_sei_message: *mut RawSeiMessageFields,
    ) -> i32;

    pub(crate) fn h265nal_sps_multilayer_extension_parse(
        data: *const u8,
        len: usize,
        out_sps_multilayer_extension: *mut RawSpsMultilayerExtensionFields,
    ) -> i32;

    pub(crate) fn h265nal_scaling_list_data_parse(
        data: *const u8,
        len: usize,
        out_scaling_list_data: *mut RawScalingListDataFields,
    ) -> i32;

    pub(crate) fn h265nal_sps_3d_extension_parse(
        data: *const u8,
        len: usize,
        out_sps_3d_extension: *mut RawSps3dExtensionFields,
    ) -> i32;

    pub(crate) fn h265nal_sps_range_extension_parse(
        data: *const u8,
        len: usize,
        out_sps_range_extension: *mut RawSpsRangeExtensionFields,
    ) -> i32;

    pub(crate) fn h265nal_sps_scc_extension_parse(
        data: *const u8,
        len: usize,
        chroma_format_idc: u32,
        bit_depth_luma_minus8: u32,
        bit_depth_chroma_minus8: u32,
        out_sps_scc_extension: *mut RawSpsSccExtensionFields,
    ) -> i32;

    pub(crate) fn h265nal_st_ref_pic_set_parse(
        data: *const u8,
        len: usize,
        st_rps_idx: u32,
        num_short_term_ref_pic_sets: u32,
        max_num_pics: u32,
        out_st_ref_pic_set: *mut RawStRefPicSetFields,
    ) -> i32;

    pub(crate) fn h265nal_sub_layer_hrd_parameters_parse(
        data: *const u8,
        len: usize,
        sub_layer_id: u32,
        cpb_cnt: u32,
        sub_pic_hrd_params_present_flag: u32,
        out_sub_layer_hrd_parameters: *mut RawSubLayerHrdParametersFields,
    ) -> i32;

    pub(crate) fn h265nal_hrd_parameters_parse(
        data: *const u8,
        len: usize,
        common_inf_present_flag: u32,
        max_num_sub_layers_minus1: u32,
        out_hrd_parameters: *mut RawHrdParametersFields,
    ) -> i32;

    pub(crate) fn h265nal_vps_parse(data: *const u8, len: usize, out_vps: *mut RawVpsFields)
        -> i32;

    pub(crate) fn h265nal_vui_parameters_parse(
        data: *const u8,
        len: usize,
        sps_max_sub_layers_minus1: u32,
        out_vui_parameters: *mut RawVuiParametersFields,
    ) -> i32;

    pub(crate) fn h265nal_abi_version() -> u32;
}
