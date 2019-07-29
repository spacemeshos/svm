#include "wasmer.h"
#include "svm_wasmer.h"
#include <stdlib.h>

typedef struct {
  uint32_t counter;
} node_data_t;

node_data_t* new_node_data(uint32_t counter) {
   node_data_t* ptr = (node_data_t*)malloc(sizeof(node_data_t));
   ptr->counter = counter;
   return ptr;
}

void inc_counter(wasmer_instance_context_t *ctx, uint32_t amount) {
  node_data_t *nd = (node_data_t*)(wasmer_svm_instance_context_node_data_get(ctx));
  nd->counter = nd->counter + amount;
}

uint32_t get_counter(wasmer_instance_context_t *ctx) {
  node_data_t *nd = (node_data_t*)(wasmer_svm_instance_context_node_data_get(ctx));
  return nd->counter;
  return 0;
}

int main() {
  uint32_t addr = 0x11223344;
  void* addr_ptr = (void*)(&addr);
  void* node_data = (void*)(new_node_data(5));
  wasmer_import_t* imports = NULL;
  uint32_t imports_len = 0;

  wasmer_validate(NULL, 0);

  wasmer_svm_import_object(addr_ptr, node_data, imports, imports_len);

  return 0;
}
