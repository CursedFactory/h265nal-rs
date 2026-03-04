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

int h265nal_annexb_count_nalus(const uint8_t* data,
                               size_t len,
                               size_t* out_count);

int h265nal_annexb_dump(const uint8_t* data, size_t len, uint32_t dump_flags);

uint32_t h265nal_abi_version(void);

#ifdef __cplusplus
}
#endif

#endif  // H265_C_API_H_
