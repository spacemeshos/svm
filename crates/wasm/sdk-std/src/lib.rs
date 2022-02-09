//! ## Why do we need this crate?
//!
//! We want the Wasm-compiled code of SVM Templates to have restrictions that will enable us to give fixed-gas estimations for transactions.
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
#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(core_intrinsics)]
#![feature(try_trait_v2)]

mod log;
pub use log::log;

mod string;
pub use string::{DecDigit, HexDigit, String, StringBuilder, ToString};

/// A replacement for the `core::option::Option` (exposed also as `std::option::Option`)
mod option;
pub use option::Option;

/// A replacement for the `core::result::Result` (exposed also as `std::result::Result`)
mod result;
pub use result::Result;

/// A replacement for the `alloc::vec::Vec` (exposed also as `std::vec::Vec`)
mod vec;
pub use vec::{Vec, VecIntoIter};

/// A replacement for the `panic!` macro
mod panic;
pub use panic::panic;

/// A replacement for the `assert!` macro
#[macro_use]
pub mod ensure;

/// Implementation of the `safe_try` macro (replacement for `?` operator for error handling)
#[macro_use]
pub mod r#try;
