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

  FillProfileInfoFields(*ptl->general, &out_profile_tier_level->general);
  out_profile_tier_level->general_level_idc = ptl->general_level_idc;
  out_profile_tier_level->sub_layer_profile_present_flag_size =
      ptl->sub_layer_profile_present_flag.size();
  out_profile_tier_level->sub_layer_level_present_flag_size =
      ptl->sub_layer_level_present_flag.size();
  out_profile_tier_level->reserved_zero_2bits_size =
      ptl->reserved_zero_2bits.size();
  out_profile_tier_level->sub_layer_size = ptl->sub_layer.size();
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

uint32_t h265nal_abi_version(void) {
  return H265NAL_C_ABI_VERSION;
}
