use crate::String;

/// The external function (a.k.a host function) to be called from `log`.
#[allow(unused)]
#[cfg(target_arch = "wasm32")]
#[link_section = "svm"]
extern "C" {
    fn svm_log(offset: u32, length: u32);
}

/// Logs the given [`String`]
#[cfg(target_arch = "wasm32")]
pub fn log(data: &String) {
    let offset = data.as_ptr() as u32;
    let length = data.as_bytes().len() as u32;

    unsafe { svm_log(offset, length) }
}

/// Stub method implementation (when code isn't compiled into Wasm)
#[cfg(not(target_arch = "wasm32"))]
pub fn log(_data: &String) {
    //
}
