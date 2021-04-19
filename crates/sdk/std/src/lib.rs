//! ## Why do we need this crate?
//!
//! We want the Wasm-compiled code of SVM Apps to have restrictions that will enable us to give fixed-gas estimations for transactions.
//! From the end-user point of view, he/she will know prior to dispatching a transaction to the network the exact quantity of gas units
//! required to execute the transaction without hitting the out of gas error.
//! Having said that, a transaction can still fail - for example by `panic`-ing.
//!
//! #### A restricted Wasm program should meet the following:
//!
//! * No usage of the `loop` opcode.
//! * No recursive calls /cycles detected when doing static analysis of the code.
//! * No usage of the `call_indirect` opcode (no function pointers, i.e no Polymorphism).
//!
//! In order to achieve the above, this crate serves as a tiny replacement for the default Rust `std`.
//! If to be more precise - part of the re-implemented parts reside within Rust `core`/ `alloc` itself and some belong to Rust `std`.
//! By introducing this crate we have full control of the emitted Wasm and we can cherry-pick only features that are relevant for us.

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

/// A replacement for the `core::option::Option` (exposed also as `std::option::Option`)
mod option;
pub use option::Option;

/// A replacement for the `core::result::Result` (exposed also as `std::result::Result`)
mod result;
pub use result::Result;

/// A replacement for the `alloc::vec::Vec` (exposed also as `std::vec::Vec`)
mod vec;
pub use vec::Vec;

/// A replacement for the `panic!` macro
mod panic;
pub use panic::panic;

/// A replacement for the `assert!` macro
#[macro_use]
pub mod ensure;

/// Implementation of the `safe_try` macro (replacement for `?` operator for error handling)
#[macro_use]
pub mod r#try;
