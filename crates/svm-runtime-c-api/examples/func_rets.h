#ifndef SVM_FUNC_RETS
#define SVM_FUNC_RETS

#include <stdint.h>

typedef struct {
  uint8_t type;
  uint32_t i32_value;
  uint64_t i64_value;
} svm_func_ret_t;


#endif
