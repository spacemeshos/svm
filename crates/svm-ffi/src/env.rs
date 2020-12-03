use std::ffi::c_void;

/// The function environment of host functions.
#[allow(non_camel_case_types)]
#[derive(Clone)]
#[repr(C)]
pub struct svm_env_t {
    /// The SVM's inner environment.
    /// (see `Context` at `svm-runtime` crate).
    pub inner_env: *const c_void,

    /// The host environment.
    pub host_env: *const c_void,
}

impl svm_env_t {
    /// Borrows the inner environment.
    pub unsafe fn inner<T>(&self) -> &T {
        &*{ self.inner_env as *const T }
    }

    /// Mutably-borrows the inner environment.
    pub unsafe fn inner_mut<T>(&self) -> &mut T {
        &mut *(self.inner_env as *const T as *mut T)
    }

    /// Borrows the host environment.
    pub unsafe fn host_env<T>(&self) -> &T {
        &*(self.host_env as *const T)
    }
}

impl From<*mut c_void> for &svm_env_t {
    fn from(env: *mut c_void) -> Self {
        unsafe { &*(env as *mut svm_env_t) }
    }
}

impl Drop for svm_env_t {
    fn drop(&mut self) {
        dbg!("dropping `svm_env_t`");
    }
}
