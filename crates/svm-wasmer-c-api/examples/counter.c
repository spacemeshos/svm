#include "svm_wasmer.h"

typedef struct {
  uint32_t counter;
} node_data_t;

void inc_counter(wasmer_instance_context_t *ctx, uint32_t amount) {
  node_data_t *node_data = (node_data_t*)(wasmer_svm_instance_context_node_data_get(ctx));
  node_data->counter = node_data->counter + 1;
}

/* uint32_t get_counter(wasmer_instance_context_t *ctx) { */
/*   node_data_t *node_data = (node_data_t*)(wasmer_svm_instance_context_node_data_get(ctx)); */
/*   return node_data->counter; */
/* } */


int main() {

  return 0;
}
