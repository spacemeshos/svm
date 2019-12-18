use std::ffi::c_void;

use crate::RuntimePtr;
use svm_runtime::traits::Runtime;

#[inline(always)]
pub fn into_raw<T>(obj: T) -> *const c_void {
    let boxed_obj = Box::new(obj);
    let raw_obj_ptr: *const _ = Box::into_raw(boxed_obj);

    raw_obj_ptr as _
}

#[inline(always)]
pub fn into_raw_mut<T>(obj: T) -> *mut c_void {
    let boxed_obj = Box::new(obj);
    let raw_obj_ptr: *mut _ = Box::into_raw(boxed_obj);

    raw_obj_ptr as _
}

#[inline(always)]
pub unsafe fn from_raw<'a, T>(raw_obj: *const c_void) -> &'a T {
    &*(raw_obj as *const T)
}

#[inline(always)]
pub unsafe fn from_raw_mut<'a, T>(raw_obj: *mut c_void) -> &'a mut T {
    &mut *(raw_obj as *mut T)
}

#[inline(always)]
pub unsafe fn cast_to_runtime<'a>(raw_runtime: *const c_void) -> &'a Box<dyn Runtime> {
    &*(raw_runtime as *const RuntimePtr)
}

#[inline(always)]
pub unsafe fn cast_to_runtime_mut<'a>(raw_runtime: *mut c_void) -> &'a mut Box<dyn Runtime> {
    &mut *(raw_runtime as *mut RuntimePtr)
}
