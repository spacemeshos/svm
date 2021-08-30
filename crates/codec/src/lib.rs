//! This crate is responsible for doing the binary encoding for SVM transactions.
//! It code is compiled as a single WASM file and it should be integrated by clients (e.g `smapp / CLI Wallet`).
//!
//! By doing that, a client can locally encode a binary transaction without having to re-implement all the logic
//! of the `svm-codec`.
//!
//! SVM's CI emits the WASM package of `svm-codec` as one of its artifacts (`svm_codec.wasm`)

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![allow(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(vec_into_raw_parts)]

mod codec;
mod error;
mod ext;
mod field;
mod section;

pub mod api;
pub mod receipt;
pub mod template;

pub use codec::Codec;
pub use error::ParseError;
pub use ext::{ReadExt, WriteExt};
pub use field::Field;
pub use section::{SectionPreview, SectionsDecoder, SectionsEncoder};
