//! This crate is responsible on decoding function buffers.
//! Its code is meant to be used as part of SVM Templates (i.e Smart-Contract) code.
//! That's the reason why we add to the crate the `#![no_std]` annotation.
//! (not using the Rust standard library should result in a smaller WASM file).

//! Besides Smart-Contracts, this crate should be ready to use in other contexts.
//! For example, a client such as `smapp` or the `Process Explorer` should be able to interpret an encoded `CallData`
//! (which is part of the SVM transaction) in a friendly manner.
//!
//! For more info regarding the encoding scheme see the counterpart `svm-abi-encoder` crate.
//!

#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

mod calldata;
mod cursor;
mod decoder;

pub use calldata::CallData;
pub use cursor::Cursor;
pub use decoder::{DecodeError, Decoder};

/// `ReturnData` is a type alias to `CallData` for now.
pub type ReturnData = CallData;
