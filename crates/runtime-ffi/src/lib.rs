#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![feature(vec_into_raw_parts)]

//! This crate is responsible of providing [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `SVM`.

mod api;
mod error;
mod result;

#[cfg(feature = "default-rocksdb")]
pub(crate) use error::raw_utf8_error;

pub(crate) use error::{raw_error, raw_validate_error};

#[cfg(feature = "default-rocksdb")]
pub use api::svm_runtime_create;

#[cfg(feature = "default-memory")]
pub use api::svm_memory_runtime_create;

/// `SVM` FFI Interface
#[rustfmt::skip]
pub use api::{
    // Transactions Validation
    svm_validate_deploy,
    svm_validate_spawn,
    svm_validate_call,

    // Transactions Execution
    svm_call,
    svm_deploy,
    svm_spawn,

    // Destroy
    svm_runtime_destroy,
    svm_byte_array_destroy,

    // Error
    svm_wasm_error_create,

    // Allocation
    svm_envelope_alloc,
    svm_context_alloc,

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
