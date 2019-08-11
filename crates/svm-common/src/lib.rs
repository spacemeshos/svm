#![deny(missing_docs)]
#![deny(unused)]

//! `svm-common` crate groups common shared code between the other `svm` crates

mod address;
mod balance;
mod contract_state;
mod default_key_hasher;
mod key_hasher;
mod macros;

/// Utility functions for messing mainly with bytes
pub mod utils;

pub use address::Address;
pub use balance::Balance;
pub use contract_state::ContractState;
pub use default_key_hasher::DefaultKeyHasher;
pub use key_hasher::KeyHasher;
