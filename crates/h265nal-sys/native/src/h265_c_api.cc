#include "h265_c_api.h"

#include <cstdio>
#include <new>

#include "h265_aud_parser.h"
#include "h265_bitstream_parser.h"
#include "h265_common.h"
#include "h265_configuration_box_parser.h"
#include "h265_hrd_parameters_parser.h"
#include "h265_nal_unit_header_parser.h"
#include "h265_nal_unit_parser.h"
#include "h265_pps_parser.h"
#include "h265_pps_multilayer_extension_parser.h"
#include "h265_pps_scc_extension_parser.h"
#include "h265_pred_weight_table_parser.h"
#include "h265_profile_tier_level_parser.h"
#include "h265_rtp_parser.h"
#include "h265_scaling_list_data_parser.h"
#include "h265_sei_parser.h"
#include "h265_slice_parser.h"
#include "h265_sps_3d_extension_parser.h"
#include "h265_sub_layer_hrd_parameters_parser.h"
#include "h265_sps_multilayer_extension_parser.h"
#include "h265_sps_parser.h"
#include "h265_sps_range_extension_parser.h"
#include "h265_sps_scc_extension_parser.h"
#include "h265_st_ref_pic_set_parser.h"
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
  parsing_options.add_offset =
      (dump_flags & H265NAL_DUMP_FLAG_ADD_OFFSET) != 0;
  parsing_options.add_length =
      (dump_flags & H265NAL_DUMP_FLAG_ADD_LENGTH) != 0;
  parsing_options.add_parsed_length =
      (dump_flags & H265NAL_DUMP_FLAG_ADD_PARSED_LENGTH) != 0;
  parsing_options.add_checksum =
      (dump_flags & H265NAL_DUMP_FLAG_ADD_CHECKSUM) != 0;
  parsing_options.add_resolution =
      (dump_flags & H265NAL_DUMP_FLAG_ADD_RESOLUTION) != 0;
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

namespace {

int FillBitstreamNals(
    const std::unique_ptr<h265nal::H265BitstreamParser::BitstreamState>& bitstream,
    h265nal_bitstream_nal_fields* out_nals,
    size_t out_capacity,
    size_t* out_count) {
  *out_count = bitstream->nal_units.size();
  if (out_nals == nullptr && out_capacity == 0) {
    return H265NAL_STATUS_OK;
  }
  if (out_capacity < bitstream->nal_units.size()) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  for (size_t i = 0; i < bitstream->nal_units.size(); ++i) {
    const auto& nal_unit = bitstream->nal_units[i];
    if (nal_unit == nullptr || nal_unit->nal_unit_header == nullptr) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }

    out_nals[i].offset = nal_unit->offset;
    out_nals[i].length = nal_unit->length;
    out_nals[i].parsed_length = nal_unit->parsed_length;
    out_nals[i].checksum_size = 0;
    out_nals[i].checksum[0] = 0;
    out_nals[i].checksum[1] = 0;
    out_nals[i].checksum[2] = 0;
    out_nals[i].checksum[3] = 0;
    out_nals[i].forbidden_zero_bit = nal_unit->nal_unit_header->forbidden_zero_bit;
    out_nals[i].nal_unit_type = nal_unit->nal_unit_header->nal_unit_type;
    out_nals[i].nuh_layer_id = nal_unit->nal_unit_header->nuh_layer_id;
    out_nals[i].nuh_temporal_id_plus1 =
        nal_unit->nal_unit_header->nuh_temporal_id_plus1;
    out_nals[i].has_slice_segment_header = 0;
    out_nals[i].first_slice_segment_in_pic_flag = 0;
    out_nals[i].slice_segment_address = 0;
    out_nals[i].slice_pic_order_cnt_lsb = 0;

    if (nal_unit->nal_unit_payload != nullptr &&
        nal_unit->nal_unit_payload->slice_segment_layer != nullptr &&
        nal_unit->nal_unit_payload->slice_segment_layer->slice_segment_header != nullptr) {
      const auto& slice_header =
          nal_unit->nal_unit_payload->slice_segment_layer->slice_segment_header;
      out_nals[i].has_slice_segment_header = 1;
      out_nals[i].first_slice_segment_in_pic_flag =
          slice_header->first_slice_segment_in_pic_flag;
      out_nals[i].slice_segment_address = slice_header->slice_segment_address;
      out_nals[i].slice_pic_order_cnt_lsb = slice_header->slice_pic_order_cnt_lsb;
    }

    if (nal_unit->checksum != nullptr) {
      const size_t checksum_size = nal_unit->checksum->GetLength();
      out_nals[i].checksum_size = checksum_size;
      const char* checksum = nal_unit->checksum->GetChecksum();
      for (size_t j = 0; j < checksum_size && j < 4; ++j) {
        out_nals[i].checksum[j] = static_cast<uint8_t>(checksum[j]);
      }
    }
  }

  return H265NAL_STATUS_OK;
}

}  // namespace

// DIVERGENCE: expose `H265BitstreamParser::ParseBitstream` metadata/checksum.
int h265nal_bitstream_parse(const uint8_t* data,
                            size_t len,
                            h265nal_bitstream_parser_state* state,
                            uint32_t add_checksum,
                            h265nal_bitstream_nal_fields* out_nals,
                            size_t out_capacity,
                            size_t* out_count) {
  if (out_count == nullptr || (data == nullptr && len > 0) ||
      (out_nals == nullptr && out_capacity > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  h265nal::ParsingOptions parsing_options;
  parsing_options.add_checksum = add_checksum != 0;

  std::unique_ptr<h265nal::H265BitstreamParser::BitstreamState> bitstream;
  if (state == nullptr) {
    bitstream = h265nal::H265BitstreamParser::ParseBitstream(
        data, len, parsing_options);
  } else {
    bitstream = h265nal::H265BitstreamParser::ParseBitstream(
        data, len, &state->state, parsing_options);
  }
  if (bitstream == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  return FillBitstreamNals(bitstream, out_nals, out_capacity, out_count);
}

// DIVERGENCE: expose `H265BitstreamParser::ParseBitstreamNALULength` metadata/checksum.
int h265nal_bitstream_parse_nalu_length(const uint8_t* data,
                                        size_t len,
                                        size_t nalu_length_bytes,
                                        h265nal_bitstream_parser_state* state,
                                        uint32_t add_checksum,
                                        h265nal_bitstream_nal_fields* out_nals,
                                        size_t out_capacity,
                                        size_t* out_count) {
  if (out_count == nullptr || (data == nullptr && len > 0) ||
      (out_nals == nullptr && out_capacity > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  h265nal::ParsingOptions parsing_options;
  parsing_options.add_checksum = add_checksum != 0;

  std::unique_ptr<h265nal::H265BitstreamParser::BitstreamState> bitstream;
  if (state == nullptr) {
    bitstream = h265nal::H265BitstreamParser::ParseBitstreamNALULength(
        data, len, nalu_length_bytes, parsing_options);
  } else {
    bitstream = h265nal::H265BitstreamParser::ParseBitstreamNALULength(
        data, len, nalu_length_bytes, &state->state, parsing_options);
  }
  if (bitstream == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  return FillBitstreamNals(bitstream, out_nals, out_capacity, out_count);
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

  out_nal_unit->checksum_size = 0;
  out_nal_unit->checksum[0] = 0;
  out_nal_unit->checksum[1] = 0;
  out_nal_unit->checksum[2] = 0;
  out_nal_unit->checksum[3] = 0;
  if (nal_unit->checksum != nullptr) {
    const size_t checksum_size = nal_unit->checksum->GetLength();
    out_nal_unit->checksum_size = checksum_size;
    const char* checksum = nal_unit->checksum->GetChecksum();
    for (size_t i = 0; i < checksum_size && i < 4; ++i) {
      out_nal_unit->checksum[i] = static_cast<uint8_t>(checksum[i]);
    }
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

constexpr size_t kSubLayerHrdVectorCapacity = 32;
constexpr size_t kLayerVectorCapacity = 8;
constexpr size_t kRtpApTypeCapacity = 8;
constexpr size_t kScalingListSizeIdCount = 4;
constexpr size_t kScalingListMatrixIdCount = 6;
constexpr size_t kScalingListFlattenedCapacity =
    kScalingListSizeIdCount * kScalingListMatrixIdCount;
constexpr size_t kSps3dFlagCapacity = 8;
constexpr size_t kStRefPicSetCapacity = 16;
constexpr size_t kPredWeightTableFlagCapacity = 16;
constexpr size_t kConfigurationBoxArrayCapacity = 8;

size_t ScalingListIndex(size_t size_id, size_t matrix_id) {
  return size_id * kScalingListMatrixIdCount + matrix_id;
}

int CopyU32VectorToCallerBuffer(const std::vector<uint32_t>& values,
                                uint32_t* out_values,
                                size_t out_capacity,
                                size_t* out_count) {
  *out_count = values.size();
  if (out_values == nullptr) {
    return out_capacity == 0 ? H265NAL_STATUS_OK : H265NAL_STATUS_INVALID_ARGUMENT;
  }
  if (out_capacity < values.size()) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }
  for (size_t i = 0; i < values.size(); ++i) {
    out_values[i] = values[i];
  }
  return H265NAL_STATUS_OK;
}

int CopyU8VectorToCallerBuffer(const std::vector<uint8_t>& values,
                               uint8_t* out_values,
                               size_t out_capacity,
                               size_t* out_count) {
  *out_count = values.size();
  if (out_values == nullptr) {
    return out_capacity == 0 ? H265NAL_STATUS_OK : H265NAL_STATUS_INVALID_ARGUMENT;
  }
  if (out_capacity < values.size()) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }
  for (size_t i = 0; i < values.size(); ++i) {
    out_values[i] = values[i];
  }
  return H265NAL_STATUS_OK;
}

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

bool FillSubLayerHrdParametersFields(
    const h265nal::H265SubLayerHrdParametersParser::SubLayerHrdParametersState&
        sub_layer_hrd,
    h265nal_sub_layer_hrd_parameters_fields* out_sub_layer_hrd) {
  out_sub_layer_hrd->sub_layer_id = sub_layer_hrd.subLayerId;
  out_sub_layer_hrd->cpb_cnt = sub_layer_hrd.CpbCnt;
  out_sub_layer_hrd->sub_pic_hrd_params_present_flag =
      sub_layer_hrd.sub_pic_hrd_params_present_flag;

  out_sub_layer_hrd->bit_rate_value_minus1_size =
      sub_layer_hrd.bit_rate_value_minus1.size();
  out_sub_layer_hrd->cpb_size_value_minus1_size =
      sub_layer_hrd.cpb_size_value_minus1.size();
  out_sub_layer_hrd->cpb_size_du_value_minus1_size =
      sub_layer_hrd.cpb_size_du_value_minus1.size();
  out_sub_layer_hrd->bit_rate_du_value_minus1_size =
      sub_layer_hrd.bit_rate_du_value_minus1.size();
  out_sub_layer_hrd->cbr_flag_size = sub_layer_hrd.cbr_flag.size();

  if (out_sub_layer_hrd->bit_rate_value_minus1_size > kSubLayerHrdVectorCapacity ||
      out_sub_layer_hrd->cpb_size_value_minus1_size > kSubLayerHrdVectorCapacity ||
      out_sub_layer_hrd->cpb_size_du_value_minus1_size > kSubLayerHrdVectorCapacity ||
      out_sub_layer_hrd->bit_rate_du_value_minus1_size > kSubLayerHrdVectorCapacity ||
      out_sub_layer_hrd->cbr_flag_size > kSubLayerHrdVectorCapacity) {
    return false;
  }

  for (size_t i = 0; i < out_sub_layer_hrd->bit_rate_value_minus1_size; ++i) {
    out_sub_layer_hrd->bit_rate_value_minus1[i] =
        sub_layer_hrd.bit_rate_value_minus1[i];
  }
  for (size_t i = 0; i < out_sub_layer_hrd->cpb_size_value_minus1_size; ++i) {
    out_sub_layer_hrd->cpb_size_value_minus1[i] =
        sub_layer_hrd.cpb_size_value_minus1[i];
  }
  for (size_t i = 0; i < out_sub_layer_hrd->cpb_size_du_value_minus1_size; ++i) {
    out_sub_layer_hrd->cpb_size_du_value_minus1[i] =
        sub_layer_hrd.cpb_size_du_value_minus1[i];
  }
  for (size_t i = 0; i < out_sub_layer_hrd->bit_rate_du_value_minus1_size; ++i) {
    out_sub_layer_hrd->bit_rate_du_value_minus1[i] =
        sub_layer_hrd.bit_rate_du_value_minus1[i];
  }
  for (size_t i = 0; i < out_sub_layer_hrd->cbr_flag_size; ++i) {
    out_sub_layer_hrd->cbr_flag[i] = sub_layer_hrd.cbr_flag[i];
  }
  return true;
}

bool FillSubLayerMask(
    const std::vector<uint32_t>& row,
    uint64_t* out_mask,
    uint32_t* out_width) {
  if (row.size() > 64) {
    return false;
  }
  uint64_t mask = 0;
  for (size_t i = 0; i < row.size(); ++i) {
    if (row[i] != 0) {
      mask |= (uint64_t{1} << i);
    }
  }
  *out_mask = mask;
  *out_width = static_cast<uint32_t>(row.size());
  return true;
}

bool FillHrdParametersFields(
    const h265nal::H265HrdParametersParser::HrdParametersState& hrd,
    uint32_t common_inf_present_flag,
    uint32_t max_num_sub_layers_minus1,
    h265nal_hrd_parameters_fields* out_hrd_parameters) {
  out_hrd_parameters->common_inf_present_flag = common_inf_present_flag;
  out_hrd_parameters->max_num_sub_layers_minus1 = max_num_sub_layers_minus1;
  out_hrd_parameters->nal_hrd_parameters_present_flag =
      hrd.nal_hrd_parameters_present_flag;
  out_hrd_parameters->vcl_hrd_parameters_present_flag =
      hrd.vcl_hrd_parameters_present_flag;
  out_hrd_parameters->sub_pic_hrd_params_present_flag =
      hrd.sub_pic_hrd_params_present_flag;
  out_hrd_parameters->tick_divisor_minus2 = hrd.tick_divisor_minus2;
  out_hrd_parameters->du_cpb_removal_delay_increment_length_minus1 =
      hrd.du_cpb_removal_delay_increment_length_minus1;
  out_hrd_parameters->sub_pic_cpb_params_in_pic_timing_sei_flag =
      hrd.sub_pic_cpb_params_in_pic_timing_sei_flag;
  out_hrd_parameters->dpb_output_delay_du_length_minus1 =
      hrd.dpb_output_delay_du_length_minus1;
  out_hrd_parameters->bit_rate_scale = hrd.bit_rate_scale;
  out_hrd_parameters->cpb_size_scale = hrd.cpb_size_scale;
  out_hrd_parameters->cpb_size_du_scale = hrd.cpb_size_du_scale;
  out_hrd_parameters->initial_cpb_removal_delay_length_minus1 =
      hrd.initial_cpb_removal_delay_length_minus1;
  out_hrd_parameters->au_cpb_removal_delay_length_minus1 =
      hrd.au_cpb_removal_delay_length_minus1;
  out_hrd_parameters->dpb_output_delay_length_minus1 =
      hrd.dpb_output_delay_length_minus1;

  out_hrd_parameters->fixed_pic_rate_general_flag_size =
      hrd.fixed_pic_rate_general_flag.size();
  out_hrd_parameters->fixed_pic_rate_within_cvs_flag_size =
      hrd.fixed_pic_rate_within_cvs_flag.size();
  out_hrd_parameters->elemental_duration_in_tc_minus1_size =
      hrd.elemental_duration_in_tc_minus1.size();
  out_hrd_parameters->low_delay_hrd_flag_size = hrd.low_delay_hrd_flag.size();
  out_hrd_parameters->cpb_cnt_minus1_size = hrd.cpb_cnt_minus1.size();

  if (out_hrd_parameters->fixed_pic_rate_general_flag_size > kLayerVectorCapacity ||
      out_hrd_parameters->fixed_pic_rate_within_cvs_flag_size >
          kLayerVectorCapacity ||
      out_hrd_parameters->elemental_duration_in_tc_minus1_size >
          kLayerVectorCapacity ||
      out_hrd_parameters->low_delay_hrd_flag_size > kLayerVectorCapacity ||
      out_hrd_parameters->cpb_cnt_minus1_size > kLayerVectorCapacity) {
    return false;
  }

  for (size_t i = 0; i < out_hrd_parameters->fixed_pic_rate_general_flag_size;
       ++i) {
    out_hrd_parameters->fixed_pic_rate_general_flag[i] =
        hrd.fixed_pic_rate_general_flag[i];
  }
  for (size_t i = 0; i < out_hrd_parameters->fixed_pic_rate_within_cvs_flag_size;
       ++i) {
    out_hrd_parameters->fixed_pic_rate_within_cvs_flag[i] =
        hrd.fixed_pic_rate_within_cvs_flag[i];
  }
  for (size_t i = 0; i < out_hrd_parameters->elemental_duration_in_tc_minus1_size;
       ++i) {
    out_hrd_parameters->elemental_duration_in_tc_minus1[i] =
        hrd.elemental_duration_in_tc_minus1[i];
  }
  for (size_t i = 0; i < out_hrd_parameters->low_delay_hrd_flag_size; ++i) {
    out_hrd_parameters->low_delay_hrd_flag[i] = hrd.low_delay_hrd_flag[i];
  }
  for (size_t i = 0; i < out_hrd_parameters->cpb_cnt_minus1_size; ++i) {
    out_hrd_parameters->cpb_cnt_minus1[i] = hrd.cpb_cnt_minus1[i];
  }

  out_hrd_parameters->sub_layer_hrd_parameters_vector_size =
      hrd.sub_layer_hrd_parameters_vector.size();
  if (out_hrd_parameters->sub_layer_hrd_parameters_vector_size >
      kLayerVectorCapacity) {
    return false;
  }
  out_hrd_parameters->sub_layer_hrd_parameters_0_present = 0;
  if (out_hrd_parameters->sub_layer_hrd_parameters_vector_size > 0 &&
      hrd.sub_layer_hrd_parameters_vector[0] != nullptr) {
    out_hrd_parameters->sub_layer_hrd_parameters_0_present = 1;
    if (!FillSubLayerHrdParametersFields(
            *hrd.sub_layer_hrd_parameters_vector[0],
            &out_hrd_parameters->sub_layer_hrd_parameters_0)) {
      return false;
    }
  }

  return true;
}

void FillRtpPayloadFields(
    const h265nal::H265NalUnitPayloadParser::NalUnitPayloadState* payload,
    h265nal_rtp_fields* out_rtp) {
  if (payload == nullptr) {
    return;
  }

  if (payload->sps != nullptr) {
    out_rtp->has_payload_sps = 1;
    out_rtp->payload_sps_seq_parameter_set_id =
        payload->sps->sps_seq_parameter_set_id;
    out_rtp->payload_sps_pic_width_in_luma_samples =
        payload->sps->pic_width_in_luma_samples;
    out_rtp->payload_sps_pic_height_in_luma_samples =
        payload->sps->pic_height_in_luma_samples;
  }

  if (payload->pps != nullptr) {
    out_rtp->has_payload_pps = 1;
    out_rtp->payload_pps_pic_parameter_set_id = payload->pps->pps_pic_parameter_set_id;
    out_rtp->payload_pps_init_qp_minus26 = payload->pps->init_qp_minus26;
  }

  if (payload->slice_segment_layer != nullptr &&
      payload->slice_segment_layer->slice_segment_header != nullptr) {
    const auto& header = payload->slice_segment_layer->slice_segment_header;
    out_rtp->has_payload_slice_segment_header = 1;
    out_rtp->payload_slice_nal_unit_type = header->nal_unit_type;
    out_rtp->payload_slice_first_slice_segment_in_pic_flag =
        header->first_slice_segment_in_pic_flag;
    out_rtp->payload_slice_no_output_of_prior_pics_flag =
        header->no_output_of_prior_pics_flag;
    out_rtp->payload_slice_pic_parameter_set_id =
        header->slice_pic_parameter_set_id;
    out_rtp->payload_slice_type = header->slice_type;
    out_rtp->payload_slice_qp_delta = header->slice_qp_delta;
  }
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

// DIVERGENCE: expose `H265ConfigurationBoxParser::ParseConfigurationBox` fields.
int h265nal_configuration_box_parse(
    const uint8_t* data,
    size_t len,
    h265nal_bitstream_parser_state* state,
    h265nal_configuration_box_fields* out_configuration_box) {
  if (state == nullptr || out_configuration_box == nullptr ||
      (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  h265nal::ParsingOptions parsing_options;
  auto configuration_box = h265nal::H265ConfigurationBoxParser::ParseConfigurationBox(
      data, len, &state->state, parsing_options);
  if (configuration_box == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_configuration_box->configuration_version =
      configuration_box->configurationVersion;
  out_configuration_box->general_profile_space =
      configuration_box->general_profile_space;
  out_configuration_box->general_tier_flag = configuration_box->general_tier_flag;
  out_configuration_box->general_profile_idc = configuration_box->general_profile_idc;
  for (size_t i = 0;
       i < configuration_box->general_profile_compatibility_flags.size(); ++i) {
    out_configuration_box->general_profile_compatibility_flag[i] =
        configuration_box->general_profile_compatibility_flags[i];
  }
  out_configuration_box->general_constraint_indicator_flags =
      configuration_box->general_constraint_indicator_flags;
  out_configuration_box->general_level_idc = configuration_box->general_level_idc;
  out_configuration_box->min_spatial_segmentation_idc =
      configuration_box->min_spatial_segmentation_idc;
  out_configuration_box->parallelism_type = configuration_box->parallelismType;
  out_configuration_box->chroma_format_idc = configuration_box->chroma_format_idc;
  out_configuration_box->bit_depth_luma_minus8 =
      configuration_box->bit_depth_luma_minus8;
  out_configuration_box->bit_depth_chroma_minus8 =
      configuration_box->bit_depth_chroma_minus8;
  out_configuration_box->avg_frame_rate = configuration_box->avgFrameRate;
  out_configuration_box->constant_frame_rate = configuration_box->constantFrameRate;
  out_configuration_box->num_temporal_layers = configuration_box->numTemporalLayers;
  out_configuration_box->temporal_id_nested = configuration_box->temporalIdNested;
  out_configuration_box->length_size_minus_one =
      configuration_box->lengthSizeMinusOne;
  out_configuration_box->num_of_arrays = configuration_box->numOfArrays;

  out_configuration_box->array_completeness_size =
      configuration_box->array_completeness.size();
  out_configuration_box->nal_unit_type_size =
      configuration_box->NAL_unit_type.size();
  out_configuration_box->num_nalus_size = configuration_box->numNalus.size();
  out_configuration_box->first_nal_unit_length_size =
      configuration_box->nalUnitLength.size();

  if (out_configuration_box->array_completeness_size >
          kConfigurationBoxArrayCapacity ||
      out_configuration_box->nal_unit_type_size > kConfigurationBoxArrayCapacity ||
      out_configuration_box->num_nalus_size > kConfigurationBoxArrayCapacity ||
      out_configuration_box->first_nal_unit_length_size >
          kConfigurationBoxArrayCapacity) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  for (size_t i = 0; i < kConfigurationBoxArrayCapacity; ++i) {
    out_configuration_box->array_completeness[i] = 0;
    out_configuration_box->nal_unit_type[i] = 0;
    out_configuration_box->num_nalus[i] = 0;
    out_configuration_box->first_nal_unit_length[i] = 0;
  }
  for (size_t i = 0; i < out_configuration_box->array_completeness_size; ++i) {
    out_configuration_box->array_completeness[i] =
        configuration_box->array_completeness[i];
  }
  for (size_t i = 0; i < out_configuration_box->nal_unit_type_size; ++i) {
    out_configuration_box->nal_unit_type[i] = configuration_box->NAL_unit_type[i];
  }
  for (size_t i = 0; i < out_configuration_box->num_nalus_size; ++i) {
    out_configuration_box->num_nalus[i] = configuration_box->numNalus[i];
  }
  for (size_t i = 0; i < out_configuration_box->first_nal_unit_length_size; ++i) {
    if (!configuration_box->nalUnitLength[i].empty()) {
      out_configuration_box->first_nal_unit_length[i] =
          configuration_box->nalUnitLength[i][0];
    }
  }

  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265PpsMultilayerExtensionParser::ParsePpsMultilayerExtension`.
int h265nal_pps_multilayer_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_pps_multilayer_extension_fields* out_pps_multilayer_extension) {
  if (out_pps_multilayer_extension == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto extension =
      h265nal::H265PpsMultilayerExtensionParser::ParsePpsMultilayerExtension(
          data, len);
  if (extension == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_pps_multilayer_extension->poc_reset_info_present_flag =
      extension->poc_reset_info_present_flag;
  out_pps_multilayer_extension->pps_infer_scaling_list_flag =
      extension->pps_infer_scaling_list_flag;
  out_pps_multilayer_extension->num_ref_loc_offsets =
      extension->num_ref_loc_offsets;
  out_pps_multilayer_extension->colour_mapping_enabled_flag =
      extension->colour_mapping_enabled_flag;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265PpsSccExtensionParser::ParsePpsSccExtension`.
int h265nal_pps_scc_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_pps_scc_extension_fields* out_pps_scc_extension) {
  if (out_pps_scc_extension == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto extension = h265nal::H265PpsSccExtensionParser::ParsePpsSccExtension(
      data, len);
  if (extension == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_pps_scc_extension->pps_curr_pic_ref_enabled_flag =
      extension->pps_curr_pic_ref_enabled_flag;
  out_pps_scc_extension->residual_adaptive_colour_transform_enabled_flag =
      extension->residual_adaptive_colour_transform_enabled_flag;
  out_pps_scc_extension->pps_palette_predictor_initializer_present_flag =
      extension->pps_palette_predictor_initializer_present_flag;
  out_pps_scc_extension->pps_palette_predictor_initializers_size =
      extension->pps_palette_predictor_initializers.size();
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265PredWeightTableParser::ParsePredWeightTable`.
int h265nal_pred_weight_table_parse(
    const uint8_t* data,
    size_t len,
    uint32_t chroma_array_type,
    uint32_t num_ref_idx_l0_active_minus1,
    h265nal_pred_weight_table_fields* out_pred_weight_table) {
  if (out_pred_weight_table == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto pred_weight_table = h265nal::H265PredWeightTableParser::ParsePredWeightTable(
      data, len, chroma_array_type, num_ref_idx_l0_active_minus1);
  if (pred_weight_table == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_pred_weight_table->chroma_array_type = chroma_array_type;
  out_pred_weight_table->num_ref_idx_l0_active_minus1 =
      num_ref_idx_l0_active_minus1;
  out_pred_weight_table->luma_log2_weight_denom =
      pred_weight_table->luma_log2_weight_denom;
  out_pred_weight_table->delta_chroma_log2_weight_denom =
      pred_weight_table->delta_chroma_log2_weight_denom;
  out_pred_weight_table->luma_weight_l0_flag_size =
      pred_weight_table->luma_weight_l0_flag.size();
  out_pred_weight_table->chroma_weight_l0_flag_size =
      pred_weight_table->chroma_weight_l0_flag.size();

  if (out_pred_weight_table->luma_weight_l0_flag_size >
          kPredWeightTableFlagCapacity ||
      out_pred_weight_table->chroma_weight_l0_flag_size >
          kPredWeightTableFlagCapacity) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  for (size_t i = 0; i < kPredWeightTableFlagCapacity; ++i) {
    out_pred_weight_table->luma_weight_l0_flag[i] = 0;
    out_pred_weight_table->chroma_weight_l0_flag[i] = 0;
  }
  for (size_t i = 0; i < out_pred_weight_table->luma_weight_l0_flag_size; ++i) {
    out_pred_weight_table->luma_weight_l0_flag[i] =
        pred_weight_table->luma_weight_l0_flag[i];
  }
  for (size_t i = 0; i < out_pred_weight_table->chroma_weight_l0_flag_size; ++i) {
    out_pred_weight_table->chroma_weight_l0_flag[i] =
        pred_weight_table->chroma_weight_l0_flag[i];
  }
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
  out_sps->st_ref_pic_set_size = sps->st_ref_pic_set.size();

  out_sps->pic_size_in_ctbs_y = sps->getPicSizeInCtbsY();
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose dynamic SPS st-ref-pic-set count for Rust parity tests.
int h265nal_sps_st_ref_pic_set_count(const uint8_t* data,
                                     size_t len,
                                     size_t* out_count) {
  if (out_count == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto sps = h265nal::H265SpsParser::ParseSps(data, len);
  if (sps == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  *out_count = sps->st_ref_pic_set.size();
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose dynamic SPS st-ref-pic-set scalar/size fields for Rust parity tests.
int h265nal_sps_st_ref_pic_set_get(
    const uint8_t* data,
    size_t len,
    size_t st_ref_pic_set_idx,
    h265nal_sps_st_ref_pic_set_fields* out_st_ref_pic_set) {
  if (out_st_ref_pic_set == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto sps = h265nal::H265SpsParser::ParseSps(data, len);
  if (sps == nullptr || st_ref_pic_set_idx >= sps->st_ref_pic_set.size()) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  const auto& st_ref_pic_set = sps->st_ref_pic_set[st_ref_pic_set_idx];
  if (st_ref_pic_set == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_st_ref_pic_set->inter_ref_pic_set_prediction_flag =
      st_ref_pic_set->inter_ref_pic_set_prediction_flag;
  out_st_ref_pic_set->delta_idx_minus1 = st_ref_pic_set->delta_idx_minus1;
  out_st_ref_pic_set->delta_rps_sign = st_ref_pic_set->delta_rps_sign;
  out_st_ref_pic_set->abs_delta_rps_minus1 = st_ref_pic_set->abs_delta_rps_minus1;
  out_st_ref_pic_set->num_negative_pics = st_ref_pic_set->num_negative_pics;
  out_st_ref_pic_set->num_positive_pics = st_ref_pic_set->num_positive_pics;
  out_st_ref_pic_set->used_by_curr_pic_flag_size =
      st_ref_pic_set->used_by_curr_pic_flag.size();
  out_st_ref_pic_set->use_delta_flag_size = st_ref_pic_set->use_delta_flag.size();
  out_st_ref_pic_set->delta_poc_s0_minus1_size =
      st_ref_pic_set->delta_poc_s0_minus1.size();
  out_st_ref_pic_set->used_by_curr_pic_s0_flag_size =
      st_ref_pic_set->used_by_curr_pic_s0_flag.size();
  out_st_ref_pic_set->delta_poc_s1_minus1_size =
      st_ref_pic_set->delta_poc_s1_minus1.size();
  out_st_ref_pic_set->used_by_curr_pic_s1_flag_size =
      st_ref_pic_set->used_by_curr_pic_s1_flag.size();
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose dynamic SPS st-ref-pic-set vectors for Rust parity tests.
int h265nal_sps_st_ref_pic_set_vector_get(const uint8_t* data,
                                          size_t len,
                                          size_t st_ref_pic_set_idx,
                                          uint32_t vector_kind,
                                          uint32_t* out_values,
                                          size_t out_capacity,
                                          size_t* out_count) {
  if (out_count == nullptr || (out_values == nullptr && out_capacity > 0) ||
      (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto sps = h265nal::H265SpsParser::ParseSps(data, len);
  if (sps == nullptr || st_ref_pic_set_idx >= sps->st_ref_pic_set.size()) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  const auto& st_ref_pic_set = sps->st_ref_pic_set[st_ref_pic_set_idx];
  if (st_ref_pic_set == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  const std::vector<uint32_t>* values = nullptr;
  switch (vector_kind) {
    case H265NAL_SPS_ST_REF_PIC_SET_VECTOR_USED_BY_CURR_PIC_FLAG:
      values = &st_ref_pic_set->used_by_curr_pic_flag;
      break;
    case H265NAL_SPS_ST_REF_PIC_SET_VECTOR_USE_DELTA_FLAG:
      values = &st_ref_pic_set->use_delta_flag;
      break;
    case H265NAL_SPS_ST_REF_PIC_SET_VECTOR_DELTA_POC_S0_MINUS1:
      values = &st_ref_pic_set->delta_poc_s0_minus1;
      break;
    case H265NAL_SPS_ST_REF_PIC_SET_VECTOR_USED_BY_CURR_PIC_S0_FLAG:
      values = &st_ref_pic_set->used_by_curr_pic_s0_flag;
      break;
    case H265NAL_SPS_ST_REF_PIC_SET_VECTOR_DELTA_POC_S1_MINUS1:
      values = &st_ref_pic_set->delta_poc_s1_minus1;
      break;
    case H265NAL_SPS_ST_REF_PIC_SET_VECTOR_USED_BY_CURR_PIC_S1_FLAG:
      values = &st_ref_pic_set->used_by_curr_pic_s1_flag;
      break;
    default:
      return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  return CopyU32VectorToCallerBuffer(*values, out_values, out_capacity, out_count);
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

// DIVERGENCE: expose `H265RtpParser::ParseRtp` flattened packet fields.
int h265nal_rtp_parse(const uint8_t* data,
                      size_t len,
                      h265nal_bitstream_parser_state* state,
                      h265nal_rtp_fields* out_rtp) {
  if (state == nullptr || out_rtp == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  *out_rtp = {};

  auto rtp = h265nal::H265RtpParser::ParseRtp(data, len, &state->state);
  if (rtp == nullptr || rtp->nal_unit_header == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_rtp->forbidden_zero_bit = rtp->nal_unit_header->forbidden_zero_bit;
  out_rtp->nal_unit_type = rtp->nal_unit_header->nal_unit_type;
  out_rtp->nuh_layer_id = rtp->nal_unit_header->nuh_layer_id;
  out_rtp->nuh_temporal_id_plus1 = rtp->nal_unit_header->nuh_temporal_id_plus1;

  if (rtp->rtp_single != nullptr) {
    out_rtp->packet_kind = H265NAL_RTP_PACKET_KIND_SINGLE;
    FillRtpPayloadFields(rtp->rtp_single->nal_unit_payload.get(), out_rtp);
    return H265NAL_STATUS_OK;
  }

  if (rtp->rtp_ap != nullptr) {
    out_rtp->packet_kind = H265NAL_RTP_PACKET_KIND_AP;
    out_rtp->ap_nal_unit_sizes_count = rtp->rtp_ap->nal_unit_sizes.size();
    out_rtp->ap_nal_unit_headers_count = rtp->rtp_ap->nal_unit_headers.size();
    out_rtp->ap_nal_unit_payloads_count = rtp->rtp_ap->nal_unit_payloads.size();

    out_rtp->ap_nal_unit_type_count = rtp->rtp_ap->nal_unit_headers.size();
    if (out_rtp->ap_nal_unit_type_count > kRtpApTypeCapacity) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }
    for (size_t i = 0; i < out_rtp->ap_nal_unit_type_count; ++i) {
      const auto& nalu_header = rtp->rtp_ap->nal_unit_headers[i];
      if (nalu_header == nullptr) {
        return H265NAL_STATUS_PARSE_FAILURE;
      }
      out_rtp->ap_nal_unit_types[i] = nalu_header->nal_unit_type;
    }

    for (const auto& payload : rtp->rtp_ap->nal_unit_payloads) {
      FillRtpPayloadFields(payload.get(), out_rtp);
    }
    return H265NAL_STATUS_OK;
  }

  if (rtp->rtp_fu != nullptr) {
    out_rtp->packet_kind = H265NAL_RTP_PACKET_KIND_FU;
    out_rtp->fu_s_bit = rtp->rtp_fu->s_bit;
    out_rtp->fu_e_bit = rtp->rtp_fu->e_bit;
    out_rtp->fu_type = rtp->rtp_fu->fu_type;
    out_rtp->fu_has_nal_unit_payload = rtp->rtp_fu->nal_unit_payload == nullptr ? 0u : 1u;
    FillRtpPayloadFields(rtp->rtp_fu->nal_unit_payload.get(), out_rtp);
    return H265NAL_STATUS_OK;
  }

  out_rtp->packet_kind = H265NAL_RTP_PACKET_KIND_NONE;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265SeiMessageParser::ParseSei` flattened payload fields.
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

  *out_sei_message = {};
  out_sei_message->payload_type = static_cast<int32_t>(sei->payload_type);
  out_sei_message->payload_size = sei->payload_size;

  auto* user_data_registered_itu_t_t35 = dynamic_cast<
      h265nal::H265SeiUserDataRegisteredItuTT35Parser::
          H265SeiUserDataRegisteredItuTT35State*>(sei->payload_state.get());
  if (user_data_registered_itu_t_t35 != nullptr) {
    out_sei_message->has_user_data_registered_itu_t_t35 = 1;
    out_sei_message->user_data_registered_itu_t_t35_country_code =
        user_data_registered_itu_t_t35->itu_t_t35_country_code;
    out_sei_message->user_data_registered_itu_t_t35_country_code_extension_byte =
        user_data_registered_itu_t_t35->itu_t_t35_country_code_extension_byte;
    out_sei_message->user_data_registered_itu_t_t35_payload_size =
        static_cast<uint32_t>(user_data_registered_itu_t_t35->payload.size());
  }

  auto* user_data_unregistered = dynamic_cast<
      h265nal::H265SeiUserDataUnregisteredParser::H265SeiUserDataUnregisteredState*>(
      sei->payload_state.get());
  if (user_data_unregistered != nullptr) {
    out_sei_message->has_user_data_unregistered = 1;
    out_sei_message->user_data_unregistered_uuid_iso_iec_11578_1 =
        user_data_unregistered->uuid_iso_iec_11578_1;
    out_sei_message->user_data_unregistered_uuid_iso_iec_11578_2 =
        user_data_unregistered->uuid_iso_iec_11578_2;
    out_sei_message->user_data_unregistered_payload_size =
        static_cast<uint32_t>(user_data_unregistered->payload.size());
  }

  auto* alpha_channel_info =
      dynamic_cast<h265nal::H265SeiAlphaChannelInfoParser::
                       H265SeiAlphaChannelInfoState*>(
          sei->payload_state.get());
  if (alpha_channel_info != nullptr) {
    out_sei_message->has_alpha_channel_info = 1;
    out_sei_message->alpha_channel_cancel_flag =
        alpha_channel_info->alpha_channel_cancel_flag;
    out_sei_message->alpha_channel_use_idc = alpha_channel_info->alpha_channel_use_idc;
    out_sei_message->alpha_channel_bit_depth_minus8 =
        alpha_channel_info->alpha_channel_bit_depth_minus8;
    out_sei_message->alpha_transparent_value =
        alpha_channel_info->alpha_transparent_value;
    out_sei_message->alpha_opaque_value = alpha_channel_info->alpha_opaque_value;
    out_sei_message->alpha_channel_incr_flag =
        alpha_channel_info->alpha_channel_incr_flag;
    out_sei_message->alpha_channel_clip_flag =
        alpha_channel_info->alpha_channel_clip_flag;
    out_sei_message->alpha_channel_clip_type_flag =
        alpha_channel_info->alpha_channel_clip_type_flag;
  }

  auto* mastering_display_colour_volume =
      dynamic_cast<h265nal::H265SeiMasteringDisplayColourVolumeParser::
                       H265SeiMasteringDisplayColourVolumeState*>(
          sei->payload_state.get());
  if (mastering_display_colour_volume != nullptr) {
    out_sei_message->has_mastering_display_colour_volume = 1;
    out_sei_message->mastering_display_display_primaries_x[0] =
        mastering_display_colour_volume->display_primaries_x[0];
    out_sei_message->mastering_display_display_primaries_x[1] =
        mastering_display_colour_volume->display_primaries_x[1];
    out_sei_message->mastering_display_display_primaries_x[2] =
        mastering_display_colour_volume->display_primaries_x[2];
    out_sei_message->mastering_display_display_primaries_y[0] =
        mastering_display_colour_volume->display_primaries_y[0];
    out_sei_message->mastering_display_display_primaries_y[1] =
        mastering_display_colour_volume->display_primaries_y[1];
    out_sei_message->mastering_display_display_primaries_y[2] =
        mastering_display_colour_volume->display_primaries_y[2];
    out_sei_message->mastering_display_white_point_x =
        mastering_display_colour_volume->white_point_x;
    out_sei_message->mastering_display_white_point_y =
        mastering_display_colour_volume->white_point_y;
    out_sei_message->mastering_display_max_display_mastering_luminance =
        mastering_display_colour_volume->max_display_mastering_luminance;
    out_sei_message->mastering_display_min_display_mastering_luminance =
        mastering_display_colour_volume->min_display_mastering_luminance;
  }

  auto* content_light_level_info = dynamic_cast<
      h265nal::H265SeiContentLightLevelInfoParser::
          H265SeiContentLightLevelInfoState*>(sei->payload_state.get());
  if (content_light_level_info != nullptr) {
    out_sei_message->has_content_light_level_info = 1;
    out_sei_message->content_light_level_max_content_light_level =
        content_light_level_info->max_content_light_level;
    out_sei_message->content_light_level_max_pic_average_light_level =
        content_light_level_info->max_pic_average_light_level;
  }

  auto* unknown_payload =
      dynamic_cast<h265nal::H265SeiUnknownParser::H265SeiUnknownState*>(
          sei->payload_state.get());
  if (unknown_payload != nullptr) {
    out_sei_message->has_unknown_payload = 1;
    out_sei_message->unknown_payload_size =
        static_cast<uint32_t>(unknown_payload->payload.size());
  }
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose dynamic SEI registered ITU-T T.35 payload bytes.
int h265nal_sei_registered_itu_t_t35_payload_get(const uint8_t* data,
                                                 size_t len,
                                                 uint8_t* out_values,
                                                 size_t out_capacity,
                                                 size_t* out_count) {
  if (out_count == nullptr || (out_values == nullptr && out_capacity > 0) ||
      (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto sei = h265nal::H265SeiMessageParser::ParseSei(data, len);
  if (sei == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  auto* payload = dynamic_cast<
      h265nal::H265SeiUserDataRegisteredItuTT35Parser::
          H265SeiUserDataRegisteredItuTT35State*>(sei->payload_state.get());
  if (payload == nullptr) {
    *out_count = 0;
    return H265NAL_STATUS_OK;
  }

  return CopyU8VectorToCallerBuffer(payload->payload, out_values, out_capacity,
                                    out_count);
}

// DIVERGENCE: expose dynamic SEI unregistered payload bytes.
int h265nal_sei_unregistered_payload_get(const uint8_t* data,
                                         size_t len,
                                         uint8_t* out_values,
                                         size_t out_capacity,
                                         size_t* out_count) {
  if (out_count == nullptr || (out_values == nullptr && out_capacity > 0) ||
      (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto sei = h265nal::H265SeiMessageParser::ParseSei(data, len);
  if (sei == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  auto* payload = dynamic_cast<
      h265nal::H265SeiUserDataUnregisteredParser::
          H265SeiUserDataUnregisteredState*>(sei->payload_state.get());
  if (payload == nullptr) {
    *out_count = 0;
    return H265NAL_STATUS_OK;
  }

  return CopyU8VectorToCallerBuffer(payload->payload, out_values, out_capacity,
                                    out_count);
}

// DIVERGENCE: expose dynamic SEI unknown payload bytes.
int h265nal_sei_unknown_payload_get(const uint8_t* data,
                                    size_t len,
                                    uint8_t* out_values,
                                    size_t out_capacity,
                                    size_t* out_count) {
  if (out_count == nullptr || (out_values == nullptr && out_capacity > 0) ||
      (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto sei = h265nal::H265SeiMessageParser::ParseSei(data, len);
  if (sei == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  auto* payload = dynamic_cast<h265nal::H265SeiUnknownParser::H265SeiUnknownState*>(
      sei->payload_state.get());
  if (payload == nullptr) {
    *out_count = 0;
    return H265NAL_STATUS_OK;
  }

  return CopyU8VectorToCallerBuffer(payload->payload, out_values, out_capacity,
                                    out_count);
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

// DIVERGENCE: expose `H265ScalingListDataParser::ParseScalingListData`.
int h265nal_scaling_list_data_parse(
    const uint8_t* data,
    size_t len,
    h265nal_scaling_list_data_fields* out_scaling_list_data) {
  if (out_scaling_list_data == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto scaling_list_data =
      h265nal::H265ScalingListDataParser::ParseScalingListData(data, len);
  if (scaling_list_data == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  if (scaling_list_data->scaling_list_pred_mode_flag.size() >
          kScalingListSizeIdCount ||
      scaling_list_data->scaling_list_pred_matrix_id_delta.size() >
          kScalingListSizeIdCount ||
      scaling_list_data->scaling_list_dc_coef_minus8.size() >
          kScalingListSizeIdCount ||
      scaling_list_data->ScalingList.size() > kScalingListSizeIdCount) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_scaling_list_data->size_id_count =
      static_cast<uint32_t>(kScalingListSizeIdCount);
  out_scaling_list_data->matrix_id_count =
      static_cast<uint32_t>(kScalingListMatrixIdCount);

  for (size_t i = 0; i < kScalingListFlattenedCapacity; ++i) {
    out_scaling_list_data->scaling_list_pred_mode_flag[i] = 0;
    out_scaling_list_data->scaling_list_pred_matrix_id_delta[i] = 0;
    out_scaling_list_data->scaling_list_dc_coef_minus8[i] = 0;
    out_scaling_list_data->scaling_list_size[i] = 0;
    out_scaling_list_data->scaling_list_first_coef[i] = 0;
    out_scaling_list_data->scaling_list_all_8[i] = 0;
  }

  for (size_t size_id = 0;
       size_id < scaling_list_data->scaling_list_pred_mode_flag.size();
       ++size_id) {
    if (scaling_list_data->scaling_list_pred_mode_flag[size_id].size() >
        kScalingListMatrixIdCount) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }
    for (size_t matrix_id = 0;
         matrix_id < scaling_list_data->scaling_list_pred_mode_flag[size_id].size();
         ++matrix_id) {
      out_scaling_list_data->scaling_list_pred_mode_flag[
          ScalingListIndex(size_id, matrix_id)] =
          scaling_list_data->scaling_list_pred_mode_flag[size_id][matrix_id];
    }
  }

  for (size_t size_id = 0;
       size_id < scaling_list_data->scaling_list_pred_matrix_id_delta.size();
       ++size_id) {
    if (scaling_list_data->scaling_list_pred_matrix_id_delta[size_id].size() >
        kScalingListMatrixIdCount) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }
    for (size_t matrix_id = 0;
         matrix_id <
         scaling_list_data->scaling_list_pred_matrix_id_delta[size_id].size();
         ++matrix_id) {
      out_scaling_list_data->scaling_list_pred_matrix_id_delta[
          ScalingListIndex(size_id, matrix_id)] =
          scaling_list_data->scaling_list_pred_matrix_id_delta[size_id][matrix_id];
    }
  }

  for (size_t size_id = 0; size_id < scaling_list_data->scaling_list_dc_coef_minus8.size();
       ++size_id) {
    if (scaling_list_data->scaling_list_dc_coef_minus8[size_id].size() >
        kScalingListMatrixIdCount) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }
    for (size_t matrix_id = 0;
         matrix_id < scaling_list_data->scaling_list_dc_coef_minus8[size_id].size();
         ++matrix_id) {
      out_scaling_list_data->scaling_list_dc_coef_minus8[
          ScalingListIndex(size_id, matrix_id)] =
          scaling_list_data->scaling_list_dc_coef_minus8[size_id][matrix_id];
    }
  }

  for (size_t size_id = 0; size_id < scaling_list_data->ScalingList.size(); ++size_id) {
    if (scaling_list_data->ScalingList[size_id].size() > kScalingListMatrixIdCount) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }
    for (size_t matrix_id = 0;
         matrix_id < scaling_list_data->ScalingList[size_id].size(); ++matrix_id) {
      const auto& coefficients = scaling_list_data->ScalingList[size_id][matrix_id];
      const size_t index = ScalingListIndex(size_id, matrix_id);
      out_scaling_list_data->scaling_list_size[index] = coefficients.size();
      if (!coefficients.empty()) {
        out_scaling_list_data->scaling_list_first_coef[index] = coefficients[0];
        bool all_8 = true;
        for (size_t i = 0; i < coefficients.size(); ++i) {
          if (coefficients[i] != 8) {
            all_8 = false;
            break;
          }
        }
        out_scaling_list_data->scaling_list_all_8[index] = all_8 ? 1u : 0u;
      }
    }
  }

  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265Sps3dExtensionParser::ParseSps3dExtension`.
int h265nal_sps_3d_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_sps_3d_extension_fields* out_sps_3d_extension) {
  if (out_sps_3d_extension == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto extension = h265nal::H265Sps3dExtensionParser::ParseSps3dExtension(data, len);
  if (extension == nullptr || extension->iv_di_mc_enabled_flag.size() > kSps3dFlagCapacity ||
      extension->iv_mv_scal_enabled_flag.size() > kSps3dFlagCapacity) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_sps_3d_extension->iv_di_mc_enabled_flag_size =
      extension->iv_di_mc_enabled_flag.size();
  out_sps_3d_extension->iv_mv_scal_enabled_flag_size =
      extension->iv_mv_scal_enabled_flag.size();
  for (size_t i = 0; i < kSps3dFlagCapacity; ++i) {
    out_sps_3d_extension->iv_di_mc_enabled_flag[i] = 0;
    out_sps_3d_extension->iv_mv_scal_enabled_flag[i] = 0;
  }
  for (size_t i = 0; i < out_sps_3d_extension->iv_di_mc_enabled_flag_size; ++i) {
    out_sps_3d_extension->iv_di_mc_enabled_flag[i] =
        extension->iv_di_mc_enabled_flag[i];
  }
  for (size_t i = 0; i < out_sps_3d_extension->iv_mv_scal_enabled_flag_size; ++i) {
    out_sps_3d_extension->iv_mv_scal_enabled_flag[i] =
        extension->iv_mv_scal_enabled_flag[i];
  }

  out_sps_3d_extension->log2_ivmc_sub_pb_size_minus3 =
      extension->log2_ivmc_sub_pb_size_minus3;
  out_sps_3d_extension->iv_res_pred_enabled_flag = extension->iv_res_pred_enabled_flag;
  out_sps_3d_extension->depth_ref_enabled_flag = extension->depth_ref_enabled_flag;
  out_sps_3d_extension->vsp_mc_enabled_flag = extension->vsp_mc_enabled_flag;
  out_sps_3d_extension->dbbp_enabled_flag = extension->dbbp_enabled_flag;
  out_sps_3d_extension->tex_mc_enabled_flag = extension->tex_mc_enabled_flag;
  out_sps_3d_extension->log2_texmc_sub_pb_size_minus3 =
      extension->log2_texmc_sub_pb_size_minus3;
  out_sps_3d_extension->intra_contour_enabled_flag =
      extension->intra_contour_enabled_flag;
  out_sps_3d_extension->intra_dc_only_wedge_enabled_flag =
      extension->intra_dc_only_wedge_enabled_flag;
  out_sps_3d_extension->cqt_cu_part_pred_enabled_flag =
      extension->cqt_cu_part_pred_enabled_flag;
  out_sps_3d_extension->inter_dc_only_enabled_flag =
      extension->inter_dc_only_enabled_flag;
  out_sps_3d_extension->skip_intra_enabled_flag = extension->skip_intra_enabled_flag;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265SpsRangeExtensionParser::ParseSpsRangeExtension`.
int h265nal_sps_range_extension_parse(
    const uint8_t* data,
    size_t len,
    h265nal_sps_range_extension_fields* out_sps_range_extension) {
  if (out_sps_range_extension == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto extension =
      h265nal::H265SpsRangeExtensionParser::ParseSpsRangeExtension(data, len);
  if (extension == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_sps_range_extension->transform_skip_rotation_enabled_flag =
      extension->transform_skip_rotation_enabled_flag;
  out_sps_range_extension->transform_skip_context_enabled_flag =
      extension->transform_skip_context_enabled_flag;
  out_sps_range_extension->implicit_rdpcm_enabled_flag =
      extension->implicit_rdpcm_enabled_flag;
  out_sps_range_extension->explicit_rdpcm_enabled_flag =
      extension->explicit_rdpcm_enabled_flag;
  out_sps_range_extension->extended_precision_processing_flag =
      extension->extended_precision_processing_flag;
  out_sps_range_extension->intra_smoothing_disabled_flag =
      extension->intra_smoothing_disabled_flag;
  out_sps_range_extension->high_precision_offsets_enabled_flag =
      extension->high_precision_offsets_enabled_flag;
  out_sps_range_extension->persistent_rice_adaptation_enabled_flag =
      extension->persistent_rice_adaptation_enabled_flag;
  out_sps_range_extension->cabac_bypass_alignment_enabled_flag =
      extension->cabac_bypass_alignment_enabled_flag;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265SpsSccExtensionParser::ParseSpsSccExtension`.
int h265nal_sps_scc_extension_parse(
    const uint8_t* data,
    size_t len,
    uint32_t chroma_format_idc,
    uint32_t bit_depth_luma_minus8,
    uint32_t bit_depth_chroma_minus8,
    h265nal_sps_scc_extension_fields* out_sps_scc_extension) {
  if (out_sps_scc_extension == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto extension = h265nal::H265SpsSccExtensionParser::ParseSpsSccExtension(
      data, len, chroma_format_idc, bit_depth_luma_minus8,
      bit_depth_chroma_minus8);
  if (extension == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_sps_scc_extension->sps_curr_pic_ref_enabled_flag =
      extension->sps_curr_pic_ref_enabled_flag;
  out_sps_scc_extension->palette_mode_enabled_flag =
      extension->palette_mode_enabled_flag;
  out_sps_scc_extension->palette_max_size = extension->palette_max_size;
  out_sps_scc_extension->delta_palette_max_predictor_size =
      extension->delta_palette_max_predictor_size;
  out_sps_scc_extension->sps_palette_predictor_initializers_present_flag =
      extension->sps_palette_predictor_initializers_present_flag;
  out_sps_scc_extension->sps_num_palette_predictor_initializers_minus1 =
      extension->sps_num_palette_predictor_initializers_minus1;
  out_sps_scc_extension->sps_palette_predictor_initializers_size =
      extension->sps_palette_predictor_initializers.size();
  out_sps_scc_extension->motion_vector_resolution_control_idc =
      extension->motion_vector_resolution_control_idc;
  out_sps_scc_extension->intra_boundary_filtering_disabled_flag =
      extension->intra_boundary_filtering_disabled_flag;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265StRefPicSetParser::ParseStRefPicSet`.
int h265nal_st_ref_pic_set_parse(
    const uint8_t* data,
    size_t len,
    uint32_t st_rps_idx,
    uint32_t num_short_term_ref_pic_sets,
    uint32_t max_num_pics,
    h265nal_st_ref_pic_set_fields* out_st_ref_pic_set) {
  if (out_st_ref_pic_set == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  std::vector<std::unique_ptr<h265nal::H265StRefPicSetParser::StRefPicSetState>>
      st_ref_pic_set_state_vector;
  auto st_ref_pic_set = h265nal::H265StRefPicSetParser::ParseStRefPicSet(
      data, len, st_rps_idx, num_short_term_ref_pic_sets,
      &st_ref_pic_set_state_vector, max_num_pics);
  if (st_ref_pic_set == nullptr ||
      st_ref_pic_set->delta_poc_s0_minus1.size() > kStRefPicSetCapacity) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_st_ref_pic_set->st_rps_idx = st_rps_idx;
  out_st_ref_pic_set->num_short_term_ref_pic_sets = num_short_term_ref_pic_sets;
  out_st_ref_pic_set->max_num_pics = max_num_pics;
  out_st_ref_pic_set->num_negative_pics = st_ref_pic_set->num_negative_pics;
  out_st_ref_pic_set->num_positive_pics = st_ref_pic_set->num_positive_pics;
  out_st_ref_pic_set->delta_poc_s0_minus1_size =
      st_ref_pic_set->delta_poc_s0_minus1.size();
  for (size_t i = 0; i < kStRefPicSetCapacity; ++i) {
    out_st_ref_pic_set->delta_poc_s0_minus1[i] = 0;
  }
  for (size_t i = 0; i < out_st_ref_pic_set->delta_poc_s0_minus1_size; ++i) {
    out_st_ref_pic_set->delta_poc_s0_minus1[i] = st_ref_pic_set->delta_poc_s0_minus1[i];
  }

  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265SubLayerHrdParametersParser::ParseSubLayerHrdParameters`.
int h265nal_sub_layer_hrd_parameters_parse(
    const uint8_t* data,
    size_t len,
    uint32_t sub_layer_id,
    uint32_t cpb_cnt,
    uint32_t sub_pic_hrd_params_present_flag,
    h265nal_sub_layer_hrd_parameters_fields* out_sub_layer_hrd_parameters) {
  if (out_sub_layer_hrd_parameters == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto sub_layer_hrd =
      h265nal::H265SubLayerHrdParametersParser::ParseSubLayerHrdParameters(
          data, len, sub_layer_id, cpb_cnt, sub_pic_hrd_params_present_flag);
  if (sub_layer_hrd == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  if (!FillSubLayerHrdParametersFields(*sub_layer_hrd,
                                       out_sub_layer_hrd_parameters)) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265HrdParametersParser::ParseHrdParameters` fields.
int h265nal_hrd_parameters_parse(
    const uint8_t* data,
    size_t len,
    uint32_t common_inf_present_flag,
    uint32_t max_num_sub_layers_minus1,
    h265nal_hrd_parameters_fields* out_hrd_parameters) {
  if (out_hrd_parameters == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto hrd_parameters = h265nal::H265HrdParametersParser::ParseHrdParameters(
      data, len, common_inf_present_flag, max_num_sub_layers_minus1);
  if (hrd_parameters == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  if (!FillHrdParametersFields(*hrd_parameters, common_inf_present_flag,
                               max_num_sub_layers_minus1,
                               out_hrd_parameters)) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265VpsParser::ParseVps` flattened fields.
int h265nal_vps_parse(const uint8_t* data,
                      size_t len,
                      h265nal_vps_fields* out_vps) {
  if (out_vps == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto vps = h265nal::H265VpsParser::ParseVps(data, len);
  if (vps == nullptr || vps->profile_tier_level == nullptr ||
      vps->profile_tier_level->general == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_vps->vps_video_parameter_set_id = vps->vps_video_parameter_set_id;
  out_vps->vps_base_layer_internal_flag = vps->vps_base_layer_internal_flag;
  out_vps->vps_base_layer_available_flag = vps->vps_base_layer_available_flag;
  out_vps->vps_max_layers_minus1 = vps->vps_max_layers_minus1;
  out_vps->vps_max_sub_layers_minus1 = vps->vps_max_sub_layers_minus1;
  out_vps->vps_temporal_id_nesting_flag = vps->vps_temporal_id_nesting_flag;
  out_vps->vps_reserved_0xffff_16bits = vps->vps_reserved_0xffff_16bits;
  FillProfileTierLevelFields(*vps->profile_tier_level, &out_vps->profile_tier_level);

  out_vps->vps_sub_layer_ordering_info_present_flag =
      vps->vps_sub_layer_ordering_info_present_flag;
  out_vps->vps_max_dec_pic_buffering_minus1_size =
      vps->vps_max_dec_pic_buffering_minus1.size();
  out_vps->vps_max_num_reorder_pics_size = vps->vps_max_num_reorder_pics.size();
  out_vps->vps_max_latency_increase_plus1_size =
      vps->vps_max_latency_increase_plus1.size();
  if (out_vps->vps_max_dec_pic_buffering_minus1_size > kLayerVectorCapacity ||
      out_vps->vps_max_num_reorder_pics_size > kLayerVectorCapacity ||
      out_vps->vps_max_latency_increase_plus1_size > kLayerVectorCapacity) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }
  for (size_t i = 0; i < out_vps->vps_max_dec_pic_buffering_minus1_size; ++i) {
    out_vps->vps_max_dec_pic_buffering_minus1[i] =
        vps->vps_max_dec_pic_buffering_minus1[i];
  }
  for (size_t i = 0; i < out_vps->vps_max_num_reorder_pics_size; ++i) {
    out_vps->vps_max_num_reorder_pics[i] = vps->vps_max_num_reorder_pics[i];
  }
  for (size_t i = 0; i < out_vps->vps_max_latency_increase_plus1_size; ++i) {
    out_vps->vps_max_latency_increase_plus1[i] =
        vps->vps_max_latency_increase_plus1[i];
  }

  out_vps->vps_max_layer_id = vps->vps_max_layer_id;
  out_vps->vps_num_layer_sets_minus1 = vps->vps_num_layer_sets_minus1;
  out_vps->layer_id_included_flag_size = vps->layer_id_included_flag.size();
  out_vps->layer_id_included_flag_row_width = 0;
  out_vps->layer_id_included_flag_row0_mask = 0;
  out_vps->layer_id_included_flag_row1_mask = 0;
  if (out_vps->layer_id_included_flag_size > 2) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }
  if (out_vps->layer_id_included_flag_size > 0) {
    if (!FillSubLayerMask(vps->layer_id_included_flag[0],
                          &out_vps->layer_id_included_flag_row0_mask,
                          &out_vps->layer_id_included_flag_row_width)) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }
  }
  if (out_vps->layer_id_included_flag_size > 1) {
    uint32_t row1_width = 0;
    if (!FillSubLayerMask(vps->layer_id_included_flag[1],
                          &out_vps->layer_id_included_flag_row1_mask,
                          &row1_width)) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }
    if (row1_width != out_vps->layer_id_included_flag_row_width) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }
  }

  out_vps->vps_timing_info_present_flag = vps->vps_timing_info_present_flag;
  out_vps->vps_num_units_in_tick = vps->vps_num_units_in_tick;
  out_vps->vps_time_scale = vps->vps_time_scale;
  out_vps->vps_poc_proportional_to_timing_flag =
      vps->vps_poc_proportional_to_timing_flag;
  out_vps->vps_num_ticks_poc_diff_one_minus1 =
      vps->vps_num_ticks_poc_diff_one_minus1;
  out_vps->vps_num_hrd_parameters = vps->vps_num_hrd_parameters;
  out_vps->hrd_layer_set_idx_size = vps->hrd_layer_set_idx.size();
  out_vps->cprms_present_flag_size = vps->cprms_present_flag.size();
  out_vps->vps_extension_flag = vps->vps_extension_flag;
  out_vps->vps_extension_data_flag = vps->vps_extension_data_flag;
  return H265NAL_STATUS_OK;
}

// DIVERGENCE: expose `H265VuiParametersParser::ParseVuiParameters` flattened fields.
int h265nal_vui_parameters_parse(
    const uint8_t* data,
    size_t len,
    uint32_t sps_max_sub_layers_minus1,
    h265nal_vui_parameters_fields* out_vui_parameters) {
  if (out_vui_parameters == nullptr || (data == nullptr && len > 0)) {
    return H265NAL_STATUS_INVALID_ARGUMENT;
  }

  auto vui = h265nal::H265VuiParametersParser::ParseVuiParameters(
      data, len, sps_max_sub_layers_minus1);
  if (vui == nullptr) {
    return H265NAL_STATUS_PARSE_FAILURE;
  }

  out_vui_parameters->sps_max_sub_layers_minus1 = vui->sps_max_sub_layers_minus1;
  out_vui_parameters->aspect_ratio_info_present_flag =
      vui->aspect_ratio_info_present_flag;
  out_vui_parameters->aspect_ratio_idc = vui->aspect_ratio_idc;
  out_vui_parameters->sar_width = vui->sar_width;
  out_vui_parameters->sar_height = vui->sar_height;
  out_vui_parameters->overscan_info_present_flag = vui->overscan_info_present_flag;
  out_vui_parameters->overscan_appropriate_flag = vui->overscan_appropriate_flag;
  out_vui_parameters->video_signal_type_present_flag =
      vui->video_signal_type_present_flag;
  out_vui_parameters->video_format = vui->video_format;
  out_vui_parameters->video_full_range_flag = vui->video_full_range_flag;
  out_vui_parameters->colour_description_present_flag =
      vui->colour_description_present_flag;
  out_vui_parameters->colour_primaries = vui->colour_primaries;
  out_vui_parameters->transfer_characteristics = vui->transfer_characteristics;
  out_vui_parameters->matrix_coeffs = vui->matrix_coeffs;
  out_vui_parameters->chroma_loc_info_present_flag =
      vui->chroma_loc_info_present_flag;
  out_vui_parameters->chroma_sample_loc_type_top_field =
      vui->chroma_sample_loc_type_top_field;
  out_vui_parameters->chroma_sample_loc_type_bottom_field =
      vui->chroma_sample_loc_type_bottom_field;
  out_vui_parameters->neutral_chroma_indication_flag =
      vui->neutral_chroma_indication_flag;
  out_vui_parameters->field_seq_flag = vui->field_seq_flag;
  out_vui_parameters->frame_field_info_present_flag =
      vui->frame_field_info_present_flag;
  out_vui_parameters->default_display_window_flag =
      vui->default_display_window_flag;
  out_vui_parameters->def_disp_win_left_offset = vui->def_disp_win_left_offset;
  out_vui_parameters->def_disp_win_right_offset = vui->def_disp_win_right_offset;
  out_vui_parameters->def_disp_win_top_offset = vui->def_disp_win_top_offset;
  out_vui_parameters->def_disp_win_bottom_offset = vui->def_disp_win_bottom_offset;
  out_vui_parameters->vui_timing_info_present_flag =
      vui->vui_timing_info_present_flag;
  out_vui_parameters->vui_num_units_in_tick = vui->vui_num_units_in_tick;
  out_vui_parameters->vui_time_scale = vui->vui_time_scale;
  out_vui_parameters->vui_poc_proportional_to_timing_flag =
      vui->vui_poc_proportional_to_timing_flag;
  out_vui_parameters->vui_num_ticks_poc_diff_one_minus1 =
      vui->vui_num_ticks_poc_diff_one_minus1;
  out_vui_parameters->vui_hrd_parameters_present_flag =
      vui->vui_hrd_parameters_present_flag;
  out_vui_parameters->bitstream_restriction_flag = vui->bitstream_restriction_flag;
  out_vui_parameters->tiles_fixed_structure_flag = vui->tiles_fixed_structure_flag;
  out_vui_parameters->motion_vectors_over_pic_boundaries_flag =
      vui->motion_vectors_over_pic_boundaries_flag;
  out_vui_parameters->restricted_ref_pic_lists_flag =
      vui->restricted_ref_pic_lists_flag;
  out_vui_parameters->min_spatial_segmentation_idc =
      vui->min_spatial_segmentation_idc;
  out_vui_parameters->max_bytes_per_pic_denom = vui->max_bytes_per_pic_denom;
  out_vui_parameters->max_bits_per_min_cu_denom = vui->max_bits_per_min_cu_denom;
  out_vui_parameters->log2_max_mv_length_horizontal =
      vui->log2_max_mv_length_horizontal;
  out_vui_parameters->log2_max_mv_length_vertical = vui->log2_max_mv_length_vertical;

  out_vui_parameters->has_hrd_parameters = vui->hrd_parameters == nullptr ? 0u : 1u;
  out_vui_parameters->hrd_nal_hrd_parameters_present_flag = 0;
  out_vui_parameters->hrd_vcl_hrd_parameters_present_flag = 0;
  out_vui_parameters->hrd_sub_pic_hrd_params_present_flag = 0;
  out_vui_parameters->hrd_tick_divisor_minus2 = 0;
  out_vui_parameters->hrd_du_cpb_removal_delay_increment_length_minus1 = 0;
  out_vui_parameters->hrd_sub_pic_cpb_params_in_pic_timing_sei_flag = 0;
  out_vui_parameters->hrd_dpb_output_delay_du_length_minus1 = 0;
  out_vui_parameters->hrd_bit_rate_scale = 0;
  out_vui_parameters->hrd_cpb_size_scale = 0;
  out_vui_parameters->hrd_cpb_size_du_scale = 0;
  out_vui_parameters->hrd_initial_cpb_removal_delay_length_minus1 = 0;
  out_vui_parameters->hrd_au_cpb_removal_delay_length_minus1 = 0;
  out_vui_parameters->hrd_dpb_output_delay_length_minus1 = 0;
  out_vui_parameters->hrd_fixed_pic_rate_general_flag_size = 0;
  out_vui_parameters->hrd_fixed_pic_rate_within_cvs_flag_size = 0;
  out_vui_parameters->hrd_elemental_duration_in_tc_minus1_size = 0;
  out_vui_parameters->hrd_low_delay_hrd_flag_size = 0;
  out_vui_parameters->hrd_cpb_cnt_minus1_size = 0;
  out_vui_parameters->hrd_sub_layer_hrd_parameters_vector_size = 0;
  out_vui_parameters->hrd_sub_layer_hrd_parameters_0_present = 0;

  if (vui->hrd_parameters != nullptr) {
    auto& hrd = vui->hrd_parameters;
    out_vui_parameters->hrd_nal_hrd_parameters_present_flag =
        hrd->nal_hrd_parameters_present_flag;
    out_vui_parameters->hrd_vcl_hrd_parameters_present_flag =
        hrd->vcl_hrd_parameters_present_flag;
    out_vui_parameters->hrd_sub_pic_hrd_params_present_flag =
        hrd->sub_pic_hrd_params_present_flag;
    out_vui_parameters->hrd_tick_divisor_minus2 = hrd->tick_divisor_minus2;
    out_vui_parameters->hrd_du_cpb_removal_delay_increment_length_minus1 =
        hrd->du_cpb_removal_delay_increment_length_minus1;
    out_vui_parameters->hrd_sub_pic_cpb_params_in_pic_timing_sei_flag =
        hrd->sub_pic_cpb_params_in_pic_timing_sei_flag;
    out_vui_parameters->hrd_dpb_output_delay_du_length_minus1 =
        hrd->dpb_output_delay_du_length_minus1;
    out_vui_parameters->hrd_bit_rate_scale = hrd->bit_rate_scale;
    out_vui_parameters->hrd_cpb_size_scale = hrd->cpb_size_scale;
    out_vui_parameters->hrd_cpb_size_du_scale = hrd->cpb_size_du_scale;
    out_vui_parameters->hrd_initial_cpb_removal_delay_length_minus1 =
        hrd->initial_cpb_removal_delay_length_minus1;
    out_vui_parameters->hrd_au_cpb_removal_delay_length_minus1 =
        hrd->au_cpb_removal_delay_length_minus1;
    out_vui_parameters->hrd_dpb_output_delay_length_minus1 =
        hrd->dpb_output_delay_length_minus1;

    out_vui_parameters->hrd_fixed_pic_rate_general_flag_size =
        hrd->fixed_pic_rate_general_flag.size();
    out_vui_parameters->hrd_fixed_pic_rate_within_cvs_flag_size =
        hrd->fixed_pic_rate_within_cvs_flag.size();
    out_vui_parameters->hrd_elemental_duration_in_tc_minus1_size =
        hrd->elemental_duration_in_tc_minus1.size();
    out_vui_parameters->hrd_low_delay_hrd_flag_size =
        hrd->low_delay_hrd_flag.size();
    out_vui_parameters->hrd_cpb_cnt_minus1_size = hrd->cpb_cnt_minus1.size();

    if (out_vui_parameters->hrd_fixed_pic_rate_general_flag_size >
            kLayerVectorCapacity ||
        out_vui_parameters->hrd_fixed_pic_rate_within_cvs_flag_size >
            kLayerVectorCapacity ||
        out_vui_parameters->hrd_elemental_duration_in_tc_minus1_size >
            kLayerVectorCapacity ||
        out_vui_parameters->hrd_low_delay_hrd_flag_size > kLayerVectorCapacity ||
        out_vui_parameters->hrd_cpb_cnt_minus1_size > kLayerVectorCapacity) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }

    for (size_t i = 0; i < out_vui_parameters->hrd_fixed_pic_rate_general_flag_size;
         ++i) {
      out_vui_parameters->hrd_fixed_pic_rate_general_flag[i] =
          hrd->fixed_pic_rate_general_flag[i];
    }
    for (size_t i = 0;
         i < out_vui_parameters->hrd_fixed_pic_rate_within_cvs_flag_size; ++i) {
      out_vui_parameters->hrd_fixed_pic_rate_within_cvs_flag[i] =
          hrd->fixed_pic_rate_within_cvs_flag[i];
    }
    for (size_t i = 0;
         i < out_vui_parameters->hrd_elemental_duration_in_tc_minus1_size; ++i) {
      out_vui_parameters->hrd_elemental_duration_in_tc_minus1[i] =
          hrd->elemental_duration_in_tc_minus1[i];
    }
    for (size_t i = 0; i < out_vui_parameters->hrd_low_delay_hrd_flag_size; ++i) {
      out_vui_parameters->hrd_low_delay_hrd_flag[i] = hrd->low_delay_hrd_flag[i];
    }
    for (size_t i = 0; i < out_vui_parameters->hrd_cpb_cnt_minus1_size; ++i) {
      out_vui_parameters->hrd_cpb_cnt_minus1[i] = hrd->cpb_cnt_minus1[i];
    }

    out_vui_parameters->hrd_sub_layer_hrd_parameters_vector_size =
        hrd->sub_layer_hrd_parameters_vector.size();
    if (out_vui_parameters->hrd_sub_layer_hrd_parameters_vector_size >
        kLayerVectorCapacity) {
      return H265NAL_STATUS_PARSE_FAILURE;
    }
    if (out_vui_parameters->hrd_sub_layer_hrd_parameters_vector_size > 0 &&
        hrd->sub_layer_hrd_parameters_vector[0] != nullptr) {
      out_vui_parameters->hrd_sub_layer_hrd_parameters_0_present = 1;
      if (!FillSubLayerHrdParametersFields(
              *hrd->sub_layer_hrd_parameters_vector[0],
              &out_vui_parameters->hrd_sub_layer_hrd_parameters_0)) {
        return H265NAL_STATUS_PARSE_FAILURE;
      }
    }
  }

  return H265NAL_STATUS_OK;
}

uint32_t h265nal_abi_version(void) {
  return H265NAL_C_ABI_VERSION;
}
