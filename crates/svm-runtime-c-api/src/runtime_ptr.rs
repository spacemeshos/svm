use svm_runtime::{traits::Runtime, DefaultRuntime};

use std::ffi::c_void;
use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct RuntimePtr {
    inner: Box<dyn Runtime>,
}

impl RuntimePtr {
    pub fn new(inner: Box<dyn Runtime>) -> Self {
        Self { inner }
    }

    pub fn into_raw(self) -> *mut c_void {
        let boxed = Box::new(self);

        let ptr: *mut RuntimePtr = Box::into_raw(boxed) as _;

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
