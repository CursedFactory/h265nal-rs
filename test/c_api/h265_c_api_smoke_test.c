#include <assert.h>
#include <stddef.h>
#include <stdint.h>

#include "h265_c_api.h"

int main(void) {
  const uint8_t sample[] = {0x00, 0x00, 0x01, 0x40, 0x01};
  size_t count = 0;
  int status = 0;

  assert(h265nal_abi_version() == H265NAL_C_ABI_VERSION);

  status = h265nal_annexb_count_nalus(NULL, 0, &count);
  assert(status == H265NAL_STATUS_OK);
  assert(count == 0);

  status = h265nal_annexb_count_nalus(sample, sizeof(sample), &count);
  assert(status == H265NAL_STATUS_OK);
  assert(count == 1);

  status = h265nal_annexb_count_nalus(NULL, sizeof(sample), &count);
  assert(status == H265NAL_STATUS_INVALID_ARGUMENT);

  return 0;
}
