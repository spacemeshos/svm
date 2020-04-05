#ifndef READ_FILE_T 
#define READ_FILE_T 

#include "../svm.h"
#include <stdio.h>

svm_byte_array read_file(const char *file_name) {
  svm_byte_array arr;

  FILE *file = fopen(file_name, "r");
  fseek(file, 0, SEEK_END);
  arr.length = ftell(file);

  arr.bytes = (uint8_t*)malloc(arr.length);
  fseek(file, 0, SEEK_SET);
  fread((void*)arr.bytes, 1, arr.length, file);
  fclose(file);

  return arr;
}

#endif /* READ_FILE_T */
