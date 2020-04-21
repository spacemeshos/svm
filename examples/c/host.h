#ifndef HOST_H
#define HOST_H

#include "../svm.h"

typedef struct {
  uint32_t counter;
} host_t;

host_t* host_new(uint32_t counter_initial) {
  host_t* host = (host_t*)malloc(sizeof(host_t));
  host->counter = counter_initial;

  return host;
}

void host_inc_func(void *ctx, uint32_t value) {
  host_t *host = (host_t*)(svm_instance_context_host_get(ctx));
  host->counter = host->counter + value;
}

uint32_t host_get_func(void *ctx) {
  host_t *host = (host_t*)(svm_instance_context_host_get(ctx));
  return host->counter;
}

#endif /* HOST_H */
