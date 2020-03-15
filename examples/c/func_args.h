#ifndef SVM_FUNC_ARGS_H
#define SVM_FUNC_ARGS_H

#include "../svm.h"

typedef struct {
  svm_value_type type;
  uint8_t* bytes; 
} svm_func_arg_t;

typedef struct {
  uint8_t arg_count;
  svm_func_arg_t* args;
} svm_func_args_t;

uint32_t func_args_length(svm_func_args_t func_args) {
  uint32_t acc = 0;

  acc += 1; // `#func args` consumes 1 byte

  for (uint8_t i = 0; i < func_args.arg_count; i++) {
    svm_func_arg_t arg = func_args.args[i];
    acc += 1; // `arg type` consumes 1 byte

    svm_value_type arg_type = arg.type;
    if (arg_type == SVM_I32) {
      acc += 4; // `arg` takes 4 bytes
    }
    else if (arg_type == SVM_I64) {
      acc += 8; // `arg` takes 8 bytes
    }
    else {
      // ilegal argument type
      exit(-1);
    }
  }

  return acc;
}

#endif
