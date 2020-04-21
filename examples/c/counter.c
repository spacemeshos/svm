#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

#include "../svm.h"
#include "constants.h"
#include "types.h"
#include "helpers.h"
#include "printers.h"
#include "read_file.h"
#include "host.h"
#include "host_ctx.h"

void assert_svm_result(svm_result_t res, svm_byte_array err, char *func_name ) {
    if (res == SVM_FAILURE) {
      printf("`%s` failure: %s\n", func_name, err);
      exit(1);
    }
}

void inc_func_import_build(void* imports) { 
  svm_byte_array module_name = from_str("env");
  svm_byte_array import_name = from_str("inc");
  void* func = (void*)host_inc_func;
  svm_value_type_array returns = { .types = NULL, .length = 0 };
  svm_byte_array err;

  svm_value_type* types = (svm_value_type*)malloc(sizeof(svm_value_type) * 1);
  types[0] = SVM_I32;
  svm_value_type_array params = { .types = types, .length = 1 };

  svm_result_t res = svm_import_func_build(imports, module_name, import_name, func, params, returns, &err);
  assert_svm_result(res, err, "svm_import_func_build");

  free(params.types);
}

void get_func_import_build(void* imports) {
  svm_byte_array module_name = from_str("env");
  svm_byte_array import_name = from_str("get");
  svm_value_type_array params = { .types = NULL, .length = 0 };
  void* func = (void*)host_get_func;
  svm_byte_array err;

  svm_value_type* types = (svm_value_type*)malloc(sizeof(svm_value_type) * 1);
  types[0] = SVM_I32;
  svm_value_type_array returns = { .types = types, .length = 1 };

  svm_result_t res = svm_import_func_build(imports, module_name, import_name, func, params, returns, &err);
  assert_svm_result(res, err, "svm_import_func_build");

  free(returns.types);
}

void* imports_build() {
  uint32_t length = 2;
  void* imports = NULL;

  svm_result_t res = svm_imports_alloc(&imports, length);
  assert(res == SVM_SUCCESS);

  inc_func_import_build(imports);
  get_func_import_build(imports);

  return imports;
}

void* runtime_create(host_t* host, void* imports) {
  // Create a key-value store.
  void *kv = NULL;
  svm_memory_kv_create(&kv);

  void *runtime = NULL;
  svm_byte_array err;
  svm_result_t res = svm_memory_runtime_create(&runtime, kv, host, imports, &err); 
  assert_svm_result(res, err, "svm_memory_runtime_create");

  return runtime;
}

deploy_template_result_t simulate_deploy_template(void* runtime, svm_byte_array code) {
    svm_byte_array host_ctx = host_ctx_empty_bytes();
    svm_byte_array receipt = empty_byte_array();
    svm_byte_array err = empty_byte_array();
    svm_result_t res;

    // Encode.
    svm_byte_array app_template;
    uint32_t version = 0;
    svm_byte_array name = from_str("name");
    uint16_t page_count = 1;
    res = svm_encode_app_template(&app_template, 0, name, page_count, code, &err);
    assert_svm_result(res, err, "svm_encode_app_template");

    // Execute.
    svm_byte_array author = alloc_str_address("author");
    res = svm_deploy_template(&receipt, runtime, app_template, author, host_ctx, GAS_METERING, GAS_LIMIT, &err);
    assert_svm_result(res, err, "svm_deploy_template");

    // Extract template address.
    svm_byte_array template_addr;
    res = svm_template_receipt_addr(&template_addr, receipt, &err);
    assert_svm_result(res, err, "svm_template_receipt_addr");

    // Extract gas used.
    uint64_t gas_used;
    res = svm_template_receipt_gas(&gas_used, receipt, &err);
    assert_svm_result(res, err, "svm_template_receipt_gas");

    // Reclaim resources.
    free(host_ctx.bytes);
    free(author.bytes);

    return (deploy_template_result_t) { 
      .status = SVM_SUCCESS,
      .receipt = receipt,
      .template_addr = template_addr, 
      .gas_used = gas_used
    };
}

spawn_app_result_t simulate_spawn_app(void* runtime, svm_byte_array template_addr) {
  svm_result_t res;
  svm_byte_array err; 

  // Encode.
  svm_byte_array spawn_app;
  uint32_t version = 0;
  uint16_t ctor_idx = 0;
  svm_byte_array ctor_buf;

  svm_value* values  = (svm_value*)malloc(sizeof(svm_value) * 1);
  values[0] = (svm_value) { .ty = SVM_I32, .i32_val = 100 };  
  svm_value_array ctor_args = { .values = values, .length = 1 };

  res = svm_encode_spawn_app(&spawn_app, version, template_addr, ctor_idx, ctor_buf, ctor_args, &err);
  assert_svm_result(res, err, "svm_encode_spawn_app");

  // Validate.
  res = svm_validate_app(runtime, spawn_app, &err);
  assert_svm_result(res, err, "svm_validate_app");

  // Execute.
  svm_byte_array host_ctx = host_ctx_empty_bytes();
  svm_byte_array creator = alloc_str_address("creator");
  svm_byte_array receipt; 
  res = svm_spawn_app(&receipt, runtime, spawn_app, creator, host_ctx, GAS_METERING, GAS_LIMIT, &err);
  assert_svm_result(res, err, "svm_spawn_app");
  assert(res == svm_app_receipt_status(receipt, &err));

  // Extract: initial state.
  svm_byte_array init_state;
  res = svm_app_receipt_state(&init_state, receipt, &err);
  assert_svm_result(res, err, "svm_app_receipt_state");

  // Extract: app address.
  svm_byte_array app_addr;
  res = svm_app_receipt_addr(&app_addr, receipt, &err);
  assert_svm_result(res, err, "svm_app_receipt_addr");

  // Extract: gas used.
  uint64_t gas_used;
  res = svm_app_receipt_gas(&gas_used, receipt, &err);
  assert_svm_result(res, err, "svm_app_receipt_gas");

  // Reclaim resources.
  free(host_ctx.bytes);
  free(creator.bytes);
  free(ctor_args.values);

  return (spawn_app_result_t) {
    .receipt = receipt,
    .app_addr = app_addr,
    .init_state = init_state,
    .gas_used = gas_used
  };
}

exec_app_result_t simulate_exec_app(void* runtime, svm_byte_array app_addr, svm_byte_array app_state, uint32_t func_idx, svm_value_array func_args) {
  svm_result_t res;
  svm_byte_array err;

  // Encode.
  svm_byte_array app_tx;
  uint32_t version = 0;
  svm_byte_array func_buf = empty_byte_array();
  res = svm_encode_app_tx(&app_tx, version, app_addr, func_idx, func_buf, func_args, &err);
  assert_svm_result(res, err, "svm_encode_app_tx");

  // Validate.
  res = svm_validate_tx(&app_addr, runtime, app_tx, &err);
  assert_svm_result(res, err, "svm_validate_tx");

  // Execute.
  svm_byte_array host_ctx = host_ctx_empty_bytes();
  svm_byte_array receipt;
  res = svm_exec_app(&receipt, runtime, app_tx, app_state, host_ctx, GAS_METERING, GAS_LIMIT, &err);
  assert_svm_result(res, err, "svm_exec_app");
  assert(res == svm_exec_receipt_status(receipt, &err));

  // Extract: new state.
  svm_byte_array new_state;
  res = svm_exec_receipt_state(&new_state, receipt, &err);
  assert_svm_result(res, err, "svm_exec_receipt_state");

  // Extract: returns.
  svm_value_array returns;
  res = svm_exec_receipt_returns(&returns, receipt, &err);
  assert_svm_result(res, err, "svm_exec_receipt_returns");

  // Extract: gas used.
  uint64_t gas_used;
  res = svm_exec_receipt_gas(&gas_used, receipt, &err);
  assert_svm_result(res, err, "svm_app_receipt_gas");

  // Reclaim resources.
  free(host_ctx.bytes);

  return (exec_app_result_t) {
    .status = SVM_SUCCESS,
    .receipt = receipt,
    .new_state = new_state,
    .returns = returns,
    .gas_used = gas_used,
  };
}

exec_app_result_t simulate_storage_inc(void* runtime, svm_byte_array app_addr, svm_byte_array app_state, uint32_t inc_by) {
  uint32_t func_idx = 0;

  svm_value* values  = (svm_value*)malloc(sizeof(svm_value) * 1);
  values[0] = (svm_value) { .ty = SVM_I32, .i32_val = inc_by };  
  svm_value_array func_args = { .values = values, .length = 1 };

  exec_app_result_t res = simulate_exec_app(runtime, app_addr, app_state, func_idx, func_args);

  free(func_args.values);

  return res;
}

exec_app_result_t simulate_storage_get(void* runtime, svm_byte_array app_addr, svm_byte_array state) {
  uint32_t func_idx = 1;
  svm_value_array func_args = { .values = NULL, .length = 0 };

  return simulate_exec_app(runtime, app_addr, state, func_idx, func_args);
}

exec_app_result_t simulate_host_inc(void* runtime, svm_byte_array app_addr, svm_byte_array state, uint32_t inc_by) {
  uint32_t func_idx = 2;

  svm_value* values  = (svm_value*)malloc(sizeof(svm_value) * 1);
  values[0] = (svm_value) { .ty = SVM_I32, .i32_val = inc_by };  
  svm_value_array func_args = { .values = values, .length = 1 };

  exec_app_result_t res = simulate_exec_app(runtime, app_addr, state, func_idx, func_args);

  free(func_args.values);

  return res;
}

exec_app_result_t simulate_host_get(void* runtime, svm_byte_array app_addr, svm_byte_array state) {
  uint32_t func_idx = 3;
  svm_value_array func_args = { .values = NULL, .length = 0 };

  return simulate_exec_app(runtime, app_addr, state, func_idx, func_args);
}

int main() {
  // 1) Initialize runtime.
  uint32_t counter_init = 0;
  host_t* host = host_new(counter_init);
  void* imports = imports_build();
  void* runtime = runtime_create(host, imports);

  // 2) Deploy Template.
  svm_byte_array code = read_file("wasm/counter.wasm");
  deploy_template_result_t deploy_template_res = simulate_deploy_template(runtime, code);
  printf("\n");
  print_deploy_template_result(deploy_template_res);
  printf("\n");

  // 3) Spawn App.
  spawn_app_result_t spawn_app_res = simulate_spawn_app(runtime, deploy_template_res.template_addr);
  print_spawn_app_result(spawn_app_res);
  printf("\n");

  // 4)  Exec App
  exec_app_result_t exec_app_res[4];

  //    a) Call: increment via storage.
  uint32_t inc_by = 10;
  exec_app_res[0] = simulate_storage_inc(runtime, spawn_app_res.app_addr, spawn_app_res.init_state, inc_by);
  print_exec_app_result(exec_app_res[0], "storage_inc");
  printf("\n");

  //    b) Call: get via storage.
  exec_app_res[1] = simulate_storage_get(runtime, spawn_app_res.app_addr, exec_app_res[0].new_state);
  print_exec_app_result(exec_app_res[1], "storage_get");
  printf("\n");

  //    c) Call: increment via host.
  inc_by = 25;
  exec_app_res[2] = simulate_host_inc(runtime, spawn_app_res.app_addr, exec_app_res[1].new_state, inc_by);
  print_exec_app_result(exec_app_res[2], "host_inc");
  printf("\n");

  //    d) Call: get via host.
  exec_app_res[3] = simulate_host_get(runtime, spawn_app_res.app_addr, exec_app_res[2].new_state);
  print_exec_app_result(exec_app_res[3], "host_get");
  printf("\n");

  // 5) Reclaim resources.
  svm_runtime_destroy(runtime);
  svm_imports_destroy(imports);

  free(code.bytes);

  svm_byte_array_destroy(deploy_template_res.receipt);
  svm_byte_array_destroy(deploy_template_res.template_addr);

  svm_byte_array_destroy(spawn_app_res.receipt);
  svm_byte_array_destroy(spawn_app_res.app_addr);
  svm_byte_array_destroy(spawn_app_res.init_state);

  for (uint8_t i = 0; i < 4; i++) {
    svm_byte_array_destroy(exec_app_res[i].receipt); 
    svm_byte_array_destroy(exec_app_res[i].new_state); 
    svm_value_array_destroy(exec_app_res[i].returns); 
  }
}
