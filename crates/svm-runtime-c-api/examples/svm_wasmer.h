#ifndef WASMER_SVM_H
#define WASMER_SVM_H

#include "wasmer.h"
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {

} svm_contract_t;


typedef struct {

} svm_transaction_t;


typedef struct {

} svm_receipt_t;

/**
 * Creates an in-memory Contract from given bytes
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t svm_contract_build(svm_contract_t **contract, void *contract_bytes, uint64_t contract_bytes_len);

/**
 * Computes contract account address and returns a pointer to the underlying array
 */
void* svm_contract_compute_address(svm_contract_t *contract);

/**
 * Store contract a persistent storage.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t svm_contract_store(svm_contract_t *contract, void *address);


/**
 * Creates a Transaction from given bytes.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t svm_transaction_build(svm_transaction_t **tx, void *tx_bytes, uint64_t tx_bytes_len);

/**
 * Executes input transaction and produces a receipt.
 *
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t svm_transaction_exec(svm_receipt_t **receipt, svm_transaction_t *tx, wasmer_import_object_t *import_object);

/**
 * Returns a pointer to register internal bytes array
 */
void *svm_register_get(const wasmer_instance_context_t *ctx, int32_t reg_bits, int32_t reg_idx);


/**
  Copies `bytes_len` bytes from raw pointer `bytes` into `wasmer svm` register indexed `reg_idx`.
 */
void svm_register_set(const wasmer_instance_context_t *ctx,
                      int32_t reg_bits,
                      int32_t reg_idx,
                      void *bytes,
                      uint8_t bytes_len);

/**
 * Returns a pointer to the `svm context node_data`.
 * It will be used by the node vmcalls implementation.
 */
void *svm_instance_context_node_data_get(const wasmer_instance_context_t *ctx);


/**
 * Creates a new Import object
 * Returns `wasmer_result_t::WASMER_OK` upon success.
 * Returns `wasmer_result_t::WASMER_ERROR` upon failure. Use `wasmer_last_error_length`
 * and `wasmer_last_error_message` to get an error message.
 */
wasmer_result_t svm_import_object(wasmer_import_object_t** import_object,
                                  void *addr,
                                  void *state,
                                  uint32_t max_pages,
                                  uint32_t max_page_slices,
                                  void *node_data,
                                  wasmer_import_t *imports,
                                  uint32_t imports_len);

/**
 * Returns `true` if transaction succedded and `false` otherwise *
 */
bool svm_receipt_result(svm_receipt_t *receipt);

/**
 * If transaction failed, usus `wasmer_last_error_message` to get an error message.
 */
void svm_receipt_error(svm_receipt_t *receipt);

/** Should be called only if transaction succedded. Returns a pointer to new state
 * Panics when called for a failed transaction
 */
void* svm_receipt_new_state(svm_receipt_t *receipt);

#endif /* WASMER_SVM_H */
