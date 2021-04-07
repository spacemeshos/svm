#![no_std]
#![feature(core_intrinsics)]
#![allow(unused)]

mod log;

pub use log::*;

#[link_section = "svm"]
extern "C" {
    fn svm_log(msg_ptr: u32, msg_len: u32, code: u32);
}

/// Log the string `msg` along with code `code` into the running App logs.
pub fn log(msg: &str, code: u8) {
    let ptr = msg.as_ptr() as u32;
    let len = msg.len() as u32;

    unsafe { svm_log(ptr, len, code as u32) }
}

mod option;
pub use option::Option;

mod result;
pub use result::Result;

mod vec;
pub use vec::Vec;

mod panic;
pub use panic::panic;

#[macro_use]
pub mod ensure;

#[macro_use]
pub mod r#try;
