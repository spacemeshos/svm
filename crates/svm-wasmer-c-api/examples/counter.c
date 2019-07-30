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

  const char *module_name = "node";
  wasmer_byte_array module_name_bytes;
  module_name_bytes.bytes = (const uint8_t *) module_name;
  module_name_bytes.bytes_len = strlen(module_name);

  /* Prepare import for `inc_counter` */
  wasmer_value_tag inc_params_sig[] = {WASM_I32};
  wasmer_value_tag inc_returns_sig[] = {};
  wasmer_import_func_t *inc_func = wasmer_import_func_new((void (*)(void *)) inc_counter, inc_params_sig, 1, inc_returns_sig, 0);
  const char *inc_import_name = "inc_counter";
  wasmer_byte_array inc_import_name_bytes;
  inc_import_name_bytes.bytes = (const uint8_t *) inc_import_name;
  inc_import_name_bytes.bytes_len = strlen(inc_import_name);
  wasmer_import_t inc_func_import;
  inc_func_import.module_name = module_name_bytes;
  inc_func_import.import_name = inc_import_name_bytes;
  inc_func_import.tag = WASM_FUNCTION;
  inc_func_import.value.func = inc_func;

  /* Prepare import for `get_counter`  */
  wasmer_value_tag get_params_sig[] = {};
  wasmer_value_tag get_returns_sig[] = {WASM_I32};
  wasmer_import_func_t *get_func = wasmer_import_func_new((void (*)(void *)) get_counter, get_params_sig, 0, get_returns_sig, 1);
  const char *get_import_name = "get_counter";
  wasmer_byte_array get_import_name_bytes;
  get_import_name_bytes.bytes = (const uint8_t *) get_import_name;
  get_import_name_bytes.bytes_len = strlen(get_import_name);
  wasmer_import_t get_func_import;
  get_func_import.module_name = module_name_bytes;
  get_func_import.import_name = get_import_name_bytes;
  get_func_import.tag = WASM_FUNCTION;
  get_func_import.value.func = get_func;

  // Create Import Object
  uint32_t addr = 0x11223344;
  void* addr_ptr = (void*)(&addr);
  void* node_data = (void*)(new_node_data(9));
  wasmer_import_t imports[] = {get_func_import, inc_func_import};
  uint32_t imports_len = 2;
  wasmer_import_object_t* import_object = NULL;
  wasmer_result_t import_result = wasmer_svm_import_object(&import_object, addr_ptr, 5, 100, node_data, imports, imports_len);
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
  wasmer_module_t* module = NULL;
  wasmer_result_t compile_res = wasmer_compile(&module, bytes, bytes_len);
  assert(compile_res == WASMER_OK);

  wasmer_instance_t *instance = NULL;
  wasmer_result_t instance_res = wasmer_svm_module_instantiate(&instance, module, import_object);
  assert(instance_res == WASMER_OK);

  // First we want to assert that the counter has been initialized with `9`
  wasmer_value_t result_one;
  wasmer_value_t get_params[] = {};
  wasmer_value_t get_results[] = {result_one};
  wasmer_result_t call_result1 = wasmer_instance_call(instance, "get_counter_proxy", get_params, 0, get_results, 1);
  printf("Result: %d\n", get_results[0].value.I32);
  assert(get_results[0].value.I32 == 9);
  assert(call_result1 == WASMER_OK);

  // Now, let's increment the counter by `7`
  wasmer_value_t arg_amount;
  arg_amount.tag = WASM_I32;
  arg_amount.value.I32 = 7;
  wasmer_value_t inc_params[] = {arg_amount};
  wasmer_value_t inc_results[] = {};
  wasmer_result_t call_result2 = wasmer_instance_call(instance, "inc_counter_proxy", inc_params, 1, inc_results, 0);
  assert(call_result2 == WASMER_OK);

  // Assert that the counter has been modified to `9 + 7 = 16`
  wasmer_result_t call_result3 = wasmer_instance_call(instance, "get_counter_proxy", get_params, 0, get_results, 1);
  printf("Result: %d\n", get_results[0].value.I32);
  assert(get_results[0].value.I32 == 16);
  assert(call_result3 == WASMER_OK);

  wasmer_import_object_destroy(import_object);
  wasmer_module_destroy(module);
  wasmer_instance_destroy(instance);
  free(bytes);

  return 0;
}
