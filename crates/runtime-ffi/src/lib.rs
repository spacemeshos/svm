//! This crate is responsible of providing [FFI] interface for `SVM`.
//!
//! [FFI]: https://doc.rust-lang.org/nomicon/ffi.html

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(ptr_as_uninit)]
#![feature(try_trait_v2)]
#![feature(allocator_api)]

mod api;
mod result;

#[cfg(feature = "default-rocksdb")]
pub(crate) use error::raw_utf8_error;

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
    svm_deploy,
    svm_spawn,
    svm_verify,
    svm_call,

    // Destroy
    svm_runtime_destroy,
};

pub use result::svm_result_t;
