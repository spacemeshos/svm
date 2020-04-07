#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

#include "../svm.h"
#include "constants.h"

#include "read_file.h"
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
  svm_byte_array template_addr;
} deploy_template_t;

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

  svm_byte_array err;
  svm_result_t res = svm_import_func_build(imports, module_name, import_name, func, params, returns, &err);
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
  svm_byte_array err;
  svm_result_t res = svm_import_func_build(imports, module_name, import_name, func, params, returns, &err);
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

  svm_byte_array err;
  svm_result_t res = svm_memory_runtime_create(&runtime, kv, host, imports, &err); 
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

void print_err(svm_byte_array err) {
  for (int i = 0; i < err.length; i++) {
    printf("%c", err.bytes[i]);
  }
  printf("\n\n");
}

void print_addr(svm_byte_array addr) {
  printf("Address:\n");
  for (int i = 0; i < addr.length; i++) {
    printf("%x", addr.bytes[i]);
  }
  printf("\n");
}
  
void print_state(svm_byte_array state) {
  printf("Address:\n");
  for (int i = 0; i < state.length; i++) {
    printf("%x", state.bytes[i]);
  }
  printf("\n");
}

deploy_template_t simulate_deploy_template(void* runtime, svm_byte_array bytes, svm_byte_array author, bool dry_run) {
    svm_byte_array host_ctx = host_ctx_empty_bytes();
    svm_byte_array receipt;
    svm_byte_array err;

    svm_result_t res = svm_deploy_template(&receipt, runtime, bytes, author, host_ctx, dry_run, &err);

    if (res == SVM_FAILURE) {
      printf("`svm_deploy_template` failure:\n");
      print_err(err);
      exit(1);
    }

    svm_byte_array template_addr;
    svm_template_receipt_addr(&template_addr, receipt);

    printf("Deployed AppTemplate successfully:\n");
    print_addr(template_addr);

    free((void*)host_ctx.bytes);
    svm_byte_array_destroy(receipt);
    svm_byte_array_destroy(err);

    return (deploy_template_t) { .template_addr = template_addr };
}

spawned_app_t simulate_spawn_app(void* runtime, svm_byte_array bytes, svm_byte_array creator) {
  svm_result_t res;
  svm_byte_array err; 

  res = svm_validate_app(runtime, bytes, &err);
  if (res == SVM_FAILURE) {
    printf("`svm_validate_app` failure:\n");
    print_err(err);
    exit(1);
  }

  svm_byte_array host_ctx = host_ctx_empty_bytes();
  svm_byte_array receipt; 

  res = svm_spawn_app(&receipt, runtime, bytes, creator, host_ctx, 1, &err);

  if (res == SVM_FAILURE) {
     printf("`svm_spawn_app` failure:\n");
     print_err(err);
     exit(1);
  }

  svm_byte_array app_addr, init_state;
  svm_app_receipt_state(&init_state, receipt);
  svm_app_receipt_addr(&app_addr, receipt);

  printf("Spawned App successfully\n");
  print_addr(app_addr);
  print_state(init_state);

  svm_byte_array_destroy(host_ctx);

  spawned_app_t spawned = {
    .app_addr = app_addr,
    .init_state = init_state
  };

  return spawned;
}


svm_byte_array simulate_get_counter(void* runtime, svm_byte_array app_addr, svm_byte_array state, void* sender) {
  svm_byte_array func_name = { .bytes = (const uint8_t*)"get", .length = strlen("get") };
  svm_func_buf_t func_buf = { .slice_count = 0, .slices = NULL };
  svm_func_args_t func_args = { .arg_count = 0, .args = NULL };

  svm_byte_array bytes = exec_app_bytes(app_addr, func_name, func_buf, func_args);

  svm_byte_array err;
  svm_result_t res = svm_validate_tx(&app_addr, runtime, bytes, &err);
  assert(res == SVM_SUCCESS);

  svm_byte_array encoded_receipt;
  svm_byte_array host_ctx = host_ctx_empty_bytes();

  res = svm_exec_app(&encoded_receipt, runtime, bytes, state, host_ctx, 1, &err);
  assert(res == SVM_SUCCESS);

  svm_byte_array_destroy(bytes);
  svm_byte_array_destroy(host_ctx);

  return encoded_receipt;
}

svm_byte_array simulate_inc_counter(void* runtime, svm_byte_array app_addr, svm_byte_array state, void* sender, uint32_t inc_by) {
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

  svm_byte_array app_tx;
  svm_byte_array err;
  svm_result_t res = svm_validate_tx(&app_tx, runtime, bytes, &err);
  assert(res == SVM_SUCCESS);

  svm_byte_array receipt;
  svm_byte_array host_ctx = host_ctx_empty_bytes();
  res = svm_exec_app(&receipt, runtime, app_tx, state, host_ctx, 1, &err);
  assert(res == SVM_SUCCESS);

  svm_byte_array_destroy(bytes);
  svm_byte_array_destroy(host_ctx);

  return receipt;
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

svm_byte_array alloc_str_address(char* str) {
  uint8_t* buf = (uint8_t*)malloc(SVM_ADDR_LEN);
  memset(buf, 0, SVM_ADDR_LEN);

  int n = SVM_ADDR_LEN < strlen(str) ? SVM_ADDR_LEN : strlen(str);
  memcpy(buf, str, n);

  return (svm_byte_array) {
    .bytes = buf, 
    .length = SVM_ADDR_LEN,
  };
}
  
int main() {
  /* svm_byte_array raw_receipts[3]; */
  /* svm_receipt_t receipts[3]; */

  bool dry_run = false;
  svm_byte_array data;

  // 1) Init Runtime
  uint32_t counter_init = 12;
  host_t* host = host_new(counter_init);

  void* imports = imports_build();
  void* runtime = runtime_create(host, imports);

  // 2) Deploy Template
  svm_byte_array author = alloc_str_address("author");
  data = read_file("raw/app_template.bin");

  deploy_template_t deploy_template_res = simulate_deploy_template(runtime, data, author, dry_run);

  free((void*)data.bytes);

   // 3) Spawn App
  svm_byte_array creator = alloc_str_address("creator");
  data = read_file("raw/spawn_app.bin");

  spawned_app_t _spawned = simulate_spawn_app(runtime, data, creator);
  free((void*)data.bytes);


//  // 4) Exec App
//  //// a) Query for the initialized counter value
//  void* sender = alloc_sender_addr();
//  raw_receipts[0] = simulate_get_counter(runtime, app_addr, init_state, sender);
//  receipts[0] = decode_receipt(raw_receipts[0]);
//  print_receipt(receipts[0]);
//
//  //// b) Increment the counter
//  printf("\n");
//
//  svm_byte_array new_state = receipts[0].new_state;
//  uint32_t inc_by = 7;
//  raw_receipts[1] = simulate_inc_counter(runtime, app_addr, new_state, sender, inc_by);
//  receipts[1] = decode_receipt(raw_receipts[1]);
//  print_receipt(receipts[1]);
//
//  //// c) Query for the new counter value
//  raw_receipts[2] = simulate_get_counter(runtime, app_addr, init_state, sender);
//  receipts[2] = decode_receipt(raw_receipts[2]);
//  print_receipt(receipts[2]);
//
//  // 5) Reclaiming resources
//  for (uint8_t i = 0; i < 3; i++) {
//    receipt_destroy(receipts[i]);
//    svm_byte_array_destroy(raw_receipts[i]);
//  }
//
//  svm_byte_array_destroy(author);
//  svm_byte_array_destroy(creator);
//  svm_byte_array_destroy(template_addr);
//  svm_byte_array_destroy(spawned.app_addr);
//  svm_byte_array_destroy(spawned.init_state);
//
//  svm_runtime_destroy(runtime);
//  svm_imports_destroy(imports);
//
//  free(sender);
}
