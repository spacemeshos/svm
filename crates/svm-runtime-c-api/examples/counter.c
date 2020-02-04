#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

#include "svm.h"
#include "constants.h"

#include "wasm_file.h"
#include "deploy_bytes.h"
#include "spawn_app_bytes.h"
#include "exec_app_bytes.h"

#include "func_buf.h"
#include "func_args.h"
#include "func_rets.h"
#include "receipt.h"
#include "host_ctx.h"

typedef struct {
  uint32_t counter;
} host_t;

typedef struct {
  svm_byte_array app_addr;
  svm_byte_array init_state;
} spawned_app_t;

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
  svm_byte_array module_name = {
    .bytes = (const uint8_t *)"env",
    .length = strlen("env")
  };

  svm_byte_array import_name = {
    .bytes = (const uint8_t *)"inc_counter",
    .length = strlen("inc_counter")
  };

  svm_value_type* types = (svm_value_type*)malloc(sizeof(svm_value_type) * 1);
  types[0] = SVM_I32;

  svm_value_type_array params = {
    .types = types,
    .length = 1
  };

  svm_value_type_array returns = {
    .types = NULL,
    .length = 0
  };

  void* func = (void*)host_inc_counter;

  svm_result_t res = svm_import_func_build(imports, module_name, import_name, func, params, returns);
  assert(res == SVM_SUCCESS);
}

void get_counter_import_build(void* imports) {
  svm_byte_array module_name = {
    .bytes = (const uint8_t *)"env",
    .length = strlen("env")
  };

  svm_byte_array import_name = {
    .bytes = (const uint8_t *)"get_counter",
    .length = strlen("get_counter")
  };

  svm_value_type_array params = {
    .types = NULL,
    .length = 0
  };

  svm_value_type* types = (svm_value_type*)malloc(sizeof(svm_value_type) * 1);
  types[0] = SVM_I32;

  svm_value_type_array returns = {
    .types = types,
    .length = 1
  };

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

void* runtime_create(host_t* host, void* imports) {
  // create a new kv-store
  void *kv = NULL;
  svm_memory_kv_create(&kv);

  void *runtime = NULL;
  svm_result_t res = svm_memory_runtime_create(&runtime, kv, host, imports); 
  assert(res == SVM_SUCCESS); 
  return runtime;
}

void* alloc_byte_address(uint8_t byte) {
  uint8_t *addr = (uint8_t*)malloc(SVM_ADDR_LEN);
  memset(addr, byte, SVM_ADDR_LEN);
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
  uint8_t *state = (uint8_t*)malloc(SVM_STATE_LEN);
  memset(state, 0, SVM_STATE_LEN);
  return (void*)state;
}

svm_byte_array simulate_deploy_template(void* runtime, svm_byte_array bytes, void* author) {
  svm_byte_array host_ctx = host_ctx_empty_bytes();

  svm_byte_array template_addr; 
  svm_result_t res = svm_deploy_template(&template_addr, runtime, author, host_ctx, bytes); 

  assert(res == SVM_SUCCESS); 

  printf("Deployed AppTemplate successfully...\n");
  for(int i = 0; i < template_addr.length; i++) {
    printf("%d ", template_addr.bytes[i]);
  }
  printf("\n\n");

  svm_byte_array_destroy(host_ctx);

  return template_addr;
}

spawned_app_t simulate_spawn_app(void* runtime, svm_byte_array bytes, void* creator) {
  svm_byte_array host_ctx = host_ctx_empty_bytes();

  svm_byte_array app_addr; 
  svm_byte_array init_state; 
  svm_result_t res = svm_spawn_app(&app_addr, &init_state, runtime, creator, host_ctx, bytes);
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

  svm_byte_array_destroy(host_ctx);

  spawned_app_t spawned = {
    .app_addr = app_addr,
    .init_state = init_state
  };

  return spawned;
}


svm_byte_array simulate_get_counter(void* runtime, svm_byte_array app_addr, void* state, void* sender) {
  svm_byte_array func_name = { .bytes = (const uint8_t*)"get", .length = strlen("get") };
  svm_func_buf_t func_buf = { .slice_count = 0, .slices = NULL };
  svm_func_args_t func_args = { .arg_count = 0, .args = NULL };

  svm_byte_array bytes = exec_app_bytes(app_addr, func_name, func_buf, func_args);

  void *app_tx = NULL;
  svm_result_t res = svm_parse_exec_app(&app_tx, runtime, sender, bytes);
  assert(res == SVM_SUCCESS);

  svm_byte_array encoded_receipt;
  svm_byte_array host_ctx = host_ctx_empty_bytes();

  res = svm_exec_app(&encoded_receipt, runtime, app_tx, state, host_ctx);
  assert(res == SVM_SUCCESS);

  svm_byte_array_destroy(bytes);
  svm_byte_array_destroy(host_ctx);

  return encoded_receipt;
}

svm_byte_array simulate_inc_counter(void* runtime, svm_byte_array app_addr, void* state, void* sender, uint32_t inc_by) {
  svm_byte_array func_name = { .bytes = (const uint8_t*)"inc", .length = strlen("inc") };
  svm_func_buf_t func_buf = { .slice_count = 0, .slices = NULL };

  uint8_t arg_bytes[4];
  for (uint8_t i = 0; i < 4; i++) {
    uint8_t off = 24 - i * 8;
    arg_bytes[i] = ((inc_by >> off) & 0xFF);
  }

  svm_func_arg_t arg = {
    .type = (svm_value_type)SVM_I32,
    .bytes = (uint8_t*)&arg_bytes[0]
  };

  svm_func_args_t func_args = { .arg_count = 1, .args = &arg };
  svm_byte_array bytes = exec_app_bytes(app_addr, func_name, func_buf, func_args);

  void *app_tx = NULL;
  svm_result_t res = svm_parse_exec_app(&app_tx, runtime, sender, bytes);
  assert(res == SVM_SUCCESS);

  svm_byte_array encoded_receipt;
  svm_byte_array host_ctx = host_ctx_empty_bytes();

  res = svm_exec_app(&encoded_receipt, runtime, app_tx, state, host_ctx);
  assert(res == SVM_SUCCESS);

  svm_byte_array_destroy(bytes);
  svm_byte_array_destroy(host_ctx);

  return encoded_receipt;
}

void receipt_destroy(svm_receipt_t receipt) {
  if (receipt.success) {
    free(receipt.returns);
    svm_byte_array_destroy(receipt.new_state); 
  }
  else {
    free(receipt.error);
  }
}
  
int main() {
  svm_byte_array bytes;
  svm_byte_array raw_receipts[3];
  svm_receipt_t receipts[3];

  /// 1) Init Runtime
  uint32_t counter_init = 12;
  host_t* host = host_new(counter_init);

  void* imports = imports_build();
  void* runtime = runtime_create(host, imports);

  // 2) Deploy Template
  void* author = alloc_author_addr();
  bytes = deploy_template_bytes(); 
  svm_byte_array template_addr = simulate_deploy_template(runtime, bytes, author);
  svm_byte_array_destroy(bytes);

  // 3) Spawn App 
  void* creator = alloc_creator_addr();
  bytes = spawn_app_bytes(template_addr);
  spawned_app_t spawned = simulate_spawn_app(runtime, bytes, creator);
  svm_byte_array app_addr = spawned.app_addr;
  void* init_state = (void*)spawned.init_state.bytes;
  svm_byte_array_destroy(bytes);

  // 4) Exec App
  //// a) Query for the initialized counter value
  void* sender = alloc_sender_addr();
  raw_receipts[0] = simulate_get_counter(runtime, app_addr, init_state, sender);
  receipts[0] = decode_receipt(raw_receipts[0]);
  print_receipt(receipts[0]);

  //// b) Increment the counter 
  printf("\n");

  void* new_state = (void*)receipts[0].new_state.bytes;
  uint32_t inc_by = 7;
  raw_receipts[1] = simulate_inc_counter(runtime, app_addr, new_state, sender, inc_by); 
  receipts[1] = decode_receipt(raw_receipts[1]);
  print_receipt(receipts[1]);

  //// c) Query for the new counter value
  raw_receipts[2] = simulate_get_counter(runtime, app_addr, init_state, sender);
  receipts[2] = decode_receipt(raw_receipts[2]);
  print_receipt(receipts[2]);

  // 5) Reclaiming resources
  free(author);
  free(creator);
  free(sender);

  for (uint8_t i = 0; i < 3; i++) {
    receipt_destroy(receipts[i]);
    svm_byte_array_destroy(raw_receipts[i]); 
  }

  svm_byte_array_destroy(template_addr);
  svm_byte_array_destroy(spawned.app_addr);
  svm_byte_array_destroy(spawned.init_state);

  svm_runtime_destroy(runtime);
  svm_imports_destroy(imports);
}
