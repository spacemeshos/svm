#ifndef HELPERS_H
#define HELPERS_H

#include "../svm.h"
#include "constants.h"

svm_byte_array from_str(char* str) {
  return (svm_byte_array) {
    .bytes = str, 
    .length = strlen(str),
  };
}

svm_byte_array alloc_str_address(char* str) {
  uint8_t* buf = (uint8_t*)malloc(SVM_ADDR_LEN);
  memset(buf, 0, SVM_ADDR_LEN);

  int n = SVM_ADDR_LEN < strlen(str) ? SVM_ADDR_LEN : strlen(str);
  memcpy(buf, str, n);

  return (svm_byte_array) {
    .bytes = buf, 
    .length = SVM_ADDR_LEN,
  };
}

svm_byte_array empty_byte_array() {
  return (svm_byte_array) {
    .bytes = NULL,
    .length = 0
  };
}

#endif /* HELPERS_H */
