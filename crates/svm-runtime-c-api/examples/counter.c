#include "svm.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

typedef struct {
  uint32_t counter;
} host_t;

typedef struct {
  uint8_t* bytes;
  long bytes_len;
} wasm_file_t;

wasm_file_t read_wasm_file(const char *file_name) {
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

uint64_t deploy_contract_bytes(uint8_t **bytes, void *author) {
  // deploy-contract wire format:
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

uint8_t* int32_arg_new(uint32_t value) {
  // we allocate 5 bytes for `int32` arg:
  // 1 - arg type
  // 4 - encoding of `value` (big-endian)
  uint8_t* buf = (uint8_t*)malloc(5);

  // arg type i32 = 0
  buf[0] = 0;

  // `value` is assumed to be laid-out in *little-endian* in memory
  buf[1] = (value >> 24) & 0xFF;
  buf[2] = (value >> 16) & 0xFF;
  buf[3] = (value >> 8) & 0xFF;
  buf[4] = (value >> 0) & 0xFF;

  return buf;
}

uint64_t transaction_exec_bytes(
    uint8_t **bytes,
    void *addr,
    void *sender,
    const char* func_name,
    uint8_t func_name_len,
    uint8_t args_count,
    uint8_t *args_buf,
    uint32_t args_buf_len
    ) {
  // transaction-execution wire format:
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

host_t* host_new(uint32_t counter_initial) {
  host_t* host = (host_t*)malloc(sizeof(host_t));
  host->counter = counter_initial;
  return host;
}

void host_inc_counter(void *ctx, uint32_t value) {
  host_t *host = (host_t*)(svm_instance_context_host_get(ctx));
  host->counter = host->counter + value;
}

uint32_t host_get_counter(void *ctx) {
  host_t *host = (host_t*)(svm_instance_context_host_get(ctx));
  return host->counter;
}

svm_result_t do_contract_deploy(uint8_t **addr, void *runtime, uint8_t *bytes, uint64_t bytes_len) {
  void *contract;
  svm_result_t res;

  res = svm_contract_build(&contract, runtime, (void*)bytes, bytes_len);
  if (res != SVM_SUCCESS) {
    return res;
  }

  uint8_t* addr_ptr = (uint8_t*)svm_contract_derive_address(runtime, contract);

  res = svm_contract_deploy(runtime, contract, (void*)addr_ptr);
  if (res != SVM_SUCCESS) {
    return res;
  }

  *addr = addr_ptr;

  return SVM_SUCCESS;
}

/* svm_import_t create_import(const char *module_name, const char *import_name, svm_import_func_t *func) { */
/*   svm_byte_array module_name_bytes; */
/*   module_name_bytes.bytes = (const uint8_t *) module_name; */
/*   module_name_bytes.bytes_len = strlen(module_name); */
/*  */
/*   svm_byte_array import_name_bytes; */
/*   import_name_bytes.bytes = (const uint8_t *) import_name; */
/*   import_name_bytes.bytes_len = strlen(import_name); */
/*  */
/*   wasmer_import_t import; */
/*   import.module_name = module_name_bytes; */
/*   import.import_name = import_name_bytes; */
/*   import.tag = WASM_FUNCTION; */
/*   import.value.func = func; */
/*  */
/*   return import; */
/* } */

svm_import_t* imports_build() {
  svm_import_t *imports = (svm_import_t*)(malloc(sizeof(svm_import_t) * 0));

  // Prepare import for `host_inc_counter`
  /* wasmer_value_tag inc_params[] = {WASM_I32}; */
  /* wasmer_value_tag inc_returns[] = {}; */
  /* wasmer_import_func_t *inc_func = wasmer_import_func_new((void (*)(void *)) host_inc_counter, inc_params, 1, inc_returns, 0); */
  /* imports[0] = create_import("env", "inc_counter", inc_func); */

  // Prepare import for `host_get_counter`
  /* wasmer_value_tag get_params[] = {}; */
  /* wasmer_value_tag get_returns[] = {WASM_I32}; */
  /* wasmer_import_func_t *get_func = wasmer_import_func_new((void (*)(void *)) host_get_counter, get_params, 0, get_returns, 1); */
  /* imports[1] = create_import("env", "get_counter", get_func); */

  return imports;
}

void* alloc_byte_address(uint8_t byte) {
  void *addr = (void*)malloc(20);
  memset(addr, byte, 20);
  return addr;
}

void* alloc_empty_state() {
  void *state = (void*)malloc(32);
  memset(state, 0, 32);
  return state;
}

int main() {
  svm_result_t res;

  // create a new kv-store
  void *kv = NULL;
  svm_memory_kv_create(&kv);

  // create a new runtime
  void *runtime = NULL;
  uint32_t balance = 10;
  host_t* host = host_new(balance);

  void *imports = (void*)imports_build();
  unsigned int imports_len = 2;

  res = svm_memory_runtime_create(&runtime, kv, host, imports, imports_len);
  assert(res == SVM_SUCCESS);

  /* #<{(| `author address = 0xAA...AA` |)}># */
  /* void *author = alloc_byte_address(0xAA); */
  /*  */
  /* uint8_t *bytes = NULL; */
  /* uint64_t bytes_len = deploy_contract_bytes(&bytes, author); */
  /*  */
  /* uint8_t *addr; */
  /* res = do_contract_deploy(&addr, runtime, bytes, bytes_len); */
  /* assert(res == SVM_SUCCESS); */
  /*  */
  /* printf("Deployed contract successfully...\n"); */
  /* printf("Contract account address:\n"); */
  /*  */
  /* for (int i = 0; i < 20; i++) { */
  /*   printf("%d ", addr[i]); */
  /* } */
  /*  */
  /* printf("\n\n"); */
  /*  */
  /* uint8_t *state = alloc_empty_state(); */
  /* void *sender = alloc_byte_address(0xBB); */
  /*  */
  /* // 1) First we want to assert that the counter has been initialized with `9` as expected (see `create_import_object` above) */
  /* bytes_len = transaction_exec_bytes( */
  /*     &bytes, */
  /*     addr, */
  /*     (void*)sender, */
  /*     "get", */
  /*     strlen("get"), */
  /*     0,    // `args_count = 0` */
  /*     NULL, // `args_buf = NULL` */
  /*     0);   // `args_buf_len = 0` */
  /*  */
  /* void *tx; */
  /* res = svm_transaction_build(&tx, runtime, (void*)bytes, bytes_len); */
  /* assert(res == SVM_SUCCESS); */
  /*  */
  /* void *receipt = NULL; */
  /* uint32_t pages_count = 10; */
  /* res = svm_transaction_exec(&receipt, runtime, tx, state, pages_count); */
  /* assert(res == SVM_SUCCESS); */
  /* assert(svm_receipt_status(receipt) == true); */
  /*  */
  /* const uint8_t *new_state = svm_receipt_new_state(receipt); */
  /*  */
  /* printf("New contract state:\n"); */
  /* for (int i = 0; i < 32; i++) { */
  /*   printf("%02X ", new_state[i]); */
  /* } */
  /*  */
  /* wasmer_value_t *results = NULL; */
  /* uint32_t results_len; */
  /* svm_receipt_results(&results, receipt, &results_len); */
  /* assert(results_len == 1); */
  /* assert(results[0].value.I32 == 10); */
  /*  */
  /* uint8_t *arg = int32_arg_new(7); */
  /*  */
  /* #<{(| 2) Now, let's increment the counter by `7` |)}># */
  /* bytes_len = transaction_exec_bytes( */
  /*     &bytes, */
  /*     addr, */
  /*     (void*)sender, */
  /*     "inc", */
  /*     strlen("inc"), */
  /*     1,     // `args_count = 1` */
  /*     arg,   // `args_buf = [1, 0, 0, 0, 7]` */
  /*     5);    // `args_buf_len = 5` */
  /*  */
  /* res = svm_transaction_build(&tx, runtime, (void*)bytes, bytes_len); */
  /* assert(res == SVM_SUCCESS); */
  /*  */
  /* res = svm_transaction_exec(&receipt, runtime, tx, new_state, pages_count); */
  /* assert(res == SVM_SUCCESS); */
  /* assert(svm_receipt_status(receipt) == true); */
  /*  */
  /* svm_receipt_results(&results, receipt, &results_len); */
  /* assert(results_len == 0); */
  /*  */
  /* // 3) Now, we'll verify that the counter has been modified to `10 + 7 = 17` */
  /* bytes_len = transaction_exec_bytes( */
  /*     &bytes, */
  /*     addr, */
  /*     (void*)sender, */
  /*     "get", */
  /*     strlen("get"), */
  /*     0,    // `args_count = 0` */
  /*     NULL, // `args_buf = NULL` */
  /*     0);   // `args_buf_len = 0` */
  /*  */
  /* res = svm_transaction_build(&tx, runtime, (void*)bytes, bytes_len); */
  /* assert(res == SVM_SUCCESS); */
  /*  */
  /* res = svm_transaction_exec(&receipt, runtime, tx, new_state, pages_count); */
  /* assert(res == SVM_SUCCESS); */
  /* assert(svm_receipt_status(receipt) == true); */
  /*  */
  /* svm_receipt_results(&results, receipt, &results_len); */
  /* assert(results_len == 1); */
  /* assert(results[0].value.I32 == 10 + 7); */
  /*  */
  /* #<{(| // TODO: clearing resources |)}># */
  /* #<{(| free(wasm_file.bytes); |)}># */
  /* #<{(| free(args_buf); |)}># */
  /*  */

  return 0;
}
