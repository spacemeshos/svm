use std::ffi::c_void;

use crate::RuntimePtr;

use svm_runtime::{Import, Runtime};

/// Casts raw pointer to borrowed Runtime
#[inline]
pub unsafe fn cast_to_runtime<'a>(raw_runtime: *const c_void) -> &'a Box<dyn Runtime> {
    &*(raw_runtime as *const RuntimePtr)
}

/// Casts raw pointer to mutably borrowed Runtime
#[inline]
pub unsafe fn cast_to_runtime_mut<'a>(raw_runtime: *mut c_void) -> &'a mut Box<dyn Runtime> {
    &mut *(raw_runtime as *mut RuntimePtr)
}

pub unsafe fn cast_to_imports<'a>(imports: *const c_void) -> &'a mut Vec<Import> {
    &mut *(imports as *mut Vec<Import>)
}
