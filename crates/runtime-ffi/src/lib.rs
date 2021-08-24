//! This crate is responsible of providing [FFI] interface for `SVM`.
//!
//! [FFI]: https://doc.rust-lang.org/nomicon/ffi.html

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![feature(vec_into_raw_parts)]

mod byte_array;
mod r#ref;

pub mod tracking;

pub use byte_array::svm_byte_array;
pub use tracking::{svm_resource_iter_t, svm_resource_t};

use std::ffi::c_void;

use svm_types::Type;

mod api;
mod error;
mod result;

#[cfg(feature = "default-rocksdb")]
pub(crate) use error::raw_utf8_error;

pub(crate) use error::raw_error;

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
    svm_byte_array_destroy,

    // Allocation
    svm_envelope_alloc,
    svm_message_alloc,
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

/// Receives an object, and returns a raw `*mut c_void` pointer to it.
#[must_use]
#[inline]
pub fn into_raw<T: 'static>(ty: Type, obj: T) -> *mut c_void {
    let ptr: *mut T = Box::into_raw(Box::new(obj));

    tracking::increment_live(ty);

    ptr as _
}

/// Given a **pointer** to an object (of type `T`) allocated on the heap, returns the object (uses `Box::from_raw`)
#[must_use]
#[inline]
pub(crate) unsafe fn from_raw<T: 'static>(ty: Type, ptr: *mut T) -> T {
    tracking::decrement_live(ty);

    *Box::from_raw(ptr)
}

/// Receives a `*const c_void` pointer and returns the a mutable borrowed reference to the underlying object.
///
/// # Safety
///
/// * If raw pointer doesn't point to a struct of type T it's an U.B
/// * In case the referenced struct is already borrowed it's an U.B
#[must_use]
#[inline]
pub(crate) unsafe fn as_mut<'a, T>(ptr: *mut c_void) -> &'a mut T {
    &mut *(ptr as *mut T)
}
