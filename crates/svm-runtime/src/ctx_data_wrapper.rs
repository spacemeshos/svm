use std::ffi::c_void;

/// Wraps the `node data` (of type `*const std::ffi::c_void`) in a thread-safe manner.
#[derive(Clone, Copy)]
pub struct SvmCtxDataWrapper {
    data: *const c_void,
}

impl SvmCtxDataWrapper {
    /// Receives a data pointer of type `*const std::ffi::c_void` to wrap
    pub fn new(data: *const c_void) -> Self {
        Self { data }
    }

    /// Releases `self` and returns its inner `data` field (`*const c_void`)
    pub fn unwrap(self) -> *const c_void {
        self.data
    }
}

unsafe impl Sync for SvmCtxDataWrapper {}
unsafe impl Send for SvmCtxDataWrapper {}
