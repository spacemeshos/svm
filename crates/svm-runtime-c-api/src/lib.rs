#![allow(missing_docs)]
#![allow(unused)]

//! This crate is responsible of providing [FFI](https://doc.rust-lang.org/nomicon/ffi.html) interface for the `SVM`.

/// Crate common helpers
pub mod helpers;

/// Crate common tests specific helpers
pub mod testing;

mod api;
mod import;
mod receipt;
mod result;
mod value;
mod wasmer;

/// `SVM` FFI Interface
pub use api::{
    svm_address_destroy, svm_deploy_template, svm_exec_app, svm_import_func_build,
    svm_imports_alloc, svm_imports_destroy, svm_instance_context_host_get, svm_parse_exec_app,
    svm_runtime_create, svm_runtime_destroy, svm_spawn_app, svm_state_destroy,
};
pub use import::{
    svm_byte_array, svm_import_func_sig_t, svm_import_func_t, svm_import_kind, svm_import_t,
    svm_import_value,
};
pub use result::svm_result_t;
pub use value::{svm_value_type, svm_value_type_array};

mod runtime_ptr;
pub use runtime_ptr::RuntimePtr;
