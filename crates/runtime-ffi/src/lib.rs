//! This crate is responsible of providing [FFI] interface for `SVM`.
//!
//! [FFI]: https://doc.rust-lang.org/nomicon/ffi.html

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(ptr_as_uninit)]
#![feature(try_trait_v2)]

mod c_api;
mod resource_tracker;
mod result;

pub use c_api::*;
pub use result::svm_result_t;
