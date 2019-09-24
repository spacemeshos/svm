#include "wasmer.h"
#include "svm_wasmer.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

typedef struct {
    uint32_t counter;
} full_node_t;

typedef struct {
    uint8_t* bytes;
    long bytes_len;
} wasm_file_t;

wasm_file_t read_wasm_file(const char* file_name) {
    wasm_file_t wasm_file;

    FILE *file = fopen(file_name, "r");
    fseek(file, 0, SEEK_END);
    wasm_file.bytes_len = ftell(file);

    wasm_file.bytes = malloc(wasm_file.bytes_len);
    fseek(file, 0, SEEK_SET);
    fread(wasm_file.bytes, 1, wasm_file.bytes_len, file);
    fclose(file);

    return wasm_file;
}

full_node_t* full_node_new(uint32_t counter) {
   full_node_t* ptr = (full_node_t*)malloc(sizeof(full_node_t));
   ptr->counter = counter;
   return ptr;
}

void inc_counter_from_reg(wasmer_instance_context_t *ctx, uint32_t reg_idx) {
    uint8_t* reg_bytes = (uint8_t*)svm_register_get(ctx, 64, reg_idx);

    uint8_t a = reg_bytes[0];
    uint8_t b = reg_bytes[1];
    uint8_t c = reg_bytes[2];
    uint8_t d = reg_bytes[3];

    uint32_t amount = a | (b << 8) | (c << 16) | (d << 24);

    full_node_t *nd = (full_node_t*)(svm_instance_context_node_data_get(ctx));
    nd->counter = nd->counter + amount;
}

uint32_t get_counter(wasmer_instance_context_t *ctx) {
    full_node_t *nd = (full_node_t*)(svm_instance_context_node_data_get(ctx));
    return nd->counter;
}

wasmer_import_t create_import(const char *module_name, const char *import_name, wasmer_import_func_t *func) {
    wasmer_byte_array module_name_bytes;
    module_name_bytes.bytes = (const uint8_t *) module_name;
    module_name_bytes.bytes_len = strlen(module_name);

    wasmer_byte_array import_name_bytes;
    import_name_bytes.bytes = (const uint8_t *) import_name;
    import_name_bytes.bytes_len = strlen(import_name);

    wasmer_import_t import;
    import.module_name = module_name_bytes;
    import.import_name = import_name_bytes;
    import.tag = WASM_FUNCTION;
    import.value.func = func;

    return import;
}

wasmer_result_t create_import_object(wasmer_import_object_t** import_object, uint32_t addr, uint32_t state, uint32_t init_counter, wasmer_import_t* imports, uint32_t imports_len) {
    void* addr_ptr = (void*)(&addr);
    void* state_ptr = (void*)(&state);
    void* node = (void*)(full_node_new(init_counter));

    uint32_t max_pages = 5;
    uint32_t max_pages_slices = 100;

    return svm_import_object(import_object, addr_ptr, state_ptr, max_pages, max_pages_slices, node, imports, imports_len);
}

int main() {
    // Credits:
    // original code has been copied and modified from:
    // https://sourcegraph.com/github.com/wasmerio/wasmer/-/blob/lib/runtime-c-api/tests/test-imports.c

    // Prepare import for `inc_counter`
    wasmer_value_tag inc_params_sig[] = {WASM_I32};
    wasmer_value_tag inc_returns_sig[] = {};
    wasmer_import_func_t *inc_func = wasmer_import_func_new((void (*)(void *)) inc_counter_from_reg, inc_params_sig, 1, inc_returns_sig, 0);
    wasmer_import_t inc_import = create_import("node", "inc_counter_from_reg", inc_func);

    // Prepare import for `get_counter`
    wasmer_value_tag get_params_sig[] = {};
    wasmer_value_tag get_returns_sig[] = {WASM_I32};
    wasmer_import_func_t *get_func = wasmer_import_func_new((void (*)(void *)) get_counter, get_params_sig, 0, get_returns_sig, 1);
    wasmer_import_t get_import = create_import("node", "get_counter", get_func);

    wasmer_import_t imports[] = {get_import, inc_import};

    // Create the import-object
    wasmer_import_object_t *import_object;
    wasmer_result_t import_result = create_import_object(&import_object, 0x11223344, 0xAABBCCDD, 9, imports, 2);
    assert(import_result == WASMER_OK);

    // Read the wasm file
    wasm_file_t wasm_file = read_wasm_file("wasm/counter.wasm");

    // Compile wasm into wasmer module
    wasmer_module_t* module = NULL;
    wasmer_result_t compile_res = wasmer_compile(&module, wasm_file.bytes, wasm_file.bytes_len);
    assert(compile_res == WASMER_OK);

    wasmer_instance_t *instance = NULL;
    wasmer_result_t instance_res = wasmer_module_import_instantiate(&instance, module, import_object);
    assert(instance_res == WASMER_OK);

    // First we want to assert that the counter has been initialized with `9`
    wasmer_value_t result_one;
    wasmer_value_t get_params[] = {};
    wasmer_value_t get_results[] = {result_one};
    wasmer_result_t call_result1 = wasmer_instance_call(instance, "get_counter_proxy", get_params, 0, get_results, 1);
    printf("Result: %d\n", get_results[0].value.I32);
    assert(get_results[0].value.I32 == 9);
    assert(call_result1 == WASMER_OK);

    // Now, let's increment the counter by `7`. In order to do that we set register `2` with `7`
    const wasmer_instance_context_t *ctx = wasmer_instance_context_get(instance);
    uint8_t counter[] = {7};
    svm_register_set(ctx, 64, 2, counter, 1);

    wasmer_value_t arg_amount;
    arg_amount.tag = WASM_I32;
    arg_amount.value.I32 = 2; // register `2`
    wasmer_value_t inc_params[] = {arg_amount};
    wasmer_value_t inc_results[] = {};
    wasmer_result_t call_result2 = wasmer_instance_call(instance, "inc_counter_proxy", inc_params, 1, inc_results, 0);
    assert(call_result2 == WASMER_OK);

    // Assert that the counter has been modified to `9 + 7 = 16`
    wasmer_result_t call_result3 = wasmer_instance_call(instance, "get_counter_proxy", get_params, 0, get_results, 1);
    printf("Result: %d\n", get_results[0].value.I32);
    assert(get_results[0].value.I32 == 16);
    assert(call_result3 == WASMER_OK);

    // Clearing resources
    wasmer_import_object_destroy(import_object);
    wasmer_module_destroy(module);
    wasmer_instance_destroy(instance);
    free(wasm_file.bytes);

    return 0;
}
