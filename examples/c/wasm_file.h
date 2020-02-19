#ifndef WASM_FILE_T 
#define WASM_FILE_T 

#include <stdint.h>
#include <stdlib.h>

typedef struct {
  uint8_t* bytes;
  long length;
} wasm_file_t;

wasm_file_t read_wasm_file(const char *file_name) {
  wasm_file_t wasm_file;

  FILE *file = fopen(file_name, "r");
  fseek(file, 0, SEEK_END);
  wasm_file.length = ftell(file);

  wasm_file.bytes = malloc(wasm_file.length);
  fseek(file, 0, SEEK_SET);
  fread(wasm_file.bytes, 1, wasm_file.length, file);
  fclose(file);

  return wasm_file;
}

#endif
