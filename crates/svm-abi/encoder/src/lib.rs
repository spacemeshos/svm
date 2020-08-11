//! This crate is responsible of encoding SVM types (its actual type and their values to be precise),
//! according to a simple ABI format.

#![no_std]
#![deny(missing_docs)]
#![deny(unused)]
#![allow(dead_code)]
#![deny(unreachable_code)]

mod layout;
mod traits;

pub mod types;

pub use traits::Encoder;
