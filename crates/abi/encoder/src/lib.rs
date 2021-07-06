//! This crate is responsible of encoding SVM types (its actual type and their values to be precise),
//! according to a simple ABI format.

#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

mod traits;
mod types;

pub use traits::{ByteSize, Encoder};
pub use types::*;
