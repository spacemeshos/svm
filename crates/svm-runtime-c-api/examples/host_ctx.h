#ifndef HOST_CTX_H
#define HOST_CTX_H

#include <stdint.h>

#include "svm.h"

svm_byte_array host_ctx_empty_bytes() {
  uint32_t length =
    4  +  // proto version
    2;   // #fields

  uint8_t* bytes = (uint8_t*)(malloc(length));
  
  uint32_t cursor = 0;

  // set `proto=0`
  memset(&bytes[cursor], 0, 4);
  cursor += 4;

  // set `#fields=0`
  memset(&bytes[cursor], 0, 2);
  cursor += 2;

  svm_byte_array host_ctx = {
    .bytes = bytes,
    .length = length
  };

  return host_ctx;
}

#endif
