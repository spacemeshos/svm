//! This crate serves a wrapper around `wasmer` compiler.
//! Additionally, it implements required `wasmer` compiler milddlewares for `SVM` usage.

#![deny(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![deny(unreachable_code)]

mod compiler;
mod middleware;

pub use compiler::{compile, new_store};
