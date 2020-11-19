use std::ffi::c_void;

#[allow(non_camel_case_types)]
#[derive(Clone)]
#[repr(C)]
pub struct svm_env_t {
    pub inner_env: *const c_void,

    pub host_env: *const c_void,
}

impl svm_env_t {
    pub unsafe fn inner<T>(&self) -> &T {
        &*{ self.inner_env as *const T }
    }

    pub unsafe fn inner_mut<T>(&self) -> &mut T {
        &mut *(self.inner_env as *const T as *mut T)
    }

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
