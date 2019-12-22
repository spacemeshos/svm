use svm_common::{Address, State};
use svm_contract::{transaction::Transaction, wasm::Contract};
use svm_runtime::{
    contract_settings::ContractSettings, register::SvmReg, traits::Runtime, Receipt,
};

use crate::helpers;
use crate::RuntimePtr;

use log::{debug, error, trace};
use std::ffi::c_void;

use wasmer_runtime::{Ctx, ImportObject};

use wasmer_runtime_c_api::{
    error::update_last_error,
    import::{wasmer_import_object_extend, wasmer_import_object_t, wasmer_import_t},
    value::wasmer_value_t,
    wasmer_result_t,
};

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_create(
    raw_runtime: *mut *mut c_void,
    host: *const c_void,
    host_funcs: *mut c_void,
    host_funcs_len: libc::c_uint,
) -> wasmer_result_t {
    debug!("`svm_runtime_create`");

    // let imports: *mut wasmer_import_t = host_funcs as _;
    let exts = Vec::new();
    let runtime = svm_runtime::create_rocksdb_runtime(host, "tests-contract-code", exts);

    let runtime: Box<dyn Runtime> = Box::new(runtime);

    let runtime_ptr: RuntimePtr = RuntimePtr::new(runtime);
    *raw_runtime = helpers::into_raw_mut(runtime_ptr);

    wasmer_result_t::WASMER_OK
}

#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_destroy(raw_runtime: *mut c_void) -> wasmer_result_t {
    debug!("`svm_runtime_create`");

    let runtime: Box<RuntimePtr> = Box::from_raw(raw_runtime as *mut RuntimePtr);
    std::mem::drop(runtime);

    wasmer_result_t::WASMER_OK
}

/// Builds an instance of `svm_contract_t`.
/// Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_contract_build(
    raw_runtime: *mut c_void,
    raw_contract: *mut *mut c_void,
    raw_bytes: *const c_void,
    raw_bytes_len: u64,
) -> wasmer_result_t {
    debug!("`svm_contract_build start`");

    let bytes = std::slice::from_raw_parts(raw_bytes as *const u8, raw_bytes_len as usize);
    let runtime = helpers::cast_to_runtime(raw_runtime);
    let result = runtime.contract_build(&bytes);

    match result {
        Ok(contract) => {
            *raw_contract = helpers::into_raw_mut(contract);
            debug!("`svm_contract_build returns `WASMER_OK`");
            wasmer_result_t::WASMER_OK
        }
        Err(err) => {
            update_last_error(err);
            error!("`svm_contract_build returns `WASMER_ERROR`");
            wasmer_result_t::WASMER_ERROR
        }
    }
}

/// Derives the contract to-be-deployed acccunt address and retures a pointer to it
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_contract_derive_address(
    raw_runtime: *const c_void,
    raw_contract: *const c_void,
) -> *const c_void {
    debug!("`svm_contract_compute_address`");

    let runtime = helpers::cast_to_runtime(raw_runtime);
    let contract = helpers::from_raw::<Contract>(raw_contract);

    let addr = runtime.contract_derive_address(contract);
    helpers::into_raw(addr)
}

/// Stores the new deployed contract under a database.
/// Future transaction will reference the contract by it's account address.
/// (see `svm_transaction_exec`)
///
/// This function should be called after performing validation.
///
/// * `raw_contract` - The wasm contract to be stored
///
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_contract_deploy(
    raw_runtime: *mut c_void,
    raw_contract: *const c_void,
    raw_addr: *const c_void,
) -> wasmer_result_t {
    debug!("`svm_contract_store` start");

    let contract = helpers::from_raw::<Contract>(raw_contract);
    let addr = Address::from(raw_addr);

    let runtime = helpers::cast_to_runtime_mut(raw_runtime);
    runtime.contract_deploy(contract, &addr);

    debug!("`svm_contract_build returns `WASMER_OK`");

    wasmer_result_t::WASMER_OK
}

/// Builds an instance of `svm_transaction_t`.
/// Should be called while the transaction is in the `mempool` of the full-node (prior mining it).
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_transaction_build(
    raw_runtime: *const c_void,
    raw_tx: *mut *mut c_void,
    raw_bytes: *const c_void,
    raw_bytes_len: u64,
) -> wasmer_result_t {
    let bytes: &[u8] = std::slice::from_raw_parts(raw_bytes as *const u8, raw_bytes_len as usize);

    let runtime = helpers::cast_to_runtime(raw_runtime);
    let result = runtime.transaction_build(bytes);

    match result {
        Ok(tx) => {
            *raw_tx = helpers::into_raw_mut(tx);
            debug!("`svm_contract_build returns `WASMER_OK`");
            wasmer_result_t::WASMER_OK
        }
        Err(error) => {
            update_last_error(error);
            error!("`svm_contract_build returns `WASMER_ERROR`");
            wasmer_result_t::WASMER_ERROR
        }
    }
}

/// Triggers a transaction execution of an already deployed contract.
///
/// `receipt` - The receipt of the contract execution.
/// `tx`      - The transaction to execute.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_transaction_exec(
    raw_receipt: *mut *mut c_void,
    raw_runtime: *mut c_void,
    raw_tx: *const c_void,
    raw_state: *const c_void,
    raw_pages_count: libc::c_int,
) -> wasmer_result_t {
    debug!("`svm_transaction_exec` start");

    let tx = helpers::from_raw::<Transaction>(raw_tx);
    let runtime = helpers::cast_to_runtime_mut(raw_runtime);
    let state = State::from(raw_state);

    let settings = ContractSettings {
        pages_count: raw_pages_count as u32,
        kv_path: String::new(),
    };

    let receipt = runtime.transaction_exec(&tx, &state, &settings);

    *raw_receipt = helpers::into_raw_mut(receipt);

    debug!("`svm_contract_build returns `WASMER_OK`");

    wasmer_result_t::WASMER_OK
}

/// Returns the transaction execution results (wasm array).
/// Should be called only after verifying that the transaction succeeded.
/// Will panic when called for a failed transaction.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_receipt_results(
    raw_receipt: *const c_void,
    results: *mut *mut wasmer_value_t,
    results_len: *mut u32,
) {
    debug!("`svm_receipt_results`");

    let receipt = helpers::from_raw::<Receipt>(raw_receipt);

    if receipt.success {
        let mut c_results = Vec::with_capacity(*results_len as usize);

        for value in receipt.results.iter() {
            let c_value = wasmer_value_t::from(value.clone());
            c_results.push(c_value);
        }

        // TODO: free `c_results` memory after usage
        let c_results: &mut Vec<wasmer_value_t> = Box::leak(Box::new(c_results));

        *results = c_results.as_mut_ptr();
        *results_len = receipt.results.len() as u32;
    } else {
        let msg = "method not allowed to be called when transaction execution failed";
        error!("{}", msg);
        panic!("{}", msg);
    }
}

/// Returns the `receipt` error in transaction failed
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_receipt_error(raw_receipt: *const c_void) {
    let receipt = helpers::from_raw::<Receipt>(raw_receipt);

    if let Some(ref _e) = receipt.error {
        // TODO: implement `std::error::Error` for `svm_runtime::runtime::error::ContractExecError`
        // update_last_error(e);
    }
}

/// Returns a pointer to the new state of the contract account.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_receipt_new_state(raw_receipt: *const c_void) -> *const u8 {
    let receipt = helpers::from_raw::<Receipt>(raw_receipt);

    if receipt.success {
        let state = receipt.new_state.as_ref().unwrap();
        state.as_ptr()
    } else {
        panic!("method not allowed to be called when transaction execution failed");
    }
}
