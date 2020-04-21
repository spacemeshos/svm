#ifndef HOST_CTX_H
#define HOST_CTX_H

#include <stdio.h>

#include "../svm.h"
#include "types.h"

void print_chars(svm_byte_array arr) {
  for (int i = 0; i < arr.length; i++) {
    printf("%c", arr.bytes[i]);
  }
}

void print_status(svm_result_t res) {
   switch (res) {
      case SVM_SUCCESS: 
        printf("SVM_SUCCESS");
        break;
      case SVM_FAILURE: 
        printf("SVM_FAILURE");
        break;
   }
}

void print_hex(svm_byte_array arr) {
  for (int i = 0; i < arr.length; i++) {
    printf("%02x", arr.bytes[i]);
  }
}

void print_values(svm_value_array arr) {
  for (int i = 0; i < arr.length; i++) {
    switch (arr.values[i].ty) {
      case SVM_I32:
            printf("%d ", arr.values[i].i32_val);
      break;
      case SVM_I64:
            printf("%d ", arr.values[i].i64_val);
      break;
    }
  }
}

void print_deploy_template_result(deploy_template_result_t res) {
  printf("Deploy template result:");
  printf("\n");

  printf("  Status: ");
  print_status(res.status);
  printf("\n");

  printf("  Receipt: ");
  print_hex(res.receipt);
  printf("\n");
        
  printf("  Template address: ");
  print_hex(res.template_addr);
  printf("\n");

  printf("  Gas used: %d", res.gas_used);
  printf("\n");
}

void print_spawn_app_result(spawn_app_result_t res) {
  printf("Spawn App result:");
  printf("\n");

  printf("  Status: ");
  print_status(res.status);
  printf("\n");

  printf("  Receipt: ");
  print_hex(res.receipt);
  printf("\n");

  printf("  App address: ");
  print_hex(res.app_addr);
  printf("\n");

  printf("  Initial state: ");
  print_hex(res.init_state);
  printf("\n");

  printf("  Gas used: %d", res.gas_used);
  printf("\n");
}

void print_exec_app_result(exec_app_result_t res, char *func_name) {
  printf("Exec App [%s] result:", func_name);
  printf("\n");

  printf("  Status: ");
  print_status(res.status);
  printf("\n");

  printf("  Receipt: ");
  print_hex(res.receipt);
  printf("\n");

  printf("  New state: ");
  print_hex(res.new_state);
  printf("\n");

  if (res.returns.length > 0) {
    printf("  Returns: ");
    print_values(res.returns);
    printf("\n");
  }

  printf("  Gas used: %d", res.gas_used);
  printf("\n");
}

#endif /* HOST_CTX_H */
