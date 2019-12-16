use std::ffi::c_void;

/// Wraps a pointer of type `*const std::ffi::c_void` in a thread-safe manner.
#[derive(Clone, Copy)]
pub struct PtrWrapper {
    data: *const c_void,
}

impl PtrWrapper {
    /// Receives a `*const c_void` pointer to wrap
    pub fn new(data: *const c_void) -> Self {
        Self { data }
    }

    /// Releases `self` and returns its wrapped pointer
    pub fn unwrap(self) -> *const c_void {
        self.data
    }
}

unsafe impl Sync for PtrWrapper {}
unsafe impl Send for PtrWrapper {}
