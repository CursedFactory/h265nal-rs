#include "h265_c_api.h"

#include <cstdio>
#include <new>

#include "h265_aud_parser.h"
#include "h265_bitstream_parser.h"
#include "h265_common.h"
#include "h265_nal_unit_header_parser.h"
#include "h265_nal_unit_parser.h"
#include "h265_pps_parser.h"
#include "h265_profile_tier_level_parser.h"
#include "h265_sei_parser.h"
#include "h265_slice_parser.h"
#include "h265_sps_multilayer_extension_parser.h"
#include "h265_sps_parser.h"
#include "h265_vui_parameters_parser.h"
#include "h265_vps_parser.h"
#include "h265_utils.h"

struct h265nal_bitstream_parser_state {
  h265nal::H265BitstreamParserState state;
};

int h265nal_annexb_count_nalus(const uint8_t* data,
                               size_t len,
                               size_t* out_count) {
  if (out_count == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  const auto nalu_indices = h265nal::H265BitstreamParser::FindNaluIndices(
      data, len);
  *out_count = nalu_indices.size();
  return H265NAL_STATUS_OK;
}

int h265nal_annexb_dump(const uint8_t* data, size_t len, uint32_t dump_flags) {
  if (data == nullptr && len > 0) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

#ifdef FDUMP_DEFINE
  const int indent_level =
      (dump_flags & H265NAL_DUMP_FLAG_ONE_LINE) != 0 ? -1 : 0;
  h265nal::ParsingOptions parsing_options;
  auto bitstream =
      h265nal::H265BitstreamParser::ParseBitstream(data, len, parsing_options);
  if (bitstream == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  bitstream->fdump(stdout, indent_level);
  return H265NAL_STATUS_OK;
#else
  (void)dump_flags;
  return H265NAL_STATUS_DUMP_UNAVAILABLE;
#endif
}

// DIVERGENCE: expose parser-state lifecycle for Rust parity tests.
int h265nal_bitstream_parser_state_create(
    h265nal_bitstream_parser_state** out_state) {
  if (out_state == nullptr) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto* state = new (std::nothrow) h265nal_bitstream_parser_state();
  if (state == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  *out_state = state;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose parser-state lifecycle for Rust parity tests.
int h265nal_bitstream_parser_state_free(h265nal_bitstream_parser_state* state) {
  if (state == nullptr) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  delete state;
  return H265NAL_STATUS_OK;
}

int h265nal_common_is_slice_segment(uint32_t nal_unit_type,
                                    uint32_t* out_is_slice_segment) {
  if (out_is_slice_segment == nullptr) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  *out_is_slice_segment = h265nal::IsSliceSegment(nal_unit_type) ? 1u : 0u;
  return H265NAL_STATUS_OK;
}

int h265nal_common_is_nal_unit_type_vcl(uint32_t nal_unit_type,
                                        uint32_t* out_is_vcl) {
  if (out_is_vcl == nullptr) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  *out_is_vcl = h265nal::IsNalUnitTypeVcl(nal_unit_type) ? 1u : 0u;
  return H265NAL_STATUS_OK;
}

int h265nal_common_is_nal_unit_type_non_vcl(uint32_t nal_unit_type,
                                            uint32_t* out_is_non_vcl) {
  if (out_is_non_vcl == nullptr) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  *out_is_non_vcl = h265nal::IsNalUnitTypeNonVcl(nal_unit_type) ? 1u : 0u;
  return H265NAL_STATUS_OK;
}

int h265nal_common_is_nal_unit_type_unspecified(
    uint32_t nal_unit_type,
    uint32_t* out_is_unspecified) {
  if (out_is_unspecified == nullptr) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  *out_is_unspecified =
      h265nal::IsNalUnitTypeUnspecified(nal_unit_type) ? 1u : 0u;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `more_rbsp_data` with explicit cursor I/O for parity tests.
int h265nal_common_more_rbsp_data(const uint8_t* data,
                                  size_t len,
                                  size_t byte_offset,
                                  size_t bit_offset,
                                  uint32_t* out_has_more,
                                  size_t* out_byte_offset,
                                  size_t* out_bit_offset) {
  if (out_has_more == nullptr || out_byte_offset == nullptr ||
      out_bit_offset == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  uint8_t empty_data = 0;
  const uint8_t* bytes = (data == nullptr) ? &empty_data : data;

  h265nal::BitBuffer bit_buffer(bytes, len);
  if (!bit_buffer.Seek(byte_offset, bit_offset)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  *out_has_more = h265nal::more_rbsp_data(&bit_buffer) ? 1u : 0u;
  bit_buffer.GetCurrentOffset(out_byte_offset, out_bit_offset);
  return H265NAL_STATUS_OK;
}

int h265nal_aud_parse_pic_type(const uint8_t* data,
                               size_t len,
                               uint32_t* out_pic_type) {
  if (out_pic_type == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto aud = h265nal::H265AudParser::ParseAud(data, len);
  if (aud == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  *out_pic_type = aud->pic_type;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265Utils::GetSliceQpY` as a C ABI helper.
int h265nal_utils_get_slice_qp_y(const uint8_t* data,
                                 size_t len,
                                 h265nal_bitstream_parser_state* state,
                                 int32_t* out_values,
                                 size_t out_capacity,
                                 size_t* out_count) {
  if (state == nullptr || out_count == nullptr ||
      (data == nullptr && len > 0) ||
      (out_values == nullptr && out_capacity > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto values = h265nal::H265Utils::GetSliceQpY(data, len, &state->state);
  *out_count = values.size();
  if (out_capacity < values.size()) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  for (size_t i = 0; i < values.size(); ++i) {
    out_values[i] = values[i];
  }
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265NalUnitHeaderParser::GetNalUnitType` helper.
int h265nal_nal_unit_header_get_nal_unit_type(const uint8_t* data,
                                              size_t len,
                                              uint32_t* out_nal_unit_type) {
  if (out_nal_unit_type == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  h265nal::NalUnitType nal_unit_type;
  if (!h265nal::H265NalUnitHeaderParser::GetNalUnitType(data, len,
                                                         nal_unit_type)) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  *out_nal_unit_type = static_cast<uint32_t>(nal_unit_type);
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265NalUnitParser::ParseNalUnit` core fields.
int h265nal_nal_unit_parse(const uint8_t* data,
                           size_t len,
                           h265nal_bitstream_parser_state* state,
                           uint32_t add_checksum,
                           h265nal_nal_unit_fields* out_nal_unit) {
  if (state == nullptr || out_nal_unit == nullptr ||
      (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  h265nal::ParsingOptions parsing_options;
  parsing_options.add_checksum = add_checksum != 0;

  auto nal_unit = h265nal::H265NalUnitParser::ParseNalUnit(
      data, len, &state->state, parsing_options);
  if (nal_unit == nullptr || nal_unit->nal_unit_header == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_nal_unit->parsed_length = nal_unit->parsed_length;
  out_nal_unit->forbidden_zero_bit = nal_unit->nal_unit_header->forbidden_zero_bit;
  out_nal_unit->nal_unit_type = nal_unit->nal_unit_header->nal_unit_type;
  out_nal_unit->nuh_layer_id = nal_unit->nal_unit_header->nuh_layer_id;
  out_nal_unit->nuh_temporal_id_plus1 =
      nal_unit->nal_unit_header->nuh_temporal_id_plus1;
  return H265NAL_STATUS_OK;
}

namespace {

void FillProfileInfoFields(
    const h265nal::H265ProfileInfoParser::ProfileInfoState& profile,
    h265nal_profile_info_fields* out_profile) {
  out_profile->profile_space = profile.profile_space;
  out_profile->tier_flag = profile.tier_flag;
  out_profile->profile_idc = profile.profile_idc;
  for (size_t i = 0; i < profile.profile_compatibility_flag.size(); ++i) {
    out_profile->profile_compatibility_flag[i] =
        profile.profile_compatibility_flag[i];
  }
  out_profile->progressive_source_flag = profile.progressive_source_flag;
  out_profile->interlaced_source_flag = profile.interlaced_source_flag;
  out_profile->non_packed_constraint_flag = profile.non_packed_constraint_flag;
  out_profile->frame_only_constraint_flag = profile.frame_only_constraint_flag;
  out_profile->max_12bit_constraint_flag = profile.max_12bit_constraint_flag;
  out_profile->max_10bit_constraint_flag = profile.max_10bit_constraint_flag;
  out_profile->max_8bit_constraint_flag = profile.max_8bit_constraint_flag;
  out_profile->max_422chroma_constraint_flag =
      profile.max_422chroma_constraint_flag;
  out_profile->max_420chroma_constraint_flag =
      profile.max_420chroma_constraint_flag;
  out_profile->max_monochrome_constraint_flag =
      profile.max_monochrome_constraint_flag;
  out_profile->intra_constraint_flag = profile.intra_constraint_flag;
  out_profile->one_picture_only_constraint_flag =
      profile.one_picture_only_constraint_flag;
  out_profile->lower_bit_rate_constraint_flag =
      profile.lower_bit_rate_constraint_flag;
  out_profile->max_14bit_constraint_flag = profile.max_14bit_constraint_flag;
  out_profile->reserved_zero_33bits = profile.reserved_zero_33bits;
  out_profile->reserved_zero_34bits = profile.reserved_zero_34bits;
  out_profile->reserved_zero_43bits = profile.reserved_zero_43bits;
  out_profile->inbld_flag = profile.inbld_flag;
  out_profile->reserved_zero_bit = profile.reserved_zero_bit;
}

void FillProfileTierLevelFields(
    const h265nal::H265ProfileTierLevelParser::ProfileTierLevelState& ptl,
    h265nal_profile_tier_level_fields* out_profile_tier_level) {
  FillProfileInfoFields(*ptl.general, &out_profile_tier_level->general);
  out_profile_tier_level->general_level_idc = ptl.general_level_idc;
  out_profile_tier_level->sub_layer_profile_present_flag_size =
      ptl.sub_layer_profile_present_flag.size();
  out_profile_tier_level->sub_layer_level_present_flag_size =
      ptl.sub_layer_level_present_flag.size();
  out_profile_tier_level->reserved_zero_2bits_size =
      ptl.reserved_zero_2bits.size();
  out_profile_tier_level->sub_layer_size = ptl.sub_layer.size();
}

}  // namespace

// DIVERGENCE: expose `H265ProfileTierLevelParser::ParseProfileTierLevel`.
int h265nal_profile_tier_level_parse(
    const uint8_t* data,
    size_t len,
    uint32_t profile_present_flag,
    uint32_t max_num_sub_layers_minus1,
    h265nal_profile_tier_level_fields* out_profile_tier_level) {
  if (out_profile_tier_level == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto ptl = h265nal::H265ProfileTierLevelParser::ParseProfileTierLevel(
      data, len, profile_present_flag != 0,
      static_cast<unsigned int>(max_num_sub_layers_minus1));
  if (ptl == nullptr || ptl->general == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  FillProfileTierLevelFields(*ptl, out_profile_tier_level);
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265PpsParser::ParsePps` core fields.
int h265nal_pps_parse(const uint8_t* data,
                      size_t len,
                      h265nal_pps_fields* out_pps) {
  if (out_pps == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto pps = h265nal::H265PpsParser::ParsePps(data, len);
  if (pps == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_pps->pps_pic_parameter_set_id = pps->pps_pic_parameter_set_id;
  out_pps->pps_seq_parameter_set_id = pps->pps_seq_parameter_set_id;
  out_pps->dependent_slice_segments_enabled_flag =
      pps->dependent_slice_segments_enabled_flag;
  out_pps->output_flag_present_flag = pps->output_flag_present_flag;
  out_pps->num_extra_slice_header_bits = pps->num_extra_slice_header_bits;
  out_pps->sign_data_hiding_enabled_flag = pps->sign_data_hiding_enabled_flag;
  out_pps->cabac_init_present_flag = pps->cabac_init_present_flag;
  out_pps->num_ref_idx_l0_default_active_minus1 =
      pps->num_ref_idx_l0_default_active_minus1;
  out_pps->num_ref_idx_l1_default_active_minus1 =
      pps->num_ref_idx_l1_default_active_minus1;
  out_pps->init_qp_minus26 = pps->init_qp_minus26;
  out_pps->constrained_intra_pred_flag = pps->constrained_intra_pred_flag;
  out_pps->transform_skip_enabled_flag = pps->transform_skip_enabled_flag;
  out_pps->cu_qp_delta_enabled_flag = pps->cu_qp_delta_enabled_flag;
  out_pps->diff_cu_qp_delta_depth = pps->diff_cu_qp_delta_depth;
  out_pps->pps_cb_qp_offset = pps->pps_cb_qp_offset;
  out_pps->pps_cr_qp_offset = pps->pps_cr_qp_offset;
  out_pps->pps_slice_chroma_qp_offsets_present_flag =
      pps->pps_slice_chroma_qp_offsets_present_flag;
  out_pps->weighted_pred_flag = pps->weighted_pred_flag;
  out_pps->weighted_bipred_flag = pps->weighted_bipred_flag;
  out_pps->transquant_bypass_enabled_flag = pps->transquant_bypass_enabled_flag;
  out_pps->tiles_enabled_flag = pps->tiles_enabled_flag;
  out_pps->entropy_coding_sync_enabled_flag =
      pps->entropy_coding_sync_enabled_flag;
  out_pps->pps_loop_filter_across_slices_enabled_flag =
      pps->pps_loop_filter_across_slices_enabled_flag;
  out_pps->deblocking_filter_control_present_flag =
      pps->deblocking_filter_control_present_flag;
  out_pps->pps_scaling_list_data_present_flag =
      pps->pps_scaling_list_data_present_flag;
  out_pps->lists_modification_present_flag = pps->lists_modification_present_flag;
  out_pps->log2_parallel_merge_level_minus2 =
      pps->log2_parallel_merge_level_minus2;
  out_pps->slice_segment_header_extension_present_flag =
      pps->slice_segment_header_extension_present_flag;
  out_pps->pps_extension_present_flag = pps->pps_extension_present_flag;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265SpsParser::ParseSps` scalar/derived fields.
int h265nal_sps_parse(const uint8_t* data,
                      size_t len,
                      h265nal_sps_fields* out_sps) {
  if (out_sps == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto sps = h265nal::H265SpsParser::ParseSps(data, len);
  if (sps == nullptr || sps->profile_tier_level == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_sps->sps_video_parameter_set_id = sps->sps_video_parameter_set_id;
  out_sps->sps_max_sub_layers_minus1 = sps->sps_max_sub_layers_minus1;
  out_sps->sps_temporal_id_nesting_flag = sps->sps_temporal_id_nesting_flag;
  FillProfileTierLevelFields(*sps->profile_tier_level, &out_sps->profile_tier_level);
  out_sps->sps_seq_parameter_set_id = sps->sps_seq_parameter_set_id;
  out_sps->chroma_format_idc = sps->chroma_format_idc;
  out_sps->pic_width_in_luma_samples = sps->pic_width_in_luma_samples;
  out_sps->pic_height_in_luma_samples = sps->pic_height_in_luma_samples;
  out_sps->conformance_window_flag = sps->conformance_window_flag;
  out_sps->conf_win_left_offset = sps->conf_win_left_offset;
  out_sps->conf_win_right_offset = sps->conf_win_right_offset;
  out_sps->conf_win_top_offset = sps->conf_win_top_offset;
  out_sps->conf_win_bottom_offset = sps->conf_win_bottom_offset;
  out_sps->bit_depth_luma_minus8 = sps->bit_depth_luma_minus8;
  out_sps->bit_depth_chroma_minus8 = sps->bit_depth_chroma_minus8;
  out_sps->log2_max_pic_order_cnt_lsb_minus4 = sps->log2_max_pic_order_cnt_lsb_minus4;
  out_sps->sps_sub_layer_ordering_info_present_flag =
      sps->sps_sub_layer_ordering_info_present_flag;
  out_sps->sps_max_dec_pic_buffering_minus1_0 =
      sps->sps_max_dec_pic_buffering_minus1.empty()
          ? 0
          : sps->sps_max_dec_pic_buffering_minus1[0];
  out_sps->sps_max_num_reorder_pics_0 = sps->sps_max_num_reorder_pics.empty()
                                           ? 0
                                           : sps->sps_max_num_reorder_pics[0];
  out_sps->sps_max_latency_increase_plus1_0 =
      sps->sps_max_latency_increase_plus1.empty()
          ? 0
          : sps->sps_max_latency_increase_plus1[0];
  out_sps->scaling_list_enabled_flag = sps->scaling_list_enabled_flag;
  out_sps->amp_enabled_flag = sps->amp_enabled_flag;
  out_sps->sample_adaptive_offset_enabled_flag =
      sps->sample_adaptive_offset_enabled_flag;
  out_sps->pcm_enabled_flag = sps->pcm_enabled_flag;
  out_sps->num_short_term_ref_pic_sets = sps->num_short_term_ref_pic_sets;
  out_sps->long_term_ref_pics_present_flag = sps->long_term_ref_pics_present_flag;
  out_sps->sps_temporal_mvp_enabled_flag = sps->sps_temporal_mvp_enabled_flag;
  out_sps->strong_intra_smoothing_enabled_flag =
      sps->strong_intra_smoothing_enabled_flag;
  out_sps->vui_parameters_present_flag = sps->vui_parameters_present_flag;
  if (sps->vui_parameters != nullptr) {
    out_sps->vui_video_signal_type_present_flag =
        sps->vui_parameters->video_signal_type_present_flag;
    out_sps->vui_video_format = sps->vui_parameters->video_format;
    out_sps->vui_video_full_range_flag = sps->vui_parameters->video_full_range_flag;
    out_sps->vui_colour_description_present_flag =
        sps->vui_parameters->colour_description_present_flag;
    out_sps->vui_colour_primaries = sps->vui_parameters->colour_primaries;
    out_sps->vui_transfer_characteristics =
        sps->vui_parameters->transfer_characteristics;
    out_sps->vui_matrix_coeffs = sps->vui_parameters->matrix_coeffs;
    out_sps->vui_bitstream_restriction_flag =
        sps->vui_parameters->bitstream_restriction_flag;
    out_sps->vui_max_bytes_per_pic_denom =
        sps->vui_parameters->max_bytes_per_pic_denom;
    out_sps->vui_max_bits_per_min_cu_denom =
        sps->vui_parameters->max_bits_per_min_cu_denom;
    out_sps->vui_log2_max_mv_length_horizontal =
        sps->vui_parameters->log2_max_mv_length_horizontal;
    out_sps->vui_log2_max_mv_length_vertical =
        sps->vui_parameters->log2_max_mv_length_vertical;
  } else {
    out_sps->vui_video_signal_type_present_flag = 0;
    out_sps->vui_video_format = 0;
    out_sps->vui_video_full_range_flag = 0;
    out_sps->vui_colour_description_present_flag = 0;
    out_sps->vui_colour_primaries = 0;
    out_sps->vui_transfer_characteristics = 0;
    out_sps->vui_matrix_coeffs = 0;
    out_sps->vui_bitstream_restriction_flag = 0;
    out_sps->vui_max_bytes_per_pic_denom = 0;
    out_sps->vui_max_bits_per_min_cu_denom = 0;
    out_sps->vui_log2_max_mv_length_horizontal = 0;
    out_sps->vui_log2_max_mv_length_vertical = 0;
  }
  out_sps->sps_extension_present_flag = sps->sps_extension_present_flag;
  out_sps->sps_range_extension_flag = sps->sps_range_extension_flag;
  out_sps->sps_multilayer_extension_flag = sps->sps_multilayer_extension_flag;
  out_sps->sps_3d_extension_flag = sps->sps_3d_extension_flag;
  out_sps->sps_scc_extension_flag = sps->sps_scc_extension_flag;
  out_sps->sps_extension_4bits = sps->sps_extension_4bits;
  out_sps->pic_size_in_ctbs_y = sps->getPicSizeInCtbsY();
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose state seeding helpers for slice parser parity tests.
int h265nal_bitstream_parser_state_seed_vps(h265nal_bitstream_parser_state* state,
                                            uint32_t vps_id) {
  if (state == nullptr) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }
  auto vps = std::make_shared<h265nal::H265VpsParser::VpsState>();
  state->state.vps[vps_id] = vps;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose state seeding helpers for slice parser parity tests.
int h265nal_bitstream_parser_state_seed_sps(h265nal_bitstream_parser_state* state,
                                            uint32_t sps_id,
                                            uint32_t sample_adaptive_offset_enabled_flag,
                                            uint32_t chroma_format_idc,
                                            uint32_t log2_min_luma_coding_block_size_minus3,
                                            uint32_t log2_diff_max_min_luma_coding_block_size,
                                            uint32_t pic_width_in_luma_samples,
                                            uint32_t pic_height_in_luma_samples) {
  if (state == nullptr) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }
  auto sps = std::make_shared<h265nal::H265SpsParser::SpsState>();
  sps->sample_adaptive_offset_enabled_flag = sample_adaptive_offset_enabled_flag;
  sps->chroma_format_idc = chroma_format_idc;
  sps->log2_min_luma_coding_block_size_minus3 =
      log2_min_luma_coding_block_size_minus3;
  sps->log2_diff_max_min_luma_coding_block_size =
      log2_diff_max_min_luma_coding_block_size;
  sps->pic_width_in_luma_samples = pic_width_in_luma_samples;
  sps->pic_height_in_luma_samples = pic_height_in_luma_samples;
  state->state.sps[sps_id] = sps;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose state seeding helpers for slice parser parity tests.
int h265nal_bitstream_parser_state_seed_pps(h265nal_bitstream_parser_state* state,
                                            uint32_t pps_id) {
  if (state == nullptr) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }
  auto pps = std::make_shared<h265nal::H265PpsParser::PpsState>();
  state->state.pps[pps_id] = pps;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265SliceSegmentLayerParser::ParseSliceSegmentLayer` fields.
int h265nal_slice_segment_layer_parse(
    const uint8_t* data,
    size_t len,
    uint32_t nal_unit_type,
    h265nal_bitstream_parser_state* state,
    h265nal_slice_segment_layer_fields* out_slice_segment_layer) {
  if (state == nullptr || out_slice_segment_layer == nullptr ||
      (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto slice = h265nal::H265SliceSegmentLayerParser::ParseSliceSegmentLayer(
      data, len, nal_unit_type, &state->state);
  if (slice == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_slice_segment_layer->has_slice_segment_header =
      slice->slice_segment_header == nullptr ? 0u : 1u;
  if (slice->slice_segment_header == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  auto& header = slice->slice_segment_header;
  out_slice_segment_layer->nal_unit_type = header->nal_unit_type;
  out_slice_segment_layer->first_slice_segment_in_pic_flag =
      header->first_slice_segment_in_pic_flag;
  out_slice_segment_layer->no_output_of_prior_pics_flag =
      header->no_output_of_prior_pics_flag;
  out_slice_segment_layer->slice_pic_parameter_set_id =
      header->slice_pic_parameter_set_id;
  out_slice_segment_layer->slice_segment_address = header->slice_segment_address;
  out_slice_segment_layer->slice_type = header->slice_type;
  out_slice_segment_layer->slice_sao_luma_flag = header->slice_sao_luma_flag;
  out_slice_segment_layer->slice_sao_chroma_flag = header->slice_sao_chroma_flag;
  out_slice_segment_layer->slice_qp_delta = header->slice_qp_delta;
  out_slice_segment_layer->num_entry_point_offsets =
      header->num_entry_point_offsets;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265SeiMessageParser::ParseSei` baseline fields.
int h265nal_sei_parse(const uint8_t* data,
                      size_t len,
                      h265nal_sei_message_fields* out_sei_message) {
  if (out_sei_message == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto sei = h265nal::H265SeiMessageParser::ParseSei(data, len);
  if (sei == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_sei_message->payload_type = static_cast<int32_t>(sei->payload_type);
  out_sei_message->payload_size = sei->payload_size;
  out_sei_message->has_user_data_unregistered = 0;
  out_sei_message->user_data_unregistered_uuid_iso_iec_11578_1 = 0;
  out_sei_message->user_data_unregistered_uuid_iso_iec_11578_2 = 0;

  auto* user_data_unregistered = dynamic_cast<
      h265nal::H265SeiUserDataUnregisteredParser::H265SeiUserDataUnregisteredState*>(
      sei->payload_state.get());
  if (user_data_unregistered != nullptr) {
    out_sei_message->has_user_data_unregistered = 1;
    out_sei_message->user_data_unregistered_uuid_iso_iec_11578_1 =
        user_data_unregistered->uuid_iso_iec_11578_1;
    out_sei_message->user_data_unregistered_uuid_iso_iec_11578_2 =
        user_data_unregistered->uuid_iso_iec_11578_2;
  }
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265SpsMultilayerExtensionParser::ParseSpsMultilayerExtension`.
int h265nal_sps_multilayer_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_sps_multilayer_extension_fields* out_sps_multilayer_extension) {
  if (out_sps_multilayer_extension == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto extension =
      h265nal::H265SpsMultilayerExtensionParser::ParseSpsMultilayerExtension(
          data, len);
  if (extension == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_sps_multilayer_extension->inter_view_mv_vert_constraint_flag =
      extension->inter_view_mv_vert_constraint_flag;
  return H265NAL_STATUS_OK;
}

uint32_t h265nal_abi_version(void) {
  return H265NAL_C_ABI_VERSION;
}
