#include "h265_c_api.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

static uint8_t* read_file(const char* path, size_t* out_len) {
  FILE* fp = fopen(path, "rb");
  long size = 0;
  size_t read_len = 0;
  uint8_t* buffer = NULL;

  if (fp == NULL) {
    return NULL;
  }

  if (fseek(fp, 0, SEEK_END) != 0) {
    fclose(fp);
    return NULL;
  }

  size = ftell(fp);
  if (size < 0) {
    fclose(fp);
    return NULL;
  }

  if (fseek(fp, 0, SEEK_SET) != 0) {
    fclose(fp);
    return NULL;
  }

  buffer = (uint8_t*)malloc((size_t)size);
  if (buffer == NULL) {
    fclose(fp);
    return NULL;
  }

  read_len = fread(buffer, 1, (size_t)size, fp);
  fclose(fp);

  if (read_len != (size_t)size) {
    free(buffer);
    return NULL;
  }

  *out_len = (size_t)size;
  return buffer;
}

int main(int argc, char** argv) {
  uint8_t* data = NULL;
  size_t len = 0;
  int status = 0;

  if (argc != 2) {
    fprintf(stderr, "usage: %s <input.265>\n", argv[0]);
    return 2;
  }

  data = read_file(argv[1], &len);
  if (data == NULL) {
    fprintf(stderr, "failed to read input file: %s\n", argv[1]);
    return 3;
  }

  status = h265nal_annexb_dump(data, len, 0);
  fprintf(stderr, "h265nal_annexb_dump status=%d\n", status);

  free(data);
  return status;
}
