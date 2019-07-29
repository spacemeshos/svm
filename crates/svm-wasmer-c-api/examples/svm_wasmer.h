// #ifndef WASMER_SVM_H
// #define WASMER_SVM_H

#include "wasmer.h"
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Return pointer to the  `svm context node_data`.
 * It will be used by the node vmcalls implementation.
**/
void *wasmer_svm_instance_context_node_data_get(const wasmer_instance_context_t *ctx);

/**
 * Creates a new Import object
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_svm_import_object(void *addr_ptr,
                                         void *node_data,
                                         wasmer_import_t *imports,
                                         uint32_t imports_len);

// #endif /* WASMER_SVM_H */
