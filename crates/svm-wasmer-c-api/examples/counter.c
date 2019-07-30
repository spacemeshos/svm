#include "wasmer.h"
#include "svm_wasmer.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

typedef struct {
  uint32_t counter;
} node_data_t;

node_data_t* new_node_data(uint32_t counter) {
   node_data_t* ptr = (node_data_t*)malloc(sizeof(node_data_t));
   ptr->counter = counter;
   return ptr;
}

void inc_counter(wasmer_instance_context_t *ctx, uint32_t amount) {
  node_data_t *nd = (node_data_t*)(wasmer_svm_instance_context_node_data_get(ctx));
  nd->counter = nd->counter + amount;
}

uint32_t get_counter(wasmer_instance_context_t *ctx) {
  node_data_t *nd = (node_data_t*)(wasmer_svm_instance_context_node_data_get(ctx));
  return nd->counter;
  return 0;
}

int main() {
  // Credits:
  // original code has been copied and modified from:
  // https://sourcegraph.com/github.com/wasmerio/wasmer/-/blob/lib/runtime-c-api/tests/test-imports.c

  /* const char *module_name = "node"; */
  /* wasmer_byte_array module_name_bytes; */
  /* module_name_bytes.bytes = (const uint8_t *) module_name; */
  /* module_name_bytes.bytes_len = strlen(module_name); */

  /* Prepare import for `inc_counter` */
  /* wasmer_value_tag inc_params_sig[] = {WASM_I32}; */
  /* wasmer_value_tag inc_returns_sig[] = {}; */
  /* wasmer_import_func_t *inc_func = wasmer_import_func_new((void (*)(void *)) inc_counter, inc_params_sig, 1, inc_returns_sig, 0); */
  /* const char *inc_import_name = "inc_counter"; */
  /* wasmer_byte_array inc_import_name_bytes; */
  /* inc_import_name_bytes.bytes = (const uint8_t *) inc_import_name; */
  /* inc_import_name_bytes.bytes_len = strlen(inc_import_name); */
  /* wasmer_import_t inc_func_import; */
  /* inc_func_import.module_name = module_name_bytes; */
  /* inc_func_import.import_name = inc_import_name_bytes; */
  /* inc_func_import.tag = WASM_FUNCTION; */
  /* inc_func_import.value.func = inc_func; */
  /*  */
  /* #<{(| Prepare import for `get_counter` |)}># */
  /* wasmer_value_tag get_params_sig[] = {}; */
  /* wasmer_value_tag get_returns_sig[] = {WASM_I32}; */
  /* wasmer_import_func_t *get_func = wasmer_import_func_new((void (*)(void *)) get_counter, get_params_sig, 0, get_returns_sig, 1); */
  /* const char *get_import_name = "get_counter"; */
  /* wasmer_byte_array get_import_name_bytes; */
  /* get_import_name_bytes.bytes = (const uint8_t *) get_import_name; */
  /* get_import_name_bytes.bytes_len = strlen(get_import_name); */
  /* wasmer_import_t get_func_import; */
  /* get_func_import.module_name = module_name_bytes; */
  /* get_func_import.import_name = get_import_name_bytes; */
  /* get_func_import.tag = WASM_FUNCTION; */
  /* get_func_import.value.func = get_func; */

  // Create Import Object
  uint32_t addr = 0x11223344;
  void* addr_ptr = (void*)(&addr);
  void* node_data = (void*)(new_node_data(5));
  wasmer_import_t imports[] = {};
  uint32_t imports_len = 0;
  wasmer_import_object_t** import_obj_ptr_ptr = (wasmer_import_object_t**)(malloc(sizeof(wasmer_import_object_t*)));
  wasmer_result_t import_result = wasmer_svm_import_object(import_obj_ptr_ptr, addr_ptr, node_data, imports, imports_len);
  assert(import_result == WASMER_OK);

  // Read the wasm file
  FILE *file = fopen("wasm/counter.wasm", "r");
  fseek(file, 0, SEEK_END);
  long bytes_len = ftell(file);
  uint8_t *bytes = malloc(bytes_len);
  fseek(file, 0, SEEK_SET);
  fread(bytes, 1, bytes_len, file);
  fclose(file);

  // Compile wasm into wasmer module
  wasmer_module_t** module_ptr_ptr = (wasmer_module_t**)malloc(sizeof(wasmer_module_t*));
  wasmer_result_t compile_result = wasmer_compile(module_ptr_ptr, bytes, bytes_len);

  wasmer_instance_t** instance_ptr_ptr = (wasmer_instance_t**)malloc(sizeof(wasmer_instance_t*));
  wasmer_result_t instance_res = wasmer_svm_module_instantiate(instance_ptr_ptr, *module_ptr_ptr, *import_obj_ptr_ptr);

  /* assert(instance_res == WASMER_OK); */

  return 0;
}
