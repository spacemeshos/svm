#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! `svm-common` crate groups common shared code between the other `SVM` crates

mod default_key_hasher;
mod ffi;
mod key_hasher;

/// Common Formatters
pub mod fmt;

pub use default_key_hasher::DefaultKeyHasher;
pub use ffi::{from_raw, from_raw_mut, into_raw, into_raw_mut};
pub use key_hasher::KeyHasher;
