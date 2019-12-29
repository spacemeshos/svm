#![allow(missing_docs)]
#![allow(unused)]

//! This crate is responsible of providing [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `SVM`.

pub mod helpers;
pub mod testing;

mod api;
mod import;
mod result;
mod value;
mod wasmer;

/// `SVM` FFI Interface
pub use api::{
    svm_contract_build, svm_contract_deploy, svm_contract_derive_address,
    svm_instance_context_host_get, svm_receipt_error, svm_receipt_new_state, svm_receipt_results,
    svm_receipt_status, svm_runtime_create, svm_runtime_destroy, svm_transaction_build,
    svm_transaction_exec,
};
pub use import::{
    svm_byte_array, svm_import_func_sig_t, svm_import_func_t, svm_import_kind, svm_import_t,
    svm_import_value,
};
pub use result::svm_result_t;
pub use value::{svm_value, svm_value_t, svm_value_type};

mod runtime_ptr;
pub use runtime_ptr::RuntimePtr;
