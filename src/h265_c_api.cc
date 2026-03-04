#include "h265_c_api.h"

#include <cstdio>
#include <new>

#include "h265_aud_parser.h"
#include "h265_bitstream_parser.h"
#include "h265_common.h"
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

uint32_t h265nal_abi_version(void) {
  return H265NAL_C_ABI_VERSION;
}
