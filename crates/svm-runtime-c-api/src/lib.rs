#![allow(missing_docs)]
#![allow(unused)]

//! This crate is responsible of providing a [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `SVM`.

pub mod helpers;
pub mod testing;

/// `SVM` FFI Interface
mod api;
pub use api::{
    svm_contract_build, svm_contract_deploy, svm_contract_derive_address, svm_receipt_error,
    svm_receipt_new_state, svm_receipt_results, svm_result_t, svm_runtime_create,
    svm_runtime_destroy, svm_transaction_build, svm_transaction_exec,
};

mod runtime_ptr;
pub use runtime_ptr::RuntimePtr;
