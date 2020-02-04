#ifndef SVM_SPAWN_APP_BYTES_H
#define SVM_SPAWN_APP_BYTES_H

#include <stdint.h>
#include <assert.h>

#include "constants.h"

svm_byte_array spawn_app_bytes(svm_byte_array template_addr) {
  assert(template_addr.length == SVM_ADDR_LEN); 

  uint64_t length =
    4  +  // proto version
    template_addr.length +  // length(`template_addr)
    1 +  // ctor #slices
    1;   // ctor func #args

  uint8_t* bytes = (uint8_t*)(malloc(length));

  uint32_t cursor = 0;

  // set `proto=0`
  memset(&bytes[cursor], 0, 4);
  cursor += 4;

  // copy `template` address
  memcpy(&bytes[cursor], template_addr.bytes, template_addr.length);
  cursor += template_addr.length;

  // `ctor buf #slices` take 1 byte 
  bytes[cursor] = 0; // no `ctor func buf`
  cursor += 1;

  // `ctor #args` take 1 byte 
  bytes[cursor] = 0; // no `ctor func args`
  cursor += 1;

  svm_byte_array app = {
    .bytes = bytes,
    .length = length
  };

  return app;
}

#endif
