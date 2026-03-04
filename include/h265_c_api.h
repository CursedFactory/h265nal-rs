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

uint32_t h265nal_abi_version(void);

#ifdef __cplusplus
}
#endif

#endif  // H265_C_API_H_
