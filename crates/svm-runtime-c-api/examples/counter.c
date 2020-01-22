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

uint64_t deploy_template_bytes(uint8_t **bytes, uint8_t *author) {
  wasm_file_t file = read_wasm_file("wasm/counter.wasm");

  uint64_t bytes_len =
    4  +  // proto version
    1  +  // name length
    7  +  // `len("Example") = 7`
    20 +  // `len(author-address)`
    2  +  // `#admins`      (we'll set it to `0`)
    2  +  // `#deps`        (we'll set it to `0`)
    2  +  // `page_count`   (we'll set it to `0`)
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

  // set `page_count=0`
  buf[36] = 0;
  buf[37] = 0;

  // set template code length (big-endian)
  uint8_t* code_length = (uint8_t*)&file.bytes_len;

  for (int i = 0; i < 8; i++) {
    // we assume `code_length` in little-endian order
    // so we reverse `wasm_length` since it should be in `big-endian` order
    buf[38 + i] = code_length[7 - i];
  }

  // copy template code
  memcpy(&buf[46], file.bytes, file.bytes_len);

  *bytes = buf;

  return bytes_len;
}


uint64_t spawn_app_bytes(uint8_t **bytes, uint8_t *creator_addr, uint8_t* template_addr) {
  uint64_t bytes_len =
    4  +  //  proto version
    20 +  // `len(creator-address)`
    20;   // `len(template-address)`

  uint8_t* buf = (uint8_t*)(malloc(bytes_len));

  // set `proto=0`
  buf[0] = 0;
  buf[1] = 0;
  buf[2] = 0;
  buf[3] = 0;

  // copy `template` address
  memcpy(&buf[4], template_addr, 20);

  // copy `creator` address
  memcpy(&buf[4 + 20], creator_addr, 20);

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

uint64_t exec_app_bytes(
    uint8_t **bytes,
    void *sender_addr,
    void *app_addr,
    const char* func_name,
    uint8_t func_name_len,
    uint8_t args_count,
    uint8_t *args_buf,
    uint32_t args_buf_len
    ) {
  uint64_t bytes_len =
    4  +   // proto version
    20  +  // app address
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

  // set `app` address
  memcpy(&buf[4], app_addr, 20);

  // set `sender` address
  memcpy(&buf[24], sender_addr, 20);

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

const svm_import_t* inc_counter_import_build() {
  svm_byte_array module_name;
  module_name.bytes = (const uint8_t *)"env";
  module_name.bytes_len = strlen("env");

  svm_byte_array inc_name;
  inc_name.bytes = (const uint8_t *)"inc_counter";
  inc_name.bytes_len = strlen("inc_counter");

  svm_value_type* types = (svm_value_type*)malloc(sizeof(svm_value_type));
  types[0] = SVM_I32;

  svm_value_type_array inc_params;
  inc_params.types = types;
  inc_params.types_len = 1;

  svm_value_type_array inc_returns;
  inc_returns.types = NULL;
  inc_returns.types_len = 0;

  svm_import_t *import = NULL;
  svm_result_t res = svm_import_func_build(&import, module_name, inc_name, host_inc_counter, inc_params, inc_returns);
  assert(res == SVM_SUCCESS);

  return import;
}

const svm_import_t* get_counter_import_build() {
  svm_byte_array module_name;
  module_name.bytes = (const uint8_t *)"env";
  module_name.bytes_len = strlen("env");

  svm_byte_array get_name;
  get_name.bytes = (const uint8_t *)"get_counter";
  get_name.bytes_len = strlen("get_counter");

  svm_value_type_array get_params;
  get_params.types = NULL;
  get_params.types_len = 0;

  svm_value_type* types = (svm_value_type*)malloc(sizeof(svm_value_type));
  types[0] = SVM_I32;

  svm_value_type_array get_returns;
  get_returns.types = types;
  get_returns.types_len = 1;

  svm_import_t *import = NULL;
  svm_result_t res = svm_import_func_build(&import, module_name, get_name, host_get_counter, get_params, get_returns);
  assert(res == SVM_SUCCESS);

  return import;
}

const svm_import_t** imports_build() {
  const svm_import_t** imports = (const svm_import_t**)(malloc(sizeof(const svm_import_t*) * 2));

  imports[0] = inc_counter_import_build();
  imports[1] = get_counter_import_build();

  return imports;
}

uint8_t* alloc_byte_address(uint8_t byte) {
  uint8_t *addr = (uint8_t*)malloc(20);
  memset(addr, byte, 20);
  return addr;
}

uint8_t* alloc_empty_state() {
  uint8_t *state = (uint8_t*)malloc(32);
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

  const svm_import_t **imports = imports_build();
  unsigned int imports_len = 2;

  res = svm_memory_runtime_create(&runtime, kv, host, imports, imports_len);
  assert(res == SVM_SUCCESS);

  /* `author address = 0xAA...AA` */
  uint8_t *author = alloc_byte_address(0xAA);
  uint8_t *bytes = NULL;
  uint64_t bytes_len = deploy_template_bytes(&bytes, author);

  uint8_t *template_addr = NULL;
  res = svm_deploy_template((void**)&template_addr, runtime, (void*)bytes, bytes_len);
  assert(res == SVM_SUCCESS);
  free(bytes);

  printf("Deployed AppTemplate successfully...\n");
  printf("AppTemplate Account Address:\n");

  for (int i = 0; i < 20; i++) {
    printf("%d ", template_addr[i]);
  }

  printf("\n\n");

  uint8_t *app_addr = NULL;
  uint8_t *creator = alloc_byte_address(0xBB);
  bytes_len = spawn_app_bytes(&bytes, creator, template_addr);
  res = svm_spawn_app((void**)&app_addr, runtime, (void*)bytes, bytes_len);
  assert(res == SVM_SUCCESS);
  free(bytes);

  printf("Spawned App successfully...\n");
  printf("App Account Address:\n");

  for (int i = 0; i < 20; i++) {
    printf("%d ", app_addr[i]);
  }

  uint8_t *sender_addr = alloc_byte_address(0xCC);

  /* 1) First we want to assert that the counter has been initialized with `9` as expected (see `create_import_object` above) */
  bytes_len = exec_app_bytes(
      &bytes,
      sender_addr,
      app_addr,
      "get",
      strlen("get"),
      0,    // `args_count = 0`
      NULL, // `args_buf = NULL`
      0);   // `args_buf_len = 0`

  void *app_tx = NULL;
  res = svm_parse_exec_app(&app_tx, runtime, bytes, bytes_len);
  assert(res == SVM_SUCCESS);
  free(bytes);

  void *receipt = NULL;
  uint8_t *state = alloc_empty_state();
  res = svm_exec_app(&receipt, runtime, app_tx, (void*)state);
  assert(res == SVM_SUCCESS);

  assert(svm_receipt_status(receipt) == true);
  const uint8_t *new_state = svm_receipt_new_state(receipt);

  printf("\n\nNew app state:\n");
  for (int i = 0; i < 32; i++) {
    printf("%02X ", new_state[i]);
  }

  svm_value_t *results = NULL;
  uint32_t results_len;
  svm_receipt_results(&results, receipt, &results_len);
  assert(results_len == 1);
  assert(results[0].value.I32 == 10);


  /* 2) Now, let's increment the counter by `7` */

  uint8_t *arg = int32_arg_new(7);
  bytes_len = exec_app_bytes(
      &bytes,
      sender_addr,
      app_addr,
      "inc",
      strlen("inc"),
      1,     // `args_count = 1`
      arg,   // `args_buf = [1, 0, 0, 0, 7]`
      5);    // `args_buf_len = 5`

  res = svm_parse_exec_app(&app_tx, runtime, bytes, bytes_len);
  assert(res == SVM_SUCCESS);
  free(bytes);

  res = svm_exec_app(&receipt, runtime, app_tx, (void*)new_state);
  assert(res == SVM_SUCCESS);
  assert(svm_receipt_status(receipt) == true);

  svm_receipt_results(&results, receipt, &results_len);
  assert(results_len == 0);

  // 3) Now, we'll verify that the counter has been modified to `10 + 7 = 17`
  bytes_len = exec_app_bytes(
      &bytes,
      sender_addr,
      app_addr,
      "get",
      strlen("get"),
      0,    // `args_count = 0`
      NULL, // `args_buf = NULL`
      0);   // `args_buf_len = 0`

  res = svm_parse_exec_app(&app_tx, runtime, bytes, bytes_len);
  assert(res == SVM_SUCCESS);
  free(bytes);

  res = svm_exec_app(&receipt, runtime, app_tx, (void*)new_state);
  assert(res == SVM_SUCCESS);
  assert(svm_receipt_status(receipt) == true);

  svm_receipt_results(&results, receipt, &results_len);
  assert(results_len == 1);
  assert(results[0].value.I32 == 10 + 7);

  /* // TODO: clearing resources */
  /* free(wasm_file.bytes); */
  /* free(args_buf); */

  return 0;
}
