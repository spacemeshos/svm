//! This crate implements SDK types or SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (not using Rust standard library) annotation in order to reduce the compiled WASM size.

#![no_std]
#![feature(maybe_uninit_uninit_array)]
#![feature(core_intrinsics)]
#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

/// SDK types
pub mod types;

/// SDK values
pub mod value;

mod amount;
pub use amount::Amount;

mod layer_id;
pub use layer_id::LayerId;

mod blob;
pub use blob::Address;

#[cfg(test)]
extern crate std;

#[cfg(test)]
fn to_std_string<T: svm_sdk_std::ToString>(value: T) -> std::string::String {
    let string: svm_sdk_std::String = value.to_string();
    let bytes = string.as_bytes();

    unsafe { std::string::String::from_utf8_unchecked(bytes.to_vec()) }
}

#[allow(missing_docs)]
pub trait Storage {
    fn get32(var_id: u32) -> u32;

    fn get64(var_id: u32) -> u64;

    fn set32(var_id: u32, value: u32);

    fn set64(var_id: u32, value: u64);

    fn store160(var_id: u32, offset: usize);

    fn load160(var_id: u32, offset: usize);
}

#[allow(missing_docs)]
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
