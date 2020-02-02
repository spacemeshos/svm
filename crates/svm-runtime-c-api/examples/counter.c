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
  long length;
} wasm_file_t;

typedef struct {
  svm_byte_array app_addr;
  svm_byte_array init_state;
} spawned_app_t;

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

svm_byte_array deploy_template_bytes() {
  wasm_file_t file = read_wasm_file("wasm/counter.wasm");

  uint64_t length =
    4  +  // proto version
    1  +  // name length
    7  +  // `len("Example") = 7`
    2  +  // `#admins`      (we'll set it to `0`)
    2  +  // `#deps`        (we'll set it to `0`)
    2  +  // `page_count`   (we'll set it to `0`)
    8  +  //  wasm code length (big-endian)
    (uint64_t)file.length;  // the wasm code

  uint8_t* buf = (uint8_t*)(malloc(length));

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

  // set `#admins=0`
  buf[12] = 0;
  buf[13] = 0;

  // set `#deps=0`
  buf[14] = 0;
  buf[15] = 0;

  // set `page_count=0`
  buf[16] = 0;
  buf[17] = 0;

  // set template code length (big-endian)
  uint8_t* code_length = (uint8_t*)&file.length;

  for (int i = 0; i < 8; i++) {
    // we assume `code_length` in Little-Endian order
    // so we reverse `wasm_length` since it should be in Big-Endian order
    buf[18 + i] = code_length[7 - i];
  }

  // copy template code
  memcpy(&buf[26], file.bytes, file.length);

  svm_byte_array template;
  template.bytes = buf;
  template.length = length;

  return template;
}


svm_byte_array spawn_app_bytes(svm_byte_array template_addr) {
  uint64_t length =
    4  +  // proto version
    template_addr.length;   // length(`template_addr)

  uint8_t* buf = (uint8_t*)(malloc(length));

  // set `proto=0`
  buf[0] = 0;
  buf[1] = 0;
  buf[2] = 0;
  buf[3] = 0;

  // copy `template` address
  memcpy(&buf[4], template_addr.bytes, template_addr.length);

  svm_byte_array app;
  app.bytes = buf;
  app.length = length;

  return app;
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
  uint64_t length =
    4  +   // proto version
    20  +  // app address
    20  +  // sender address
    1   +  // function name length
    (uint64_t)func_name_len +  // `len(func_name0)`
    1   +  // #args
    args_buf_len; // `len(arg_buf)`

  uint8_t* buf = (uint8_t*)(malloc(length));

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

  return length;
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

void inc_counter_import_build(void* imports) { 
  svm_byte_array module_name;
  module_name.bytes = (const uint8_t *)"env";
  module_name.length = strlen("env");

  svm_byte_array import_name;
  import_name.bytes = (const uint8_t *)"inc_counter";
  import_name.length = strlen("inc_counter");

  svm_value_type* types = (svm_value_type*)malloc(sizeof(svm_value_type) * 1);
  types[0] = SVM_I32;

  svm_value_type_array params;
  params.types = types;
  params.length = 1;

  svm_value_type_array returns;
  returns.types = NULL;
  returns.length = 0;

  void* func = (void*)host_inc_counter;

  svm_result_t res = svm_import_func_build(imports, module_name, import_name, func, params, returns);
  assert(res == SVM_SUCCESS);
}

void get_counter_import_build(void* imports) {
  svm_byte_array module_name;
  module_name.bytes = (const uint8_t *)"env";
  module_name.length = strlen("env");

  svm_byte_array import_name;
  import_name.bytes = (const uint8_t *)"get_counter";
  import_name.length = strlen("get_counter");

  svm_value_type_array params;
  params.types = NULL;
  params.length = 0;

  svm_value_type* types = (svm_value_type*)malloc(sizeof(svm_value_type) * 1);
  types[0] = SVM_I32;

  svm_value_type_array returns;
  returns.types = types;
  returns.length = 1;

  void* func = (void*)host_get_counter;

  svm_result_t res = svm_import_func_build(imports, module_name, import_name, func, params, returns);
  assert(res == SVM_SUCCESS);
}

void* imports_build() {
  uint32_t length = 2;
  void* imports = NULL;

  svm_result_t res = svm_imports_alloc(&imports, length);
  assert(res == SVM_SUCCESS);

  inc_counter_import_build(imports);
  get_counter_import_build(imports);

  return imports;
}

void* runtime_create(void* imports) {
  // create a new kv-store
  void *kv = NULL;
  svm_memory_kv_create(&kv);

  uint32_t balance = 10;
  host_t* host = host_new(balance);

  void *runtime = NULL;
  svm_result_t res = svm_memory_runtime_create(&runtime, kv, host, imports); 
  assert(res == SVM_SUCCESS); 
  return runtime;
}

void* alloc_byte_address(uint8_t byte) {
  uint8_t *addr = (uint8_t*)malloc(20);
  memset(addr, byte, 20);
  return (void*)addr;
}

void* alloc_author_addr() {
  return alloc_byte_address(0xAA);
}  

void* alloc_creator_addr() {
  return alloc_byte_address(0xBB);
}  

void* alloc_sender_addr() {
  return alloc_byte_address(0xCC);
}  

void* alloc_empty_state() {
  uint8_t *state = (uint8_t*)malloc(32);
  memset(state, 0, 32);
  return (void*)state;
}

svm_byte_array simulate_deploy_template(svm_byte_array bytes, void* author) {
  void* imports = imports_build();
  void* runtime = runtime_create(imports);

  svm_byte_array host_ctx; 
  host_ctx.bytes = NULL;
  host_ctx.length = 0;

  svm_byte_array template_addr; 
  svm_result_t res = svm_deploy_template(&template_addr, runtime, author, host_ctx, bytes); 

  svm_runtime_destroy(runtime);

  assert(res == SVM_SUCCESS); 

  printf("Deployed AppTemplate successfully...\n");
  for(int i = 0; i < template_addr.length; i++) {
    printf("%d ", template_addr.bytes[i]);
  }
  printf("\n\n");

  return template_addr;
}

spawned_app_t simulate_spawn_app(svm_byte_array bytes, void* creator) {
  void* imports = imports_build();
  void* runtime = runtime_create(imports);

  svm_byte_array host_ctx; 
  host_ctx.bytes = NULL;
  host_ctx.length = 0;

  svm_byte_array app_addr; 
  svm_byte_array init_state; 
  svm_result_t res = svm_spawn_app(&app_addr, &init_state, runtime, creator, host_ctx, bytes);

  svm_runtime_destroy(runtime);

  assert(res == SVM_SUCCESS); 

  printf("Spawned App successfully...\n");
  printf("App Account Address:\n");

  for (int i = 0; i < app_addr.length; i++) {
    printf("%d ", app_addr.bytes[i]);
  }

  printf("\n\n");
  printf("App initial state:\n");
  for (int i = 0; i < init_state.length; i++) {
    printf("%d ", init_state.bytes[i]);
  }
  printf("\n\n");

  spawned_app_t spawned;
  spawned.app_addr = app_addr;
  spawned.init_state = init_state;
  return spawned;
}

  
int main() {
  svm_byte_array bytes;

  // 1) Deploy Template
  void* author = alloc_author_addr();
  bytes = deploy_template_bytes(); 
  svm_byte_array template_addr = simulate_deploy_template(bytes, author);

  // 2) Spawn App
  void* creator = alloc_creator_addr();
  bytes = spawn_app_bytes(template_addr); 
  spawned_app_t spawned = simulate_spawn_app(bytes, creator);

  /* /\* 1) First we want to assert that the counter has been initialized with `9` as expected (see `create_import_object` above) *\/ */
  /* length = exec_app_bytes( */
  /*     &bytes, */
  /*     sender_addr, */
  /*     app_addr, */
  /*     "get", */
  /*     strlen("get"), */
  /*     0,    // `args_count = 0` */
  /*     NULL, // `args_buf = NULL` */
  /*     0);   // `args_buf_len = 0` */

  /* void *app_tx = NULL; */
  /* res = svm_parse_exec_app(&app_tx, runtime, bytes, length); */
  /* assert(res == SVM_SUCCESS); */
  /* free(bytes); */

  /* void *receipt = NULL; */
  /* uint8_t *state = alloc_empty_state(); */
  /* res = svm_exec_app(&receipt, runtime, app_tx, (void*)state); */
  /* assert(res == SVM_SUCCESS); */

  /* assert(svm_receipt_status(receipt) == true); */
  /* const uint8_t *new_state = svm_receipt_new_state(receipt); */

  /* printf("\n\nNew app state:\n"); */
  /* for (int i = 0; i < 32; i++) { */
  /*   printf("%02X ", new_state[i]); */
  /* } */

  /* svm_value_t *results = NULL; */
  /* uint32_t results_len; */
  /* svm_receipt_results(&results, receipt, &results_len); */
  /* assert(results_len == 1); */
  /* assert(results[0].value.I32 == 10); */


  /* /\* 2) Now, let's increment the counter by `7` *\/ */
  /* uint8_t *arg = int32_arg_new(7); */

  /* length = exec_app_bytes( */
  /*     &bytes, */
  /*     sender_addr, */
  /*     app_addr, */
  /*     "inc", */
  /*     strlen("inc"), */
  /*     1,     // `args_count = 1` */
  /*     arg,   // `args_buf = [1, 0, 0, 0, 7]` */
  /*     5);    // `args_buf_len = 5` */

  /* res = svm_parse_exec_app(&app_tx, runtime, bytes, length); */
  /* assert(res == SVM_SUCCESS); */
  /* free(bytes); */

  /* res = svm_exec_app(&receipt, runtime, app_tx, (void*)new_state); */
  /* assert(res == SVM_SUCCESS); */
  /* assert(svm_receipt_status(receipt) == true); */

  /* svm_receipt_results(&results, receipt, &results_len); */
  /* assert(results_len == 0); */

  /* // 3) Now, we'll verify that the counter has been modified to `10 + 7 = 17` */
  /* length = exec_app_bytes( */
  /*     &bytes, */
  /*     sender_addr, */
  /*     app_addr, */
  /*     "get", */
  /*     strlen("get"), */
  /*     0,    // `args_count = 0` */
  /*     NULL, // `args_buf = NULL` */
  /*     0);   // `args_buf_len = 0` */

  /* res = svm_parse_exec_app(&app_tx, runtime, bytes, length); */
  /* assert(res == SVM_SUCCESS); */
  /* free(bytes); */

  /* void*res = svm_exec_app(&receipt, runtime, app_tx, (void*)new_state); *\/ */
  /* assert(res == SVM_SUCCESS); */

  /* svm_receipt_results(&results, receipt, &results_len); */
  /* assert(results_len == 1); */
  /* assert(results[0].value.I32 == 10 + 7); */

  // destroy...

  return 0;
}
