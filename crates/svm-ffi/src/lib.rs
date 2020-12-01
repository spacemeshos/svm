#![feature(vec_into_raw_parts)]

mod address;
mod byte_array;
mod callback;
mod env;
mod layout;
mod macros;
mod state;
mod types;
mod value;

pub use byte_array::svm_byte_array;
pub mod tracking;

pub use callback::svm_func_callback_t;
pub use env::svm_env_t;
pub use tracking::{svm_resource_iter_t, svm_resource_t};
pub use value::alloc_wasm_values;

use std::ffi::c_void;
use svm_types::Type;

pub static SVM_WASM_ERROR_TYPE: Type = Type::Str("wasm error type");
pub static SVM_WASM_ERROR_TYPE_PTR: Type = Type::Str("wasm error type pointer");
pub static SVM_RESOURCE_TYPE: Type = Type::of::<svm_resource_t>();
pub static SVM_RESOURCES_ITER_TYPE: Type = Type::of::<svm_resource_iter_t>();
pub static SVM_RESOURCE_NAME_TYPE: Type = Type::Str("resource-name");
pub static SVM_RESOURCE_NAME_PTR_TYPE: Type = Type::Str("resource-name ptr");

/// Receives an object, and returns a raw `*mut c_void` pointer to it.
#[must_use]
#[inline]
pub fn into_raw<T: 'static>(ty: svm_types::Type, obj: T) -> *mut c_void {
    let ptr: *mut T = Box::into_raw(Box::new(obj));

    tracking::increment_live_2(ty);

    ptr as _
}

#[must_use]
#[inline]
pub fn from_raw<T: 'static>(ty: svm_types::Type, ptr: *mut T) -> T {
    tracking::decrement_live_1(ty);

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
