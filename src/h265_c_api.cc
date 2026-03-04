#include "h265_c_api.h"

#include <cstdio>

#include "h265_bitstream_parser.h"

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

uint32_t h265nal_abi_version(void) {
  return H265NAL_C_ABI_VERSION;
}
