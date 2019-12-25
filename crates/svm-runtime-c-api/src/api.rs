use log::{debug, error, trace};
use std::ffi::c_void;

use svm_common::{Address, State};
use svm_contract::{transaction::Transaction, wasm::Contract};
use svm_runtime::{register::SvmReg, settings::ContractSettings, traits::Runtime, Receipt};

use crate::{helpers, RuntimePtr};

use wasmer_runtime::{Ctx, ImportObject};
use wasmer_runtime_c_api::{
    error::update_last_error,
    import::{wasmer_import_object_extend, wasmer_import_object_t, wasmer_import_t},
    value::wasmer_value_t,
};

#[repr(C)]
pub enum svm_result_t {
    SUCCESS = 0,
    FAILURE = 1,
}

/// Creates a new SVM Runtime instance.
/// Returns it via the `raw_runtime` parameter.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_create(
    raw_runtime: *mut *mut c_void,
    path_bytes: *const c_void,
    path_len: libc::c_uint,
    host: *mut c_void,
    imports: *mut c_void,
    imports_len: libc::c_uint,
) -> svm_result_t {
    debug!("`svm_runtime_create` start");

    let slice = std::slice::from_raw_parts(path_bytes as *const u8, path_len as usize);
    let path = String::from_utf8(slice.to_vec());

    if let Err(err) = path {
        update_last_error(err);
        return svm_result_t::FAILURE;
    }

    let imports = helpers::cast_host_imports(imports, imports_len);
    let runtime = svm_runtime::create_rocksdb_runtime(host, &path.unwrap(), imports);

    let runtime: Box<dyn Runtime> = Box::new(runtime);

    let runtime_ptr = RuntimePtr::new(runtime);
    *raw_runtime = svm_common::into_raw_mut(runtime_ptr);

    debug!("`svm_runtime_create` end");

    svm_result_t::SUCCESS
}

/// Destroys the Runtime and it's associated resources.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_runtime_destroy(raw_runtime: *mut c_void) -> svm_result_t {
    debug!("`svm_runtime_destroy`");

    let _runtime: Box<RuntimePtr> = Box::from_raw(raw_runtime as *mut RuntimePtr);

    svm_result_t::SUCCESS
}

/// Builds an in-memory contract instance.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_contract_build(
    raw_contract: *mut *mut c_void,
    raw_runtime: *mut c_void,
    raw_bytes: *const c_void,
    raw_bytes_len: u64,
) -> svm_result_t {
    debug!("`svm_contract_build start`");

    let bytes = std::slice::from_raw_parts(raw_bytes as *const u8, raw_bytes_len as usize);
    let runtime = helpers::cast_to_runtime(raw_runtime);

    match runtime.contract_build(&bytes) {
        Ok(contract) => {
            *raw_contract = svm_common::into_raw_mut(contract);
            debug!("`svm_contract_build returns `SUCCESS`");
            svm_result_t::SUCCESS
        }
        Err(err) => {
            update_last_error(err);
            error!("`svm_contract_build returns `FAILURE`");
            svm_result_t::FAILURE
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
    let contract = svm_common::from_raw::<Contract>(raw_contract);

    let addr = runtime.contract_derive_address(contract);
    svm_common::into_raw(addr)
}

/// Stores the new deployed contract under a database.
/// Future transaction will reference the contract by its account address.
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
) -> svm_result_t {
    debug!("`svm_contract_store` start");

    let contract = svm_common::from_raw::<Contract>(raw_contract);
    let addr = Address::from(raw_addr);

    let runtime = helpers::cast_to_runtime_mut(raw_runtime);
    runtime.contract_deploy(contract, &addr);

    debug!("`svm_contract_build returns `SUCCESS`");

    svm_result_t::SUCCESS
}

/// Builds an in-memory Contract Transaction instance.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_transaction_build(
    raw_tx: *mut *mut c_void,
    raw_runtime: *const c_void,
    raw_bytes: *const c_void,
    raw_bytes_len: u64,
) -> svm_result_t {
    debug!("`svm_transaction_build` start");

    let bytes = std::slice::from_raw_parts(raw_bytes as *const u8, raw_bytes_len as usize);
    let runtime = helpers::cast_to_runtime(raw_runtime);

    match runtime.transaction_build(bytes) {
        Ok(tx) => {
            *raw_tx = svm_common::into_raw_mut(tx);
            debug!("`svm_transaction_build returns `SUCCESS`");
            svm_result_t::SUCCESS
        }
        Err(error) => {
            update_last_error(error);
            error!("`svm_transaction_build returns `FAILURE`");
            svm_result_t::FAILURE
        }
    }
}

/// Triggers a transaction execution of an already deployed contract.
///
/// Returns the receipt of the contract execution via the `raw_receipt` parameter.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_transaction_exec(
    raw_receipt: *mut *mut c_void,
    raw_runtime: *mut c_void,
    raw_tx: *const c_void,
    raw_state: *const c_void,
    raw_pages_count: libc::c_int,
) -> svm_result_t {
    debug!("`svm_transaction_exec` start");

    let tx = svm_common::from_raw::<Transaction>(raw_tx);
    let runtime = helpers::cast_to_runtime_mut(raw_runtime);
    let state = State::from(raw_state);

    let settings = ContractSettings {
        pages_count: raw_pages_count as u32,
        kv_path: String::new(),
    };

    let receipt = runtime.transaction_exec(&tx, &state, &settings);

    *raw_receipt = svm_common::into_raw_mut(receipt);

    debug!("`svm_transaction_exec returns `SUCCESS`");

    svm_result_t::SUCCESS
}

/// Returns the transaction execution results (wasm array).
/// Should be called only after verifying that the transaction succeeded.
/// Will panic when called for a failed transaction.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_receipt_results(
    results: *mut *mut wasmer_value_t,
    raw_receipt: *const c_void,
    results_len: *mut u32,
) {
    debug!("`svm_receipt_results`");

    let receipt = svm_common::from_raw::<Receipt>(raw_receipt);

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
    let receipt = svm_common::from_raw::<Receipt>(raw_receipt);

    if let Some(ref _e) = receipt.error {
        // TODO: implement `std::error::Error` for `svm_runtime::runtime::error::ContractExecError`
        // update_last_error(e);
    }
}

/// Returns a pointer to the new state of the contract account.
#[must_use]
#[no_mangle]
pub unsafe extern "C" fn svm_receipt_new_state(raw_receipt: *const c_void) -> *const u8 {
    let receipt = svm_common::from_raw::<Receipt>(raw_receipt);

    if receipt.success {
        let state = receipt.new_state.as_ref().unwrap();
        state.as_ptr()
    } else {
        panic!("method not allowed to be called when transaction execution failed");
    }
}
