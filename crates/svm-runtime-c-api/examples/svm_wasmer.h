#include "wasmer.h"

#ifndef WASMER_SVM_H
#define WASMER_SVM_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * `*const svm_contract_t` is a raw pointer to a Rust `svm_contract::wasm::Contract` struct.
 */
typedef struct {

} svm_contract_t;

/**
 * `*const svm_receipt_t` is a raw pointer to a Rust `Receipt` struct.
 */
typedef struct {

} svm_receipt_t;

/**
 * `*const svm_transaction_t` is a raw pointer to a Rust `svm_contract::Transaction` struct.
 */
typedef struct {

} svm_transaction_t;

/**
 * Builds an instance of `svm_contract_t`.
 * Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
 */
wasmer_result_t svm_contract_build(svm_contract_t **raw_contract,
                                   const void *raw_bytes,
                                   uint64_t raw_bytes_len);

/**
 * Computes the contract to-be-deployed acccunt address and retures a pointer to it
 */
const void *svm_contract_compute_address(const svm_contract_t *raw_contract);

/**
 * Stores the new deployed contract under a database.
 * Future transaction will reference the contract by it's account address.
 * (see `svm_transaction_exec`)
 *
 * This function should be called after performing validation.
 *
 * * `raw_contract` - The wasm contract to be stored
 *
 */
wasmer_result_t svm_contract_store(const svm_contract_t *raw_contract, const void *raw_addr);

/**
 * Creates a new `wasmer` import object.
 * The import object will include imports of two flavors:
 * * external vmcalls (i.e: node vmcalls)
 * * internal vmcalls (i.e: register/storage/etc vmcalls)
 */
wasmer_result_t svm_import_object(wasmer_import_object_t **raw_import_object,
                                  const void *raw_addr,
                                  const void *raw_state,
                                  int raw_max_pages,
                                  int raw_max_page_slices,
                                  const void *node_data,
                                  wasmer_import_t *imports,
                                  unsigned int imports_len);

/**
 * Gets the `node_data` field within the `svm context` (a.k.a `data` of the wasmer context).
 */
const void *svm_instance_context_node_data_get(const wasmer_instance_context_t *raw_ctx);

/**
 * Returns the `receipt` error in transaction failed
 */
void svm_receipt_error(const svm_receipt_t *raw_receipt);

/**
 * Returns a pointer to the new state of the contract account.
 */
const uint8_t *svm_receipt_new_state(const svm_receipt_t *raw_receipt);

/**
 * Returns the transaction execution results (wasm array).
 * Should be called only after verifying that the transaction succeeded.
 * Will panic when called for a failed transaction.
 */
void svm_receipt_results(const svm_receipt_t *raw_receipt,
                         wasmer_value_t **results,
                         uint32_t *results_len);

/**
 * Returns the receipt outcome (`true` for success and `false` otherwise)
 */
bool svm_receipt_status(const svm_receipt_t *raw_receipt);

/**
 * Returns a raw pointer to the `wasmer svm` register's internal content
 */
const void *svm_register_get(const wasmer_instance_context_t *raw_ctx,
                             int32_t reg_bits,
                             int32_t reg_idx);

/**
 * Copies `bytes_len` bytes from raw pointer `bytes` into `wasmer svm` register indexed `reg_idx`.
 */
void svm_register_set(const wasmer_instance_context_t *raw_ctx,
                      int32_t reg_bits,
                      int32_t reg_idx,
                      const void *bytes,
                      uint8_t bytes_len);

/**
 * Builds an instance of `svm_transaction_t`.
 * Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
 */
wasmer_result_t svm_transaction_build(svm_transaction_t **raw_tx,
                                      const void *raw_bytes,
                                      uint64_t raw_bytes_len);

/**
 * Triggers a transaction execution of an already deployed contract.
 *
 * `receipt` - The receipt of the contract execution.
 * `tx`      - The transaction to execute.
 */
wasmer_result_t svm_transaction_exec(svm_receipt_t **raw_receipt,
                                     const svm_transaction_t *raw_tx,
                                     const wasmer_import_object_t *raw_import_object);

#endif /* WASMER_SVM_H */
