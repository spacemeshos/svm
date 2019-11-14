#include "wasmer.h"
#include "svm_wasmer.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

// Credits:
// wasmer related code has been copied and modified from:
// https://sourcegraph.com/github.com/wasmerio/wasmer/-/blob/lib/runtime-c-api/tests/test-imports.c

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

uint64_t create_wire_contract(uint8_t **bytes, void* author) {
    // https://github.com/spacemeshos/svm/blob/master/crates/svm-contract/src/wire/deploy/mod.rs
    wasm_file_t file = read_wasm_file("wasm/counter.wasm");

    uint64_t bytes_len =
      4  +  // proto version
      1  +  // name length
      7  +  // `len("Example") = 7`
      20 +  // `len(author-address)`
      2  +  // `#admins` (we'll set it to `0`)
      2  +  // `#deps`   (we'll set it to `0`)
      8  +  //  wasm code length (big-endian)
      (uint64_t)file.bytes_len;  // the wasm code

    uint8_t* buf = (uint8_t*)(malloc(bytes_len));

    // set `proto=0`
    buf[0] = 0;
    buf[1] = 0;
    buf[2] = 0;
    buf[3] = 0;

    // set `name_length=7`
    buf[4] = 7;

    // set `name="Example"` (no terminating `NULL`)
    const char* name = "Example";
    memcpy(&buf[5], name, 7);

    // set `author address`
    memcpy(&buf[12], author, 20);

    // set `#admins=0`
    buf[32] = 0;
    buf[33] = 0;

    // set `#deps=0`
    buf[34] = 0;
    buf[35] = 0;

    // set contract wasm length (big-endian)
    uint8_t* wasm_length = (uint8_t*)&file.bytes_len;

    for (int i = 0; i < 8; i++) {
        // we assume `wasm_length` in little-endian order
        // so we reverse `wasm_length` since it should be in `big-endian` order
        buf[36 + i] = wasm_length[7 - i];
    }

    // copy contract wasm
    memcpy(&buf[44], file.bytes, file.bytes_len);

    *bytes = buf;

    return bytes_len;
}

uint8_t* create_wire_int32_arg(uint32_t value) {
    // we allocate 5 bytes for `int32` arg:
    // 1 - arg type
    // 4 - encoding of `value` (big-endian)
    uint8_t* arg_buf = (uint8_t*)malloc(5);

    // arg type i32 = 0
    arg_buf[0] = 0;

    // `value` is assumed to be laid-out in *little-endian* in memory
    arg_buf[1] = (value >> 24) & 0xFF;
    arg_buf[2] = (value >> 16) & 0xFF;
    arg_buf[3] = (value >> 8) & 0xFF;
    arg_buf[4] = (value >> 0) & 0xFF;

    return arg_buf;
}

uint64_t create_wire_transaction(
    uint8_t **bytes,
    void *addr,
    void *sender,
    const char* func_name,
    uint8_t func_name_len,
    uint8_t args_count,
    uint8_t* args_buf,
    uint32_t args_buf_len
) {
    // https://github.com/spacemeshos/svm/blob/master/crates/svm-contract/src/wire/exec/mod.rs

    uint64_t bytes_len =
      4  +   // proto version
      20  +  // contract address
      20  +  // sender address
      1   +  // function name length
      (uint64_t)func_name_len +  // `len(func_name0)`
      1   +  // #args
      args_buf_len; // `len(arg_buf)`

    uint8_t* buf = (uint8_t*)(malloc(bytes_len));

    // set `proto=0`
    buf[0] = 0;
    buf[1] = 0;
    buf[2] = 0;
    buf[3] = 0;

    // set contract address
    memcpy(&buf[4], addr, 20);

    // set sender address
    memcpy(&buf[24], sender, 20);

    // set `func_name_len`
    buf[44] = func_name_len;

    // set `func_name`
    memcpy(&buf[45], func_name, func_name_len);

    // set `#args_count`
    buf[45 + func_name_len] = args_count;

    // `args_buf`
    memcpy(&buf[45 + func_name_len + args_count], args_buf, args_buf_len);

    *bytes = buf;

    return bytes_len;
}

full_node_t* full_node_new(uint32_t counter) {
   full_node_t* node = (full_node_t*)malloc(sizeof(full_node_t));
   node->counter = counter;
   return node;
}

full_node_t* extract_ctx_nod(wasmer_instance_context_t *ctx) {
    return (full_node_t*)(svm_instance_context_node_data_get(ctx));
}

void vmcall_inc_counter(wasmer_instance_context_t *ctx, uint32_t value) {
    full_node_t *node = extract_ctx_nod(ctx);
    node->counter = node->counter + value;
}

uint32_t vmcall_get_counter(wasmer_instance_context_t *ctx) {
    full_node_t *node = extract_ctx_nod(ctx);
    return node->counter;
}

wasmer_result_t contract_deploy(uint8_t **addr, uint8_t* bytes, uint64_t bytes_len) {
    void *contract;

    wasmer_result_t build_res = svm_contract_build(&contract, (void*)bytes, bytes_len);
    if (build_res != WASMER_OK) {
        return build_res;
    }

    uint8_t* buf = (uint8_t*)svm_contract_compute_address(contract);

    wasmer_result_t store_res = svm_contract_store(contract, (void*)buf);
    if (store_res != WASMER_OK) {
        return store_res;
    }

    *addr = buf;

    return WASMER_OK;
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

wasmer_result_t create_import_object(
    wasmer_import_object_t** import_object,
    void *addr,
    void *state,
    uint32_t init_counter,
    wasmer_import_t* imports,
    uint32_t imports_len
) {
    void* node = (void*)(full_node_new(init_counter));

    uint32_t max_pages = 5;
    uint32_t max_pages_slices = 100;

    return svm_import_object((void**)import_object, addr, state, max_pages, max_pages_slices, node, imports, imports_len);
}

wasmer_import_t* prepare_imports() {
    // Prepare import for `inc_counter`
    wasmer_value_tag inc_params_sig[] = {WASM_I32};
    wasmer_value_tag inc_returns_sig[] = {};
    wasmer_import_func_t *inc_func = wasmer_import_func_new((void (*)(void *)) vmcall_inc_counter, inc_params_sig, 1, inc_returns_sig, 0);
    wasmer_import_t inc_import = create_import("node", "vmcall_inc_counter", inc_func);

    // Prepare import for `get_counter`
    wasmer_value_tag get_params_sig[] = {};
    wasmer_value_tag get_returns_sig[] = {WASM_I32};
    wasmer_import_func_t *get_func = wasmer_import_func_new((void (*)(void *)) vmcall_get_counter, get_params_sig, 0, get_returns_sig, 1);
    wasmer_import_t get_import = create_import("node", "vmcall_get_counter", get_func);

    wasmer_import_t *imports = (wasmer_import_t*)(malloc(sizeof(wasmer_import_t) * 2));
    imports[0] = inc_import;
    imports[1] = get_import;

    return imports;
}

int main() {
    // `author address = 0xAA..AA`
    void *author = (void*)malloc(20);
    memset(author, 0xBB, 20);

    uint8_t *deploy_bytes;
    uint64_t deploy_bytes_len = create_wire_contract(&deploy_bytes, author);

    uint8_t *addr;
    wasmer_result_t deploy_res = contract_deploy(&addr, deploy_bytes, deploy_bytes_len);
    assert(deploy_res == WASMER_OK);

    printf("Deployed contract successfully...\n");
    printf("Contract account address:\n");

    for (int i = 0; i < 20; i++) {
        printf("%d ", addr[i]);
    }

    printf("\n\n");

    wasmer_import_t* imports = prepare_imports();

    // `state` consists of 32 bytes
    uint8_t *state = (uint8_t*)malloc(32);

    // we'll run with a zero-state (`00...00`)
    memset(state, 0, 32);

    // import object
    wasmer_import_object_t *import_object;
    wasmer_result_t import_result = create_import_object(&import_object,(void*)addr, (void*)state, 9, imports, 2);
    assert(import_result == WASMER_OK);

    // `sender address = 0xBB..BB`
    uint8_t *sender = (uint8_t*)malloc(20);
    memset(sender, 0xBB, 20);

    // 1) First we want to assert that the counter has been initialized with `9` as expected (see `create_import_object` above)
    uint8_t *tx1_bytes;
    uint64_t tx1_bytes_len = create_wire_transaction(
        &tx1_bytes,
        addr,
        (void*)sender,
        "get",
        strlen("get"),
        0,    // `args_count = 0`
        NULL, // `args_buf = NULL`
        0);   // `args_buf_len = 0`

    void *tx1;
    wasmer_result_t tx1_res = svm_transaction_build(&tx1, (void*)tx1_bytes, tx1_bytes_len);
    assert(tx1_res == WASMER_OK);

    void *receipt1;
    wasmer_result_t exec1_res = svm_transaction_exec(&receipt1, tx1, import_object);
    assert(exec1_res == WASMER_OK);
    assert(svm_receipt_status(receipt1) == true);

    const uint8_t *new_state = svm_receipt_new_state(receipt1);

    printf("New contract state:\n");
    for (int i = 0; i < 32; i++) {
        printf("%02X ", new_state[i]);
    }

    wasmer_value_t *results1;
    uint32_t results1_len;
    svm_receipt_results(receipt1, &results1, &results1_len);

    assert(results1_len == 1);
    wasmer_value_t result = results1[0];
    assert(result.value.I32 == 9);

    uint8_t *arg_buf = create_wire_int32_arg(7);

    /* 2) Now, let's increment the counter by `7` */
    uint8_t *tx2_bytes;
    uint64_t tx2_bytes_len = create_wire_transaction(
        &tx2_bytes,
        addr,
        (void*)sender,
        "inc",
        strlen("inc"),
        1,       // `args_count = 1`
        arg_buf, // `args_buf = [1, 0, 0, 0, 7]`
        5);      // `args_buf_len = 5`

    void *tx2;
    wasmer_result_t tx2_res = svm_transaction_build(&tx2, (void*)tx2_bytes, tx2_bytes_len);
    assert(tx2_res == WASMER_OK);

    void *receipt2;
    wasmer_result_t exec2_res = svm_transaction_exec(&receipt2, tx2, import_object);
    assert(exec2_res == WASMER_OK);
    assert(svm_receipt_status(receipt2) == true);

    wasmer_value_t *results2;
    uint32_t results2_len;
    svm_receipt_results(receipt2, &results2, &results2_len);
    assert(results2_len == 0);

    // 3) Now, we'll verify that the counter has been modified to `9 + 7 = 16`
    uint8_t *tx3_bytes;
    uint64_t tx3_bytes_len = create_wire_transaction(
        &tx3_bytes,
        addr,
        (void*)sender,
        "get",
        strlen("get"),
        0,    // `args_count = 0`
        NULL, // `args_buf = NULL`
        0);   // `args_buf_len = 0`

    void *tx3;
    wasmer_result_t tx3_res = svm_transaction_build(&tx3, (void*)tx3_bytes, tx3_bytes_len);
    assert(tx3_res == WASMER_OK);

    void *receipt3;
    wasmer_result_t exec3_res = svm_transaction_exec(&receipt3, tx3, import_object);
    assert(exec3_res == WASMER_OK);
    assert(svm_receipt_status(receipt3) == true);

    wasmer_value_t *results3;
    uint32_t results3_len;
    svm_receipt_results(receipt3, &results3, &results3_len);
    assert(results3_len == 1);

    wasmer_value_t result3 = results3[0];
    for (int i = 0; i < 10; i++) {
        printf("%d  ", result3.value.I32);
    }

    assert(result3.value.I32 == 16);

    /* // TODO: clearing resources */
    /* wasmer_import_object_destroy(import_object); */
    /* wasmer_module_destroy(module); */
    /* wasmer_instance_destroy(instance); */
    /* free(wasm_file.bytes); */
    /* free(args_buf); */

    return 0;
}
