#include "wasmer.h"

#ifndef WASMER_SVM_H
#define WASMER_SVM_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Builds an instance of `svm_contract_t`.
 * Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
 */
wasmer_result_t svm_contract_build(void *raw_runtime,
                                   void **raw_contract,
                                   const void *raw_bytes,
                                   uint64_t raw_bytes_len);

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
wasmer_result_t svm_contract_deploy(void *raw_runtime,
                                    const void *raw_contract,
                                    const void *raw_addr);

/**
 * Derives the contract to-be-deployed acccunt address and retures a pointer to it
 */
const void *svm_contract_derive_address(const void *raw_runtime, const void *raw_contract);

/**
 * Returns the `receipt` error in transaction failed
 */
void svm_receipt_error(const void *raw_receipt);

/**
 * Returns a pointer to the new state of the contract account.
 */
const uint8_t *svm_receipt_new_state(const void *raw_receipt);

/**
 * Returns the transaction execution results (wasm array).
 * Should be called only after verifying that the transaction succeeded.
 * Will panic when called for a failed transaction.
 */
void svm_receipt_results(const void *raw_receipt, wasmer_value_t **results, uint32_t *results_len);

wasmer_result_t svm_runtime_create(void **raw_runtime);

wasmer_result_t svm_runtime_destroy(void *raw_runtime);

/**
 * Builds an instance of `svm_transaction_t`.
 * Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
 */
wasmer_result_t svm_transaction_build(const void *raw_runtime,
                                      void **raw_tx,
                                      const void *raw_bytes,
                                      uint64_t raw_bytes_len);

/**
 * Triggers a transaction execution of an already deployed contract.
 *
 * `receipt` - The receipt of the contract execution.
 * `tx`      - The transaction to execute.
 */
wasmer_result_t svm_transaction_exec(void *raw_runtime,
                                     void **raw_receipt,
                                     const void *raw_tx,
                                     const void *raw_import_object);

#endif /* WASMER_SVM_H */
