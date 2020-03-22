use std::ffi::c_void;

/// Receives an object, and returns a raw `*const c_void` pointer to it.
#[inline]
pub fn into_raw<T>(obj: T) -> *const c_void {
    let boxed_obj = Box::new(obj);
    let raw_obj_ptr: *const _ = Box::into_raw(boxed_obj);

    raw_obj_ptr as _
}

/// Receives an object, and returns a raw `*mut c_void` pointer to it.
#[inline]
pub fn into_raw_mut<T>(obj: T) -> *mut c_void {
    let boxed_obj = Box::new(obj);
    let raw_obj_ptr: *mut _ = Box::into_raw(boxed_obj);

    raw_obj_ptr as _
}

/// # Safety
///
/// Receives a `*const c_void` pointer and returns the a borrowed reference to the underlying object.
#[inline]
pub unsafe fn from_raw<'a, T>(raw_obj: *const c_void) -> &'a T {
    &*(raw_obj as *const T)
}

/// # Safety
///
/// Receives a `*const c_void` pointer and returns the a mutable borrowed reference to the underlying object.
#[inline]
pub unsafe fn from_raw_mut<'a, T>(raw_obj: *mut c_void) -> &'a mut T {
    &mut *(raw_obj as *mut T)
}
