#ifndef SVM_DEPLOY_TEMPLATE_BYTES_H
#define SVM_DEPLOY_TEMPLATE_BYTES_H

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

#include "../svm.h"
#include "wasm_file.h"

svm_byte_array deploy_template_bytes() {
  wasm_file_t file = read_wasm_file("wasm/counter.wasm");

  uint64_t length =
    4  +  // proto version
    1  +  // name length 
    strlen("Example")  +  // length("Example")
    2  +  // `#admins`    (we'll set it to `0`)
    2  +  // `#deps`      (we'll set it to `0`)
    2  +  // `page_count` (we'll set it to `0`)
    8  +  //  code length (Big-Endian)
    (uint64_t)file.length; // code

  uint8_t* bytes = (uint8_t*)(malloc(length));

  uint32_t cursor = 0;

  // set `proto=0`
  memset(&bytes[cursor], 0, 4);
  cursor += 4;

  // name length takes 1 bytes
  bytes[cursor] = strlen("Example");
  cursor += 1;

  const char* name = "Example";
  memcpy(&bytes[cursor], name, strlen("Example"));
  cursor += strlen("Example");

  // `#admins` takes 2 bytes
  memset(&bytes[cursor], 0, 2);
  cursor += 2;

  // `#deps` takes 2 bytes
  memset(&bytes[cursor], 0, 2);
  cursor += 2;

  // `#page_count` takes 2 bytes
  memset(&bytes[cursor], 0, 2);
  cursor += 2;

  // set code-length (Big-Endian)
  uint8_t* code_length = (uint8_t*)&file.length;

  for (int i = 0; i < 8; i++) {
    // we assume `code_length` is in Little-Endian order,
    // so we reverse it since a raw template format expects it in Big-Endian order.
    bytes[cursor + i] = code_length[7 - i];
  }
  cursor += 8;

  // copy template code
  memcpy(&bytes[cursor], file.bytes, file.length);

  svm_byte_array template = {
    .bytes = bytes,
    .length = length
  };

  return template;
}

#endif
