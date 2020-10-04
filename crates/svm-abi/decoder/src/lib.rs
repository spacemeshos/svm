#![no_std]

//! This crate is responsible on decoding function buffers.
//! Its code is meant to be used as part of SVM Templates (i.e Smart-Contract) code.
//! That's the reason why we add to the crate the `#![no_std]` annotation.
//! (not using the Rust stdlib should result in smaller WASM file).

//! Besides Smart-Contracts, it crate should be ready to called in other contexts.
//! For example, a `Wallet Apps` (UX or CLI)` or `Transactions Explorer` should be able to interpret an encoded function buffer
//! (which is part of the SVM transaction) in a friendly manner.
//! That's why the `svm-codec` crate also exposes a `decode func-buf` interface.
//!
//! For more info regarding the encoding scheme see the counterpart `svm-abi-encoder` crate.
//!

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

mod calldata;
mod cursor;
mod decoder;

pub use calldata::CallData;
pub(crate) use cursor::Cursor;
pub(crate) use decoder::{DecodeError, Decoder};
