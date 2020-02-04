#ifndef SVM_EXE_APP_BYTES
#define SVM_EXE_APP_BYTES

#include "svm.h"
#include "func_buf.h"
#include "func_args.h"

svm_byte_array exec_app_bytes(
    svm_byte_array app_addr,
    svm_byte_array func_name,
    svm_func_buf_t func_buf,
    svm_func_args_t func_args
) {
  uint32_t length =
    4   +  // proto version
    20  +  // app address
    1   +  // function name length
    func_name.length +  
    func_buf_length(func_buf) + 
    func_args_length(func_args);

  uint8_t* bytes = (uint8_t*)(malloc(length));  
  uint32_t cursor = 0;

  // set `proto=0`
  bytes[0] = 0;
  bytes[1] = 0;
  bytes[2] = 0;
  bytes[3] = 0;
  cursor += 4;

  // set `app` address
  memcpy(&bytes[cursor], app_addr.bytes, app_addr.length);
  cursor += app_addr.length;
  
  // `name length` consumes 1 byte
  assert(func_name.length <= 0xFF);
  bytes[cursor] = (uint8_t)func_name.length; 
  cursor += 1;

  // set `name length`
  memcpy(&bytes[cursor], func_name.bytes, func_name.length);
  cursor += func_name.length;

  // function buf

  //// `func buf #slices` conumes 1 byte
  bytes[cursor] = func_buf.slice_count; 
  cursor += 1;

  for (uint8_t i = 0; i < func_buf.slice_count; i++) {
    svm_byte_array slice = func_buf.slices[i];

    assert(slice.length <= 0xFFFF);

    //// we assume `slice_len` is laid out in Little-Endian in memory.
    uint16_t slice_len = (uint16_t)slice.length; 

    //// slice length consumes 2 bytes (Big-Endian)
    bytes[cursor + 0] = (uint8_t)((slice_len >> 8) & 0xFF);
    bytes[cursor + 1] = (uint8_t)((slice_len >> 0) & 0xFF); 
    cursor += 2;

    //// copy slice to `buf`
    memcpy(&bytes[cursor], slice.bytes, slice.length);
    cursor += slice.length;
  }

  // function args

  //// `func #args` consumes 1 byte
  bytes[cursor] = func_args.arg_count; 
  cursor += 1;

  //// copy `func args` to `buf`
  for (uint8_t i = 0; i < func_args.arg_count; i++) {
    svm_func_arg_t arg = func_args.args[i];

    svm_value_type arg_type = arg.type;

    //// arg type consumes 1 byte
    bytes[cursor] = (uint8_t)arg_type;
    cursor += 1;

    if (arg_type == SVM_I32) {
      memcpy(&bytes[cursor], arg.bytes, 4);
      cursor += 4; //// arg value takes 4 bytes
    }
    else if (arg_type == SVM_I64) {
      memcpy(&bytes[cursor], arg.bytes, 8);
      cursor += 8; //// arg value takes 8 bytes
    }
    else {
      //// ilegal argument type
      exit(-1);
    }
  }

  assert(cursor == length);

  svm_byte_array tx = {
    .length = length,
    .bytes = bytes
  };

  return tx;
}

#endif
