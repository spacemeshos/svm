#![deny(missing_docs)]
#![deny(unused)]

//! `svm-common` crate groups common shared code between the other `SVM` crates

mod address;
mod balance;
mod default_key_hasher;
mod key_hasher;
mod macros;
mod state;

/// Utility functions for messing mainly with bytes
pub mod utils;

pub use address::Address;
pub use balance::Balance;
pub use default_key_hasher::DefaultKeyHasher;
pub use key_hasher::KeyHasher;
pub use state::State;
