//! This crate is responsible of providing [FFI] interface for `SVM`.
//!
//! [FFI]: https://doc.rust-lang.org/nomicon/ffi.html

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![feature(vec_into_raw_parts)]

mod address;
mod byte_array;
mod macros;
mod state;
mod tracking;

pub(crate) use byte_array::svm_byte_array;
pub(crate) use tracking::{svm_resource_iter_t, svm_resource_t};

use std::ffi::c_void;
use svm_types::Type;

/// Type for `Wasm Error`
pub static SVM_WASM_ERROR_TYPE: Type = Type::Str("Wasm error type");

/// Type for a pointer to `Wasm Error`
pub static SVM_WASM_ERROR_TYPE_PTR: Type = Type::Str("Wasm error type pointer");

/// Type for `svm_resource_t`
pub static SVM_RESOURCE_TYPE: Type = Type::of::<svm_resource_t>();

/// Type for `svm_resource_iter_t`
pub static SVM_RESOURCES_ITER_TYPE: Type = Type::of::<svm_resource_iter_t>();

/// Type that represents a resource-name
pub static SVM_RESOURCE_NAME_TYPE: Type = Type::Str("resource-name");

/// Type that represents pointer to a resource-name
pub static SVM_RESOURCE_NAME_PTR_TYPE: Type = Type::Str("resource-name ptr");

/// Receives an object, and returns a raw `*mut c_void` pointer to it.
#[must_use]
#[inline]
pub fn into_raw<T: 'static>(ty: svm_types::Type, obj: T) -> *mut c_void {
    let ptr: *mut T = Box::into_raw(Box::new(obj));

    tracking::increment_live(ty);

    ptr as _
}

/// Given a pointer to a `T` object allocated on the heap, returns its (uses `Box::from_raw`)
#[must_use]
#[inline]
pub fn from_raw<T: 'static>(ty: svm_types::Type, ptr: *mut T) -> T {
    tracking::decrement_live(ty);

    unsafe { *Box::from_raw(ptr) }
}

/// # Safety
///
/// Receives a `*const c_void` pointer and returns the a mutable borrowed reference to the underlying object.
#[must_use]
#[inline]
pub unsafe fn as_mut<'a, T>(ptr: *mut c_void) -> &'a mut T {
    &mut *(ptr as *mut T)
}

mod api;
mod error;
mod result;

#[cfg(feature = "default-rocksdb")]
pub(crate) use error::raw_utf8_error;

pub(crate) use error::{raw_io_error, raw_validate_error};

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
