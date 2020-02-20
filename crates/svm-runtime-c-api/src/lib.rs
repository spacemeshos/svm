#![allow(missing_docs)]
#![allow(unused)]
#![feature(vec_into_raw_parts)]

//! This crate is responsible of providing [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `SVM`.

/// Crate common helpers
pub mod helpers;

/// Crate common tests specific helpers
pub mod testing;

mod address;
mod api;
mod byte_array;
mod import;
mod macros;
mod receipt;
mod result;
mod state;
mod value;
mod wasmer;

/// `SVM` FFI Interface
pub use api::{
    svm_byte_array_destroy, svm_deploy_template, svm_exec_app, svm_import_func_build,
    svm_imports_alloc, svm_imports_destroy, svm_instance_context_host_get, svm_parse_exec_app,
    svm_runtime_create, svm_runtime_destroy, svm_spawn_app,
};
pub use byte_array::svm_byte_array;
pub use import::{
    svm_import_func_sig_t, svm_import_func_t, svm_import_kind, svm_import_t, svm_import_value,
};
pub use result::svm_result_t;
pub use value::{svm_value_type, svm_value_type_array};

mod runtime_ptr;
pub use runtime_ptr::RuntimePtr;
