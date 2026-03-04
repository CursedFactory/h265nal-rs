#ifndef H265_C_API_H_
#define H265_C_API_H_

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define H265NAL_C_ABI_VERSION 1u

enum {
  H265NAL_STATUS_OK = 0,
  H265NAL_STATUS_INVALID_ARGUMENT = 1,
  H265NAL_STATUS_PARSE_FAILURE = 2,
  H265NAL_STATUS_DUMP_UNAVAILABLE = 3,
};

enum {
  H265NAL_DUMP_FLAG_ONE_LINE = 1u << 0,
};

enum {
  H265NAL_RTP_PACKET_KIND_NONE = 0,
  H265NAL_RTP_PACKET_KIND_SINGLE = 1,
  H265NAL_RTP_PACKET_KIND_AP = 2,
  H265NAL_RTP_PACKET_KIND_FU = 3,
};

typedef struct h265nal_bitstream_parser_state
    h265nal_bitstream_parser_state;

// DIVERGENCE: flattened profile info fields for C ABI parity tests.
typedef struct h265nal_profile_info_fields {
  uint32_t profile_space;
  uint32_t tier_flag;
  uint32_t profile_idc;
  uint32_t profile_compatibility_flag[32];
  uint32_t progressive_source_flag;
  uint32_t interlaced_source_flag;
  uint32_t non_packed_constraint_flag;
  uint32_t frame_only_constraint_flag;
  uint32_t max_12bit_constraint_flag;
  uint32_t max_10bit_constraint_flag;
  uint32_t max_8bit_constraint_flag;
  uint32_t max_422chroma_constraint_flag;
  uint32_t max_420chroma_constraint_flag;
  uint32_t max_monochrome_constraint_flag;
  uint32_t intra_constraint_flag;
  uint32_t one_picture_only_constraint_flag;
  uint32_t lower_bit_rate_constraint_flag;
  uint32_t max_14bit_constraint_flag;
  uint64_t reserved_zero_33bits;
  uint64_t reserved_zero_34bits;
  uint64_t reserved_zero_43bits;
  uint32_t inbld_flag;
  uint32_t reserved_zero_bit;
} h265nal_profile_info_fields;

// DIVERGENCE: flattened profile_tier_level fields for C ABI parity tests.
typedef struct h265nal_profile_tier_level_fields {
  h265nal_profile_info_fields general;
  uint32_t general_level_idc;
  size_t sub_layer_profile_present_flag_size;
  size_t sub_layer_level_present_flag_size;
  size_t reserved_zero_2bits_size;
  size_t sub_layer_size;
} h265nal_profile_tier_level_fields;

// DIVERGENCE: flattened PPS fields for C ABI parity tests.
typedef struct h265nal_pps_fields {
  uint32_t pps_pic_parameter_set_id;
  uint32_t pps_seq_parameter_set_id;
  uint32_t dependent_slice_segments_enabled_flag;
  uint32_t output_flag_present_flag;
  uint32_t num_extra_slice_header_bits;
  uint32_t sign_data_hiding_enabled_flag;
  uint32_t cabac_init_present_flag;
  uint32_t num_ref_idx_l0_default_active_minus1;
  uint32_t num_ref_idx_l1_default_active_minus1;
  int32_t init_qp_minus26;
  uint32_t constrained_intra_pred_flag;
  uint32_t transform_skip_enabled_flag;
  uint32_t cu_qp_delta_enabled_flag;
  uint32_t diff_cu_qp_delta_depth;
  int32_t pps_cb_qp_offset;
  int32_t pps_cr_qp_offset;
  uint32_t pps_slice_chroma_qp_offsets_present_flag;
  uint32_t weighted_pred_flag;
  uint32_t weighted_bipred_flag;
  uint32_t transquant_bypass_enabled_flag;
  uint32_t tiles_enabled_flag;
  uint32_t entropy_coding_sync_enabled_flag;
  uint32_t pps_loop_filter_across_slices_enabled_flag;
  uint32_t deblocking_filter_control_present_flag;
  uint32_t pps_scaling_list_data_present_flag;
  uint32_t lists_modification_present_flag;
  uint32_t log2_parallel_merge_level_minus2;
  uint32_t slice_segment_header_extension_present_flag;
  uint32_t pps_extension_present_flag;
} h265nal_pps_fields;

// DIVERGENCE: flattened PPS multilayer-extension fields for Rust parity tests.
typedef struct h265nal_pps_multilayer_extension_fields {
  uint32_t poc_reset_info_present_flag;
  uint32_t pps_infer_scaling_list_flag;
  uint32_t num_ref_loc_offsets;
  uint32_t colour_mapping_enabled_flag;
} h265nal_pps_multilayer_extension_fields;

// DIVERGENCE: flattened PPS SCC-extension fields for Rust parity tests.
typedef struct h265nal_pps_scc_extension_fields {
  uint32_t pps_curr_pic_ref_enabled_flag;
  uint32_t residual_adaptive_colour_transform_enabled_flag;
  uint32_t pps_palette_predictor_initializer_present_flag;
  size_t pps_palette_predictor_initializers_size;
} h265nal_pps_scc_extension_fields;

// DIVERGENCE: flattened pred-weight-table fields for Rust parity tests.
typedef struct h265nal_pred_weight_table_fields {
  uint32_t chroma_array_type;
  uint32_t num_ref_idx_l0_active_minus1;
  uint32_t luma_log2_weight_denom;
  int32_t delta_chroma_log2_weight_denom;
  size_t luma_weight_l0_flag_size;
  uint32_t luma_weight_l0_flag[16];
  size_t chroma_weight_l0_flag_size;
  uint32_t chroma_weight_l0_flag[16];
} h265nal_pred_weight_table_fields;

// DIVERGENCE: flattened NAL unit fields for C ABI parity tests.
typedef struct h265nal_nal_unit_fields {
  size_t checksum_size;
  uint8_t checksum[4];
  size_t parsed_length;
  uint32_t forbidden_zero_bit;
  uint32_t nal_unit_type;
  uint32_t nuh_layer_id;
  uint32_t nuh_temporal_id_plus1;
} h265nal_nal_unit_fields;

// DIVERGENCE: flattened bitstream NAL fields for C ABI parity tests.
typedef struct h265nal_bitstream_nal_fields {
  size_t offset;
  size_t length;
  size_t parsed_length;
  size_t checksum_size;
  uint8_t checksum[4];
  uint32_t forbidden_zero_bit;
  uint32_t nal_unit_type;
  uint32_t nuh_layer_id;
  uint32_t nuh_temporal_id_plus1;
} h265nal_bitstream_nal_fields;

// DIVERGENCE: flattened configuration-box fields for C ABI parity tests.
typedef struct h265nal_configuration_box_fields {
  uint32_t configuration_version;
  uint32_t general_profile_space;
  uint32_t general_tier_flag;
  uint32_t general_profile_idc;
  uint32_t general_profile_compatibility_flag[32];
  uint64_t general_constraint_indicator_flags;
  uint32_t general_level_idc;
  uint32_t min_spatial_segmentation_idc;
  uint32_t parallelism_type;
  uint32_t chroma_format_idc;
  uint32_t bit_depth_luma_minus8;
  uint32_t bit_depth_chroma_minus8;
  uint32_t avg_frame_rate;
  uint32_t constant_frame_rate;
  uint32_t num_temporal_layers;
  uint32_t temporal_id_nested;
  uint32_t length_size_minus_one;
  uint32_t num_of_arrays;
  size_t array_completeness_size;
  uint32_t array_completeness[8];
  size_t nal_unit_type_size;
  uint32_t nal_unit_type[8];
  size_t num_nalus_size;
  uint32_t num_nalus[8];
  size_t first_nal_unit_length_size;
  uint32_t first_nal_unit_length[8];
} h265nal_configuration_box_fields;

// DIVERGENCE: flattened SPS scalar fields for Rust parity tests.
typedef struct h265nal_sps_fields {
  uint32_t sps_video_parameter_set_id;
  uint32_t sps_max_sub_layers_minus1;
  uint32_t sps_temporal_id_nesting_flag;
  h265nal_profile_tier_level_fields profile_tier_level;
  uint32_t sps_seq_parameter_set_id;
  uint32_t chroma_format_idc;
  uint32_t pic_width_in_luma_samples;
  uint32_t pic_height_in_luma_samples;
  uint32_t conformance_window_flag;
  uint32_t conf_win_left_offset;
  uint32_t conf_win_right_offset;
  uint32_t conf_win_top_offset;
  uint32_t conf_win_bottom_offset;
  uint32_t bit_depth_luma_minus8;
  uint32_t bit_depth_chroma_minus8;
  uint32_t log2_max_pic_order_cnt_lsb_minus4;
  uint32_t sps_sub_layer_ordering_info_present_flag;
  uint32_t sps_max_dec_pic_buffering_minus1_0;
  uint32_t sps_max_num_reorder_pics_0;
  uint32_t sps_max_latency_increase_plus1_0;
  uint32_t scaling_list_enabled_flag;
  uint32_t amp_enabled_flag;
  uint32_t sample_adaptive_offset_enabled_flag;
  uint32_t pcm_enabled_flag;
  uint32_t num_short_term_ref_pic_sets;
  uint32_t long_term_ref_pics_present_flag;
  uint32_t sps_temporal_mvp_enabled_flag;
  uint32_t strong_intra_smoothing_enabled_flag;
  uint32_t vui_parameters_present_flag;
  uint32_t vui_video_signal_type_present_flag;
  uint32_t vui_video_format;
  uint32_t vui_video_full_range_flag;
  uint32_t vui_colour_description_present_flag;
  uint32_t vui_colour_primaries;
  uint32_t vui_transfer_characteristics;
  uint32_t vui_matrix_coeffs;
  uint32_t vui_bitstream_restriction_flag;
  uint32_t vui_max_bytes_per_pic_denom;
  uint32_t vui_max_bits_per_min_cu_denom;
  uint32_t vui_log2_max_mv_length_horizontal;
  uint32_t vui_log2_max_mv_length_vertical;
  uint32_t sps_extension_present_flag;
  uint32_t sps_range_extension_flag;
  uint32_t sps_multilayer_extension_flag;
  uint32_t sps_3d_extension_flag;
  uint32_t sps_scc_extension_flag;
  uint32_t sps_extension_4bits;
  uint32_t pic_size_in_ctbs_y;
} h265nal_sps_fields;

// DIVERGENCE: flattened slice-segment-layer header fields for Rust parity tests.
typedef struct h265nal_slice_segment_layer_fields {
  uint32_t has_slice_segment_header;
  uint32_t nal_unit_type;
  uint32_t first_slice_segment_in_pic_flag;
  uint32_t no_output_of_prior_pics_flag;
  uint32_t slice_pic_parameter_set_id;
  uint32_t slice_segment_address;
  uint32_t slice_type;
  uint32_t slice_sao_luma_flag;
  uint32_t slice_sao_chroma_flag;
  int32_t slice_qp_delta;
  uint32_t num_entry_point_offsets;
} h265nal_slice_segment_layer_fields;

// DIVERGENCE: flattened RTP packet fields for Rust RTP parity tests.
typedef struct h265nal_rtp_fields {
  uint32_t forbidden_zero_bit;
  uint32_t nal_unit_type;
  uint32_t nuh_layer_id;
  uint32_t nuh_temporal_id_plus1;

  uint32_t packet_kind;

  uint32_t has_payload_sps;
  uint32_t payload_sps_seq_parameter_set_id;
  uint32_t payload_sps_pic_width_in_luma_samples;
  uint32_t payload_sps_pic_height_in_luma_samples;

  uint32_t has_payload_pps;
  uint32_t payload_pps_pic_parameter_set_id;
  int32_t payload_pps_init_qp_minus26;

  uint32_t has_payload_slice_segment_header;
  uint32_t payload_slice_nal_unit_type;
  uint32_t payload_slice_first_slice_segment_in_pic_flag;
  uint32_t payload_slice_no_output_of_prior_pics_flag;
  uint32_t payload_slice_pic_parameter_set_id;
  uint32_t payload_slice_type;
  int32_t payload_slice_qp_delta;

  size_t ap_nal_unit_sizes_count;
  size_t ap_nal_unit_headers_count;
  size_t ap_nal_unit_payloads_count;
  size_t ap_nal_unit_type_count;
  uint32_t ap_nal_unit_types[8];

  uint32_t fu_s_bit;
  uint32_t fu_e_bit;
  uint32_t fu_type;
  uint32_t fu_has_nal_unit_payload;
} h265nal_rtp_fields;

// DIVERGENCE: flattened SEI payload-family fields for Rust parity tests.
typedef struct h265nal_sei_message_fields {
  int32_t payload_type;
  uint32_t payload_size;
  uint32_t has_user_data_registered_itu_t_t35;
  uint8_t user_data_registered_itu_t_t35_country_code;
  uint8_t user_data_registered_itu_t_t35_country_code_extension_byte;
  uint32_t has_user_data_unregistered;
  uint64_t user_data_unregistered_uuid_iso_iec_11578_1;
  uint64_t user_data_unregistered_uuid_iso_iec_11578_2;
  uint32_t has_alpha_channel_info;
  uint32_t alpha_channel_cancel_flag;
  uint32_t alpha_channel_use_idc;
  uint32_t alpha_channel_bit_depth_minus8;
  uint32_t alpha_transparent_value;
  uint32_t alpha_opaque_value;
  uint32_t alpha_channel_incr_flag;
  uint32_t alpha_channel_clip_flag;
  uint32_t alpha_channel_clip_type_flag;
  uint32_t has_mastering_display_colour_volume;
  uint16_t mastering_display_display_primaries_x[3];
  uint16_t mastering_display_display_primaries_y[3];
  uint16_t mastering_display_white_point_x;
  uint16_t mastering_display_white_point_y;
  uint32_t mastering_display_max_display_mastering_luminance;
  uint32_t mastering_display_min_display_mastering_luminance;
  uint32_t has_content_light_level_info;
  uint16_t content_light_level_max_content_light_level;
  uint16_t content_light_level_max_pic_average_light_level;
} h265nal_sei_message_fields;

// DIVERGENCE: flattened SPS multilayer extension fields for Rust parity tests.
typedef struct h265nal_sps_multilayer_extension_fields {
  uint32_t inter_view_mv_vert_constraint_flag;
} h265nal_sps_multilayer_extension_fields;

// DIVERGENCE: flattened scaling-list-data fields for Rust parity tests.
typedef struct h265nal_scaling_list_data_fields {
  uint32_t size_id_count;
  uint32_t matrix_id_count;
  uint32_t scaling_list_pred_mode_flag[24];
  uint32_t scaling_list_pred_matrix_id_delta[24];
  int32_t scaling_list_dc_coef_minus8[24];
  size_t scaling_list_size[24];
  uint32_t scaling_list_first_coef[24];
  uint32_t scaling_list_all_8[24];
} h265nal_scaling_list_data_fields;

// DIVERGENCE: flattened SPS 3D extension fields for Rust parity tests.
typedef struct h265nal_sps_3d_extension_fields {
  size_t iv_di_mc_enabled_flag_size;
  uint32_t iv_di_mc_enabled_flag[8];
  size_t iv_mv_scal_enabled_flag_size;
  uint32_t iv_mv_scal_enabled_flag[8];
  uint32_t log2_ivmc_sub_pb_size_minus3;
  uint32_t iv_res_pred_enabled_flag;
  uint32_t depth_ref_enabled_flag;
  uint32_t vsp_mc_enabled_flag;
  uint32_t dbbp_enabled_flag;
  uint32_t tex_mc_enabled_flag;
  uint32_t log2_texmc_sub_pb_size_minus3;
  uint32_t intra_contour_enabled_flag;
  uint32_t intra_dc_only_wedge_enabled_flag;
  uint32_t cqt_cu_part_pred_enabled_flag;
  uint32_t inter_dc_only_enabled_flag;
  uint32_t skip_intra_enabled_flag;
} h265nal_sps_3d_extension_fields;

// DIVERGENCE: flattened SPS range extension fields for Rust parity tests.
typedef struct h265nal_sps_range_extension_fields {
  uint32_t transform_skip_rotation_enabled_flag;
  uint32_t transform_skip_context_enabled_flag;
  uint32_t implicit_rdpcm_enabled_flag;
  uint32_t explicit_rdpcm_enabled_flag;
  uint32_t extended_precision_processing_flag;
  uint32_t intra_smoothing_disabled_flag;
  uint32_t high_precision_offsets_enabled_flag;
  uint32_t persistent_rice_adaptation_enabled_flag;
  uint32_t cabac_bypass_alignment_enabled_flag;
} h265nal_sps_range_extension_fields;

// DIVERGENCE: flattened SPS SCC extension fields for Rust parity tests.
typedef struct h265nal_sps_scc_extension_fields {
  uint32_t sps_curr_pic_ref_enabled_flag;
  uint32_t palette_mode_enabled_flag;
  uint32_t palette_max_size;
  uint32_t delta_palette_max_predictor_size;
  uint32_t sps_palette_predictor_initializers_present_flag;
  uint32_t sps_num_palette_predictor_initializers_minus1;
  size_t sps_palette_predictor_initializers_size;
  uint32_t motion_vector_resolution_control_idc;
  uint32_t intra_boundary_filtering_disabled_flag;
} h265nal_sps_scc_extension_fields;

// DIVERGENCE: flattened ST-ref-pic-set fields for Rust parity tests.
typedef struct h265nal_st_ref_pic_set_fields {
  uint32_t st_rps_idx;
  uint32_t num_short_term_ref_pic_sets;
  uint32_t max_num_pics;
  uint32_t num_negative_pics;
  uint32_t num_positive_pics;
  size_t delta_poc_s0_minus1_size;
  uint32_t delta_poc_s0_minus1[16];
} h265nal_st_ref_pic_set_fields;

// DIVERGENCE: flattened sub-layer-HRD fields for Rust parity tests.
typedef struct h265nal_sub_layer_hrd_parameters_fields {
  uint32_t sub_layer_id;
  uint32_t cpb_cnt;
  uint32_t sub_pic_hrd_params_present_flag;
  size_t bit_rate_value_minus1_size;
  uint32_t bit_rate_value_minus1[32];
  size_t cpb_size_value_minus1_size;
  uint32_t cpb_size_value_minus1[32];
  size_t cpb_size_du_value_minus1_size;
  uint32_t cpb_size_du_value_minus1[32];
  size_t bit_rate_du_value_minus1_size;
  uint32_t bit_rate_du_value_minus1[32];
  size_t cbr_flag_size;
  uint32_t cbr_flag[32];
} h265nal_sub_layer_hrd_parameters_fields;

// DIVERGENCE: flattened HRD-parameters fields for Rust parity tests.
typedef struct h265nal_hrd_parameters_fields {
  uint32_t common_inf_present_flag;
  uint32_t max_num_sub_layers_minus1;
  uint32_t nal_hrd_parameters_present_flag;
  uint32_t vcl_hrd_parameters_present_flag;
  uint32_t sub_pic_hrd_params_present_flag;
  uint32_t tick_divisor_minus2;
  uint32_t du_cpb_removal_delay_increment_length_minus1;
  uint32_t sub_pic_cpb_params_in_pic_timing_sei_flag;
  uint32_t dpb_output_delay_du_length_minus1;
  uint32_t bit_rate_scale;
  uint32_t cpb_size_scale;
  uint32_t cpb_size_du_scale;
  uint32_t initial_cpb_removal_delay_length_minus1;
  uint32_t au_cpb_removal_delay_length_minus1;
  uint32_t dpb_output_delay_length_minus1;
  size_t fixed_pic_rate_general_flag_size;
  uint32_t fixed_pic_rate_general_flag[8];
  size_t fixed_pic_rate_within_cvs_flag_size;
  uint32_t fixed_pic_rate_within_cvs_flag[8];
  size_t elemental_duration_in_tc_minus1_size;
  uint32_t elemental_duration_in_tc_minus1[8];
  size_t low_delay_hrd_flag_size;
  uint32_t low_delay_hrd_flag[8];
  size_t cpb_cnt_minus1_size;
  uint32_t cpb_cnt_minus1[8];
  size_t sub_layer_hrd_parameters_vector_size;
  uint32_t sub_layer_hrd_parameters_0_present;
  h265nal_sub_layer_hrd_parameters_fields sub_layer_hrd_parameters_0;
} h265nal_hrd_parameters_fields;

// DIVERGENCE: flattened VPS fields for Rust parity tests.
typedef struct h265nal_vps_fields {
  uint32_t vps_video_parameter_set_id;
  uint32_t vps_base_layer_internal_flag;
  uint32_t vps_base_layer_available_flag;
  uint32_t vps_max_layers_minus1;
  uint32_t vps_max_sub_layers_minus1;
  uint32_t vps_temporal_id_nesting_flag;
  uint32_t vps_reserved_0xffff_16bits;
  h265nal_profile_tier_level_fields profile_tier_level;
  uint32_t vps_sub_layer_ordering_info_present_flag;
  size_t vps_max_dec_pic_buffering_minus1_size;
  uint32_t vps_max_dec_pic_buffering_minus1[8];
  size_t vps_max_num_reorder_pics_size;
  uint32_t vps_max_num_reorder_pics[8];
  size_t vps_max_latency_increase_plus1_size;
  uint32_t vps_max_latency_increase_plus1[8];
  uint32_t vps_max_layer_id;
  uint32_t vps_num_layer_sets_minus1;
  size_t layer_id_included_flag_size;
  uint32_t layer_id_included_flag_row_width;
  uint64_t layer_id_included_flag_row0_mask;
  uint64_t layer_id_included_flag_row1_mask;
  uint32_t vps_timing_info_present_flag;
  uint32_t vps_num_units_in_tick;
  uint32_t vps_time_scale;
  uint32_t vps_poc_proportional_to_timing_flag;
  uint32_t vps_num_ticks_poc_diff_one_minus1;
  uint32_t vps_num_hrd_parameters;
  size_t hrd_layer_set_idx_size;
  size_t cprms_present_flag_size;
  uint32_t vps_extension_flag;
  uint32_t vps_extension_data_flag;
} h265nal_vps_fields;

// DIVERGENCE: flattened VUI fields for Rust parity tests.
typedef struct h265nal_vui_parameters_fields {
  uint32_t sps_max_sub_layers_minus1;
  uint32_t aspect_ratio_info_present_flag;
  uint32_t aspect_ratio_idc;
  uint32_t sar_width;
  uint32_t sar_height;
  uint32_t overscan_info_present_flag;
  uint32_t overscan_appropriate_flag;
  uint32_t video_signal_type_present_flag;
  uint32_t video_format;
  uint32_t video_full_range_flag;
  uint32_t colour_description_present_flag;
  uint32_t colour_primaries;
  uint32_t transfer_characteristics;
  uint32_t matrix_coeffs;
  uint32_t chroma_loc_info_present_flag;
  uint32_t chroma_sample_loc_type_top_field;
  uint32_t chroma_sample_loc_type_bottom_field;
  uint32_t neutral_chroma_indication_flag;
  uint32_t field_seq_flag;
  uint32_t frame_field_info_present_flag;
  uint32_t default_display_window_flag;
  uint32_t def_disp_win_left_offset;
  uint32_t def_disp_win_right_offset;
  uint32_t def_disp_win_top_offset;
  uint32_t def_disp_win_bottom_offset;
  uint32_t vui_timing_info_present_flag;
  uint32_t vui_num_units_in_tick;
  uint32_t vui_time_scale;
  uint32_t vui_poc_proportional_to_timing_flag;
  uint32_t vui_num_ticks_poc_diff_one_minus1;
  uint32_t vui_hrd_parameters_present_flag;
  uint32_t bitstream_restriction_flag;
  uint32_t tiles_fixed_structure_flag;
  uint32_t motion_vectors_over_pic_boundaries_flag;
  uint32_t restricted_ref_pic_lists_flag;
  uint32_t min_spatial_segmentation_idc;
  uint32_t max_bytes_per_pic_denom;
  uint32_t max_bits_per_min_cu_denom;
  uint32_t log2_max_mv_length_horizontal;
  uint32_t log2_max_mv_length_vertical;
  uint32_t has_hrd_parameters;
  uint32_t hrd_nal_hrd_parameters_present_flag;
  uint32_t hrd_vcl_hrd_parameters_present_flag;
  uint32_t hrd_sub_pic_hrd_params_present_flag;
  uint32_t hrd_tick_divisor_minus2;
  uint32_t hrd_du_cpb_removal_delay_increment_length_minus1;
  uint32_t hrd_sub_pic_cpb_params_in_pic_timing_sei_flag;
  uint32_t hrd_dpb_output_delay_du_length_minus1;
  uint32_t hrd_bit_rate_scale;
  uint32_t hrd_cpb_size_scale;
  uint32_t hrd_cpb_size_du_scale;
  uint32_t hrd_initial_cpb_removal_delay_length_minus1;
  uint32_t hrd_au_cpb_removal_delay_length_minus1;
  uint32_t hrd_dpb_output_delay_length_minus1;
  size_t hrd_fixed_pic_rate_general_flag_size;
  uint32_t hrd_fixed_pic_rate_general_flag[8];
  size_t hrd_fixed_pic_rate_within_cvs_flag_size;
  uint32_t hrd_fixed_pic_rate_within_cvs_flag[8];
  size_t hrd_elemental_duration_in_tc_minus1_size;
  uint32_t hrd_elemental_duration_in_tc_minus1[8];
  size_t hrd_low_delay_hrd_flag_size;
  uint32_t hrd_low_delay_hrd_flag[8];
  size_t hrd_cpb_cnt_minus1_size;
  uint32_t hrd_cpb_cnt_minus1[8];
  size_t hrd_sub_layer_hrd_parameters_vector_size;
  uint32_t hrd_sub_layer_hrd_parameters_0_present;
  h265nal_sub_layer_hrd_parameters_fields hrd_sub_layer_hrd_parameters_0;
} h265nal_vui_parameters_fields;

int h265nal_annexb_count_nalus(const uint8_t* data,
                               size_t len,
                               size_t* out_count);

int h265nal_annexb_dump(const uint8_t* data, size_t len, uint32_t dump_flags);

// DIVERGENCE: expose parser-state lifecycle for stateful Rust parity tests.
int h265nal_bitstream_parser_state_create(
    h265nal_bitstream_parser_state** out_state);

// DIVERGENCE: expose parser-state lifecycle for stateful Rust parity tests.
int h265nal_bitstream_parser_state_free(
    h265nal_bitstream_parser_state* state);

int h265nal_common_is_slice_segment(uint32_t nal_unit_type,
                                    uint32_t* out_is_slice_segment);

int h265nal_common_is_nal_unit_type_vcl(uint32_t nal_unit_type,
                                        uint32_t* out_is_vcl);

int h265nal_common_is_nal_unit_type_non_vcl(uint32_t nal_unit_type,
                                            uint32_t* out_is_non_vcl);

int h265nal_common_is_nal_unit_type_unspecified(
    uint32_t nal_unit_type,
    uint32_t* out_is_unspecified);

// DIVERGENCE: expose `more_rbsp_data` with explicit cursor I/O for parity tests.
int h265nal_common_more_rbsp_data(const uint8_t* data,
                                  size_t len,
                                  size_t byte_offset,
                                  size_t bit_offset,
                                  uint32_t* out_has_more,
                                  size_t* out_byte_offset,
                                  size_t* out_bit_offset);

int h265nal_aud_parse_pic_type(const uint8_t* data,
                               size_t len,
                               uint32_t* out_pic_type);

// DIVERGENCE: expose `H265Utils::GetSliceQpY` as a C ABI helper.
int h265nal_utils_get_slice_qp_y(const uint8_t* data,
                                 size_t len,
                                 h265nal_bitstream_parser_state* state,
                                 int32_t* out_values,
                                 size_t out_capacity,
                                 size_t* out_count);

// DIVERGENCE: expose `H265BitstreamParser::ParseBitstream` metadata/checksum.
int h265nal_bitstream_parse(const uint8_t* data,
                            size_t len,
                            h265nal_bitstream_parser_state* state,
                            uint32_t add_checksum,
                            h265nal_bitstream_nal_fields* out_nals,
                            size_t out_capacity,
                            size_t* out_count);

// DIVERGENCE: expose `H265NalUnitHeaderParser::GetNalUnitType` helper.
int h265nal_nal_unit_header_get_nal_unit_type(const uint8_t* data,
                                              size_t len,
                                              uint32_t* out_nal_unit_type);

// DIVERGENCE: expose `H265NalUnitParser::ParseNalUnit` core fields.
int h265nal_nal_unit_parse(const uint8_t* data,
                           size_t len,
                           h265nal_bitstream_parser_state* state,
                           uint32_t add_checksum,
                           h265nal_nal_unit_fields* out_nal_unit);

// DIVERGENCE: expose `H265ProfileTierLevelParser::ParseProfileTierLevel`.
int h265nal_profile_tier_level_parse(
    const uint8_t* data,
    size_t len,
    uint32_t profile_present_flag,
    uint32_t max_num_sub_layers_minus1,
    h265nal_profile_tier_level_fields* out_profile_tier_level);

// DIVERGENCE: expose `H265PpsParser::ParsePps` core fields.
int h265nal_pps_parse(const uint8_t* data,
                      size_t len,
                      h265nal_pps_fields* out_pps);

// DIVERGENCE: expose `H265ConfigurationBoxParser::ParseConfigurationBox` fields.
int h265nal_configuration_box_parse(
    const uint8_t* data,
    size_t len,
    h265nal_bitstream_parser_state* state,
    h265nal_configuration_box_fields* out_configuration_box);

// DIVERGENCE: expose `H265PpsMultilayerExtensionParser::ParsePpsMultilayerExtension`.
int h265nal_pps_multilayer_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_pps_multilayer_extension_fields* out_pps_multilayer_extension);

// DIVERGENCE: expose `H265PpsSccExtensionParser::ParsePpsSccExtension`.
int h265nal_pps_scc_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_pps_scc_extension_fields* out_pps_scc_extension);

// DIVERGENCE: expose `H265PredWeightTableParser::ParsePredWeightTable`.
int h265nal_pred_weight_table_parse(
    const uint8_t* data,
    size_t len,
    uint32_t chroma_array_type,
    uint32_t num_ref_idx_l0_active_minus1,
    h265nal_pred_weight_table_fields* out_pred_weight_table);

// DIVERGENCE: expose `H265SpsParser::ParseSps` scalar/derived fields.
int h265nal_sps_parse(const uint8_t* data,
                      size_t len,
                      h265nal_sps_fields* out_sps);

// DIVERGENCE: expose state seeding helpers for slice parser parity tests.
int h265nal_bitstream_parser_state_seed_vps(h265nal_bitstream_parser_state* state,
                                            uint32_t vps_id);

// DIVERGENCE: expose state seeding helpers for slice parser parity tests.
int h265nal_bitstream_parser_state_seed_sps(h265nal_bitstream_parser_state* state,
                                            uint32_t sps_id,
                                            uint32_t sample_adaptive_offset_enabled_flag,
                                            uint32_t chroma_format_idc,
                                            uint32_t log2_min_luma_coding_block_size_minus3,
                                            uint32_t log2_diff_max_min_luma_coding_block_size,
                                            uint32_t pic_width_in_luma_samples,
                                            uint32_t pic_height_in_luma_samples);

// DIVERGENCE: expose state seeding helpers for slice parser parity tests.
int h265nal_bitstream_parser_state_seed_pps(h265nal_bitstream_parser_state* state,
                                            uint32_t pps_id);

// DIVERGENCE: expose `H265SliceSegmentLayerParser::ParseSliceSegmentLayer` fields.
int h265nal_slice_segment_layer_parse(
    const uint8_t* data,
    size_t len,
    uint32_t nal_unit_type,
    h265nal_bitstream_parser_state* state,
    h265nal_slice_segment_layer_fields* out_slice_segment_layer);

// DIVERGENCE: expose `H265RtpParser::ParseRtp` flattened packet fields.
int h265nal_rtp_parse(const uint8_t* data,
                      size_t len,
                      h265nal_bitstream_parser_state* state,
                      h265nal_rtp_fields* out_rtp);

// DIVERGENCE: expose `H265SeiMessageParser::ParseSei` baseline fields.
int h265nal_sei_parse(const uint8_t* data,
                      size_t len,
                      h265nal_sei_message_fields* out_sei_message);

// DIVERGENCE: expose `H265SpsMultilayerExtensionParser::ParseSpsMultilayerExtension`.
int h265nal_sps_multilayer_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_sps_multilayer_extension_fields* out_sps_multilayer_extension);

// DIVERGENCE: expose `H265ScalingListDataParser::ParseScalingListData`.
int h265nal_scaling_list_data_parse(
    const uint8_t* data,
    size_t len,
    h265nal_scaling_list_data_fields* out_scaling_list_data);

// DIVERGENCE: expose `H265Sps3dExtensionParser::ParseSps3dExtension`.
int h265nal_sps_3d_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_sps_3d_extension_fields* out_sps_3d_extension);

// DIVERGENCE: expose `H265SpsRangeExtensionParser::ParseSpsRangeExtension`.
int h265nal_sps_range_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_sps_range_extension_fields* out_sps_range_extension);

// DIVERGENCE: expose `H265SpsSccExtensionParser::ParseSpsSccExtension`.
int h265nal_sps_scc_extension_parse(
    const uint8_t* data,
    size_t len,
    uint32_t chroma_format_idc,
    uint32_t bit_depth_luma_minus8,
    uint32_t bit_depth_chroma_minus8,
    h265nal_sps_scc_extension_fields* out_sps_scc_extension);

// DIVERGENCE: expose `H265StRefPicSetParser::ParseStRefPicSet`.
int h265nal_st_ref_pic_set_parse(
    const uint8_t* data,
    size_t len,
    uint32_t st_rps_idx,
    uint32_t num_short_term_ref_pic_sets,
    uint32_t max_num_pics,
    h265nal_st_ref_pic_set_fields* out_st_ref_pic_set);

// DIVERGENCE: expose `H265SubLayerHrdParametersParser::ParseSubLayerHrdParameters`.
int h265nal_sub_layer_hrd_parameters_parse(
    const uint8_t* data,
    size_t len,
    uint32_t sub_layer_id,
    uint32_t cpb_cnt,
    uint32_t sub_pic_hrd_params_present_flag,
    h265nal_sub_layer_hrd_parameters_fields* out_sub_layer_hrd_parameters);

// DIVERGENCE: expose `H265HrdParametersParser::ParseHrdParameters` fields.
int h265nal_hrd_parameters_parse(
    const uint8_t* data,
    size_t len,
    uint32_t common_inf_present_flag,
    uint32_t max_num_sub_layers_minus1,
    h265nal_hrd_parameters_fields* out_hrd_parameters);

// DIVERGENCE: expose `H265VpsParser::ParseVps` flattened fields.
int h265nal_vps_parse(const uint8_t* data,
                      size_t len,
                      h265nal_vps_fields* out_vps);

// DIVERGENCE: expose `H265VuiParametersParser::ParseVuiParameters` flattened fields.
int h265nal_vui_parameters_parse(
    const uint8_t* data,
    size_t len,
    uint32_t sps_max_sub_layers_minus1,
    h265nal_vui_parameters_fields* out_vui_parameters);

uint32_t h265nal_abi_version(void);

#ifdef __cplusplus
}
#endif

#endif  // H265_C_API_H_
