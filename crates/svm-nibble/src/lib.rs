#![no_std]
#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! This crate is managing reading/write streams of Nibble.
//! One nibble equals 4 bits (half a byte).
//! The mechanism implemented within this crate will be used for implementing:
//! * `svm-codec`
//! * `svm-abi`
//! * writing SVM apps.

mod concat;
mod iter;
mod macros;
mod nibble;
mod writer;

pub use concat::concat_nibbles;
pub use iter::NibbleIter;
pub use nibble::Nibble;
pub use writer::NibbleWriter;
