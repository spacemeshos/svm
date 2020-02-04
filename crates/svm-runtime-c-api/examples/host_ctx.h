#ifndef HOST_CTX
#define HOST_CTX

#include <stdint.h>

#include "svm.h"

svm_byte_array host_ctx_empty_bytes() {
  uint32_t length =
    4  +  // proto version
    2;   // #fields

  uint8_t* bytes = (uint8_t*)(malloc(length));
  
  uint32_t cursor = 0;

  // set `proto=0`
  bytes[0] = 0;
  bytes[1] = 0;
  bytes[2] = 0;
  bytes[3] = 0;
  cursor += 4;

  // set `#fields=0`
  bytes[cursor + 0] = 0;
  bytes[cursor + 1] = 0;
  cursor += 2;

  svm_byte_array host_ctx = {
    .bytes = bytes,
    .length = length
  };

  return host_ctx;
}

#endif
