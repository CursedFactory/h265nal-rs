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

// DIVERGENCE: flattened NAL unit fields for C ABI parity tests.
typedef struct h265nal_nal_unit_fields {
  size_t parsed_length;
  uint32_t forbidden_zero_bit;
  uint32_t nal_unit_type;
  uint32_t nuh_layer_id;
  uint32_t nuh_temporal_id_plus1;
} h265nal_nal_unit_fields;

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

// DIVERGENCE: SEI baseline fields for user-data-unregistered parity tests.
typedef struct h265nal_sei_message_fields {
  int32_t payload_type;
  uint32_t payload_size;
  uint32_t has_user_data_unregistered;
  uint64_t user_data_unregistered_uuid_iso_iec_11578_1;
  uint64_t user_data_unregistered_uuid_iso_iec_11578_2;
} h265nal_sei_message_fields;

// DIVERGENCE: flattened SPS multilayer extension fields for Rust parity tests.
typedef struct h265nal_sps_multilayer_extension_fields {
  uint32_t inter_view_mv_vert_constraint_flag;
} h265nal_sps_multilayer_extension_fields;

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

// DIVERGENCE: expose `H265SeiMessageParser::ParseSei` baseline fields.
int h265nal_sei_parse(const uint8_t* data,
                      size_t len,
                      h265nal_sei_message_fields* out_sei_message);

// DIVERGENCE: expose `H265SpsMultilayerExtensionParser::ParseSpsMultilayerExtension`.
int h265nal_sps_multilayer_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_sps_multilayer_extension_fields* out_sps_multilayer_extension);

uint32_t h265nal_abi_version(void);

#ifdef __cplusplus
}
#endif

#endif  // H265_C_API_H_
