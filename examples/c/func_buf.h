#ifndef SVM_FUNC_BUF_H
#define SVM_FUNC_BUF_H

#include <stdint.h>
#include "svm.h"

typedef struct {
  uint8_t slice_count;
  svm_byte_array* slices;
} svm_func_buf_t;


uint32_t func_buf_length(svm_func_buf_t func_buf) {
  uint32_t acc = 0;

  acc += 1; // `#func-buf #slices` consumes 1 byte

  for (uint8_t i = 0; i < func_buf.slice_count; i++) {
    svm_byte_array slice = func_buf.slices[i];
    acc += slice.length;
  }

  return acc;
}

#endif
