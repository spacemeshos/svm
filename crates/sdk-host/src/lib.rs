//! This crate implements SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust standard-library) annotation in order to reduce the compiled WASM size.

#![no_std]
#![allow(missing_docs)]
#![allow(missing_docs)]

use svm_sdk_types::{Address, Amount, LayerId};

pub trait Host {
    fn calldata(&self) -> &'static [u8];

    fn set_returndata(&mut self, bytes: &[u8]);

    fn principal(&self) -> Address;

    fn target(&self) -> Address;

    fn value(&self) -> Amount;

    fn layer_id(&self) -> LayerId;

    fn balance(&self) -> Amount;

    fn transfer(&mut self, dst: &Address, amount: Amount);

    fn log(&mut self, msg: &str, code: u8);
}
