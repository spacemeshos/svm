#![allow(missing_docs)]
#![allow(unused)]

//! This crate is responsible of providing a [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `SVM`.

/// C-API for the `SVM`
mod c_api;
pub use c_api::{
    svm_contract_build, svm_contract_deploy, svm_contract_derive_address, svm_receipt_error,
    svm_receipt_new_state, svm_receipt_results, svm_runtime_create, svm_runtime_destroy,
    svm_transaction_build, svm_transaction_exec,
};

mod runtime_ptr;
use runtime_ptr::RuntimePtr;

/// Types to be used for FFI integration.
pub mod c_types;

/// C-API utilities to be used primarily for tests / integration-tests
pub mod c_utils;
