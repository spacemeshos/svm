use std::ffi::c_void;
use std::ops::{Deref, DerefMut};

use svm_runtime::Runtime;
use svm_types::Type;

/// A Smart-pointer for a [`Runtime`] implementor.
///
/// Its main usage will be FFI related code.
#[repr(C)]
pub struct RuntimePtr {
    inner: Box<dyn Runtime>,
}

static RUNTIME_PTR_TY: Type = Type::of::<RuntimePtr>();

impl RuntimePtr {
    /// A new [`RuntimePtr`] smart pointer.
    pub fn new(inner: Box<dyn Runtime>) -> Self {
        Self { inner }
    }

    /// Copies the [`RuntimePtr`] into the heap, and returns a raw pointer to
    /// it.
    pub fn into_raw(self) -> *mut c_void {
        crate::into_raw(RUNTIME_PTR_TY, self)
    }

    /// Converts a raw pointer into [`RuntimePtr`].
    pub fn from_raw(ptr: *mut c_void) -> Self {
        let ptr: *mut RuntimePtr = ptr as _;

        crate::from_raw(RUNTIME_PTR_TY, ptr)
    }
}

impl<'a> From<*mut c_void> for &'a mut Box<dyn Runtime> {
    fn from(ptr: *mut c_void) -> Self {
        let ptr: &mut RuntimePtr = unsafe { crate::as_mut(ptr) };

        &mut *ptr
    }
}

impl Deref for RuntimePtr {
    type Target = Box<dyn Runtime>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for RuntimePtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
