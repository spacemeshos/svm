use std::ffi::c_void;

/// Wraps a pointer of type `*mut std::ffi::c_void` in a thread-safe manner.
#[derive(Clone, Copy)]
pub struct PtrWrapper {
    data: *mut c_void,
}

impl PtrWrapper {
    /// Receives a `*mut c_void` pointer to wrap
    pub fn new(data: *mut c_void) -> Self {
        Self { data }
    }

    /// Releases `self` and returns its wrapped pointer
    pub fn unwrap(self) -> *mut c_void {
        self.data
    }
}

unsafe impl Sync for PtrWrapper {}
unsafe impl Send for PtrWrapper {}
