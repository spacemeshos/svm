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
mod macros;
mod receipt;
mod result;
mod state;
mod types;
mod value;
mod wasmer;

pub(crate) use error::{raw_error, raw_io_error, raw_utf8_error, raw_validate_error};

/// `SVM` FFI Interface
pub use api::{
    svm_app_receipt_addr, svm_app_receipt_state, svm_byte_array_destroy, svm_deploy_template,
    svm_estimate_deploy_template, svm_estimate_exec_app, svm_estimate_spawn_app, svm_exec_app,
    svm_exec_receipt_state, svm_import_func_build, svm_imports_alloc, svm_imports_destroy,
    svm_instance_context_host_get, svm_memory_kv_create, svm_memory_kv_create2,
    svm_memory_runtime_create, svm_runtime_create, svm_runtime_destroy, svm_spawn_app,
    svm_template_receipt_addr, svm_validate_tx,
};
pub use byte_array::svm_byte_array;
pub use result::svm_result_t;

mod runtime_ptr;
pub use runtime_ptr::RuntimePtr;
