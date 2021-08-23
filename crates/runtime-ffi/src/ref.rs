use std::ffi::c_void;
use std::ops::{Deref, DerefMut};

use svm_runtime::Runtime;
use svm_types::Type;

/// A Smart-Pointer for a [`Runtime`] implementation.
///
/// Its main usage will be FFI related code.
#[repr(transparent)]
pub struct RuntimeRef {
    inner: Box<dyn Runtime>,
}

static RUNTIME_REF_TYPE: Type = Type::of::<RuntimeRef>();

impl RuntimeRef {
    /// A new [`RuntimeRef`] smart pointer.
    pub fn new(inner: Box<dyn Runtime>) -> Self {
        Self { inner }
    }

    /// Copies the [`RuntimeRef`] into the heap, and returns a raw pointer to
    /// it.
    pub fn into_raw(self) -> *mut c_void {
        crate::into_raw(RUNTIME_REF_TYPE, self)
    }

    /// Converts a raw pointer into [`RuntimeRef`].
    pub unsafe fn from_raw(ptr: *mut c_void) -> Self {
        let ptr: *mut RuntimeRef = ptr as _;

        crate::from_raw(RUNTIME_REF_TYPE, ptr)
    }
}

impl Deref for RuntimeRef {
    type Target = Box<dyn Runtime>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for RuntimeRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
