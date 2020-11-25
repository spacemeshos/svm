#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![feature(vec_into_raw_parts)]

//! This crate is responsible of providing [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `SVM`.

/// Crate common helpers
pub mod helpers;

/// Crate common tests specific helpers
pub mod testing;

mod api;
mod error;
mod result;

pub(crate) use error::{raw_error, raw_io_error, raw_utf8_error, raw_validate_error};

/// `SVM` FFI Interface
#[rustfmt::skip]
pub use api::{
    // Runtime
    svm_exec_app,
    svm_deploy_template,
    svm_runtime_create,
    svm_spawn_app,
    
    // Gas Estimations
    svm_estimate_deploy_template,
    svm_estimate_exec_app,
    svm_estimate_spawn_app,

    // Imports
    svm_import_func_new,
    svm_imports_alloc,

    // In-Memory
    svm_memory_state_kv_create,
    svm_memory_runtime_create,

    // FFI
    svm_ffi_state_kv_create,

    // Validations
    svm_validate_template,
    svm_validate_app,
    svm_validate_tx,

    // Destroy
    svm_runtime_destroy,
    svm_state_kv_destroy,
    svm_imports_destroy,
    svm_byte_array_destroy,

    // Error
    svm_wasm_error_create
};

pub use result::svm_result_t;

mod runtime_ptr;
use runtime_ptr::RuntimePtr;
