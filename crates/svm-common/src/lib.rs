#![deny(missing_docs)]
#![deny(unused)]
#![feature(vec_into_raw_parts)]

//! `svm-common` crate groups common shared code between the other `SVM` crates

mod address;
mod address_of;

mod balance;
mod default_key_hasher;
mod ffi;
mod helpers;
mod key_hasher;
mod macros;
mod state;

/// Common Formatters
pub mod fmt;

pub use address::Address;
pub use address_of::AddressOf;
pub use balance::Balance;
pub use default_key_hasher::DefaultKeyHasher;
pub use ffi::{from_raw, from_raw_mut, into_raw, into_raw_mut};
pub use key_hasher::KeyHasher;
pub use state::State;
