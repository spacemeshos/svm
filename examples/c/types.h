#ifndef TYPES_H
#define TYPES_H

#include "../svm.h"

typedef struct {
  svm_result_t status;
  svm_byte_array receipt;
  svm_byte_array template_addr;
  uint64_t gas_used;
} deploy_template_result_t;

typedef struct {
  svm_result_t status;
  svm_byte_array receipt;
  svm_byte_array app_addr;
  svm_byte_array init_state;
  uint64_t gas_used;
} spawn_app_result_t;

typedef struct {  
  svm_result_t status;
  svm_byte_array receipt;
  svm_byte_array new_state;
  svm_value_array returns;
  uint64_t gas_used;
} exec_app_result_t;

#endif /* TYPES_H */
