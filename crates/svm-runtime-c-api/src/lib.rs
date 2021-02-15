#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![feature(vec_into_raw_parts)]

//! This crate is responsible of providing [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `SVM`.

/// Crate common tests specific helpers
pub mod testing;

mod api;
mod error;
mod result;

pub(crate) use error::{raw_error, raw_io_error, raw_utf8_error, raw_validate_error};

#[cfg(feature = "default-rocksdb")]
pub use api::svm_runtime_create;

/// `SVM` FFI Interface
#[rustfmt::skip]
pub use api::{
    // Runtime
    svm_exec_app,
    svm_deploy_template,
    svm_spawn_app,
    
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
    svm_wasm_error_create,

    // Resources tracking
    svm_total_live_resources,
    svm_resource_iter_new,
    svm_resource_iter_next,
    svm_resource_iter_destroy,
    svm_resource_destroy,
    svm_resource_type_name_resolve,
    svm_resource_type_name_destroy
};

pub use result::svm_result_t;
