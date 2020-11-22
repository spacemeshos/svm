use std::ffi::c_void;

use crate::svm_wasm_types_t;

use svm_types::WasmType;

#[allow(non_camel_case_types)]
#[derive(Clone)]
#[repr(C)]
pub struct svm_env_t {
    pub inner_env: *const c_void,

    pub host_env: *const c_void,

    pub returns: svm_wasm_types_t,
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

    pub unsafe fn return_types(&self) -> &[WasmType] {
        let ptr = &*(self.returns.ptr as *const WasmType);
        let length = self.returns.length;

        std::slice::from_raw_parts(ptr, length)
    }
}

impl From<*mut c_void> for &svm_env_t {
    fn from(env: *mut c_void) -> Self {
        unsafe { &*(env as *mut svm_env_t) }
    }
}

impl Drop for svm_env_t {
    fn drop(&mut self) {
        unsafe {
            let ptr = &mut *(self.returns.ptr as *mut WasmType);
            let length = self.returns.length;
            let capacity = self.returns.capacity;

            let _ = Vec::from_raw_parts(ptr, length, capacity);
        }

        dbg!("dropping `svm_env_t`");
    }
}
