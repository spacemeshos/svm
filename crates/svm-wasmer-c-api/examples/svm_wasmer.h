#ifndef WASMER_SVM_H
#define WASMER_SVM_H

#include "wasmer.h"
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {

} wasmer_import_object_t;

/**
 * Returns a pointer to the `svm context node_data`.
 * It will be used by the node vmcalls implementation.
**/
void *wasmer_svm_instance_context_node_data_get(const wasmer_instance_context_t *ctx);

/**
 * Creates a new Import object
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t wasmer_svm_import_object(wasmer_import_object_t** import_object,
                                         void *addr_ptr,
                                         void *node_data,
                                         wasmer_import_t *imports,
                                         uint32_t imports_len);


/**
 * Given a compiler wasmer module (param `module`) and a ready-made import object (param `import_object`),
 * instantiates a new wasmer instance.
 * The instance is returned via the param `instance` (that's why it's of type `wasmer_instance_t**`)
 *
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
**/
wasmer_result_t wasmer_svm_module_instantiate(wasmer_instance_t** instance,
                                              wasmer_module_t* module,
                                              wasmer_import_object_t* import_object);

#endif /* WASMER_SVM_H */
