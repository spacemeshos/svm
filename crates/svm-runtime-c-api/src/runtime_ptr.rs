use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use log::debug;

use svm_runtime::Runtime;

/// Smart-pointer for a `Runtime`
#[repr(C)]
pub struct RuntimePtr {
    inner: Box<dyn Runtime>,
}

impl RuntimePtr {
    /// A new `RuntimePtr` smart-pointer
    pub fn new(inner: Box<dyn Runtime>) -> Self {
        Self { inner }
    }

    /// Copies the `RuntimePtr` into the heap, and returns a raw pointer to it.
    pub fn into_raw(self) -> *mut c_void {
        let boxed = Box::new(self);

        let ptr: *mut RuntimePtr = Box::into_raw(boxed);

        ptr as _
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

impl Drop for RuntimePtr {
    fn drop(&mut self) {
        debug!("Dropping RuntimePtr...");
    }
}
