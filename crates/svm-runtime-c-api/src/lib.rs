#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![feature(vec_into_raw_parts)]

//! This crate is responsible of providing [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `SVM`.

/// Crate common helpers
pub mod helpers;

/// Crate common tests specific helpers
pub mod testing;

mod address;
mod api;
mod byte_array;
mod error;
mod import;
mod layout;
mod macros;
mod result;
mod state;
mod types;
mod value;
mod wasmer;

pub(crate) use error::{raw_error, raw_io_error, raw_utf8_error, raw_validate_error};

/// `SVM` FFI Interface
#[rustfmt::skip]
pub use api::{
    // Runtime
    svm_exec_app,
    svm_deploy_template,
    svm_runtime_create,
    svm_runtime_destroy,
    svm_spawn_app,

    // Receipts helpers
    svm_app_receipt_addr,
    svm_template_receipt_addr,
    svm_app_receipt_state,
    svm_exec_receipt_returns,
    svm_exec_receipt_state,
    
    // Utils
    svm_byte_array_destroy,
    svm_instance_context_host_get,
    
    // Gas Estimations
    svm_estimate_deploy_template,
    svm_estimate_exec_app,
    svm_estimate_spawn_app,

    // Imports
    svm_import_func_build,
    svm_imports_alloc,
    svm_imports_destroy,

    // In-Memory
    svm_memory_state_kv_create,
    svm_state_kv_destroy,
    svm_memory_runtime_create,

    // FFI
    svm_ffi_state_kv_create,

    // Validations
    svm_validate_template,
    svm_validate_app,
    svm_validate_tx,
};
pub use byte_array::svm_byte_array;
pub use result::svm_result_t;

mod runtime_ptr;
pub use runtime_ptr::RuntimePtr;
