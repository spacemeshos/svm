#include "svm.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

// Credits:
// wasmer related code has been copied and modified from:
// https://sourcegraph.com/github.com/wasmerio/wasmer/-/blob/lib/runtime-c-api/tests/test-imports.c

typedef struct {
    uint32_t counter;
} host_t;

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

uint64_t deploy_contract_bytes(uint8_t **bytes, void* author) {
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

uint64_t transaction_exec_bytes(
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

host_t* host_new(uint32_t counter) {
   host_t* host = (host_t*)malloc(sizeof(host_t));
   host->counter = counter;
   return host;
}

void host_inc_counter(wasmer_instance_context_t *ctx, uint32_t value) {
    host_t *host = (host_t*)(svm_instance_context_host_get(ctx));
    host->counter = host->counter + value;
}

uint32_t host_get_counter(wasmer_instance_context_t *ctx) {
    host_t *host = (host_t*)(svm_instance_context_host_get(ctx));
    return host->counter;
}

wasmer_result_t do_contract_deploy(uint8_t **addr, uint8_t* bytes, uint64_t bytes_len) {
    void *contract;

    wasmer_result_t res = svm_contract_build(&contract, (void*)bytes, bytes_len);
    if (res != WASMER_OK) {
        return res;
    }

    uint8_t* buf = (uint8_t*)svm_contract_derive_address(contract);

    wasmer_result_t res = svm_contract_deploy(contract, (void*)buf);
    if (res != WASMER_OK) {
        return res;
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

wasmer_import_t* imports_build() {
    wasmer_import_t *imports = (wasmer_import_t*)(malloc(sizeof(wasmer_import_t) * 2));

    // Prepare import for `host_inc_counter`
    wasmer_value_tag params[] = {WASM_I32};
    wasmer_value_tag returns[] = {};
    wasmer_import_func_t *func = wasmer_import_func_new((void (*)(void *)) host_inc_counter, params, 1, returns, 0);
    imports[0] = create_import("env", "inc_counter", func);

    // Prepare import for `host_get_counter`
    wasmer_value_tag params[] = {};
    wasmer_value_tag returns[] = {WASM_I32};
    wasmer_import_func_t *func = wasmer_import_func_new((void (*)(void *)) host_get_counter, params, 0, returns, 1);
    imports[1] = create_import("env", "get_counter", func);

    return imports;
}

int main() {
    // `author address = 0xAA...AA`
    void *author = (void*)malloc(20);
    memset(author, 0xAA, 20);

    uint8_t *bytes;
    uint64_t bytes_len = deploy_contract_bytes(&deploy_bytes, author);

    uint8_t *addr;
    wasmer_result_t res = do_contract_deploy(&addr, bytes, bytes_len);
    assert(res == WASMER_OK);

    printf("Deployed contract successfully...\n");
    printf("Contract account address:\n");

    for (int i = 0; i < 20; i++) {
        printf("%d ", addr[i]);
    }

    printf("\n\n");

    wasmer_import_t* imports = imports_build();

    // `state` consists of 32 bytes
    uint8_t *state = (uint8_t*)malloc(32);

    // we'll run with a zero-state (`00...00`)
    memset(state, 0, 32);

    // `sender address = 0xBB..BB`
    uint8_t *sender = (uint8_t*)malloc(20);
    memset(sender, 0xBB, 20);

    // 1) First we want to assert that the counter has been initialized with `9` as expected (see `create_import_object` above)
    uint8_t *bytes;
    uint64_t bytes_len = transaction_exec_bytes(
        &bytes,
        addr,
        (void*)sender,
        "get",
        strlen("get"),
        0,    // `args_count = 0`
        NULL, // `args_buf = NULL`
        0);   // `args_buf_len = 0`

    void *tx1;
    wasmer_result_t res = svm_transaction_build(&tx1, (void*)bytes, bytes_len);
    assert(res == WASMER_OK);

    void *receipt1;
    wasmer_result_t res = svm_transaction_exec(&receipt1, tx1);
    assert(res == WASMER_OK);
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
    uint8_t *bytes;
    uint64_t bytes_len = transaction_exec_bytes(
        &bytes,
        addr,
        (void*)sender,
        "inc",
        strlen("inc"),
        1,       // `args_count = 1`
        arg_buf, // `args_buf = [1, 0, 0, 0, 7]`
        5);      // `args_buf_len = 5`

    void *tx2;
    wasmer_result_t res = svm_transaction_build(&tx2, (void*)bytes, bytes_len);
    assert(res == WASMER_OK);

    void *receipt2;
    wasmer_result_t res = svm_transaction_exec(&receipt2, tx2);
    assert(res == WASMER_OK);
    assert(svm_receipt_status(receipt2) == true);

    wasmer_value_t *results2;
    uint32_t results2_len;
    svm_receipt_results(receipt2, &results2, &results2_len);
    assert(results2_len == 0);

    // 3) Now, we'll verify that the counter has been modified to `9 + 7 = 16`
    uint8_t *bytes;
    uint64_t bytes_len = transaction_exec_bytes(
        &bytes,
        addr,
        (void*)sender,
        "get",
        strlen("get"),
        0,    // `args_count = 0`
        NULL, // `args_buf = NULL`
        0);   // `args_buf_len = 0`

    void *tx3;
    wasmer_result_t res = svm_transaction_build(&tx3, (void*)bytes, bytes_len);
    assert(res == WASMER_OK);

    void *receipt3;
    wasmer_result_t res = svm_transaction_exec(&receipt3, tx3);
    assert(res == WASMER_OK);
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
    /* wasmer_module_destroy(module); */
    /* wasmer_instance_destroy(instance); */
    /* free(wasm_file.bytes); */
    /* free(args_buf); */

    return 0;
}
