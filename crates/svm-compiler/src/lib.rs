#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

//! This crate serves a wrapper around `wasmer` compiler. Additionally, it implements required
//! `wasmer` compiler milddlewares for `SVM` usage.

mod compiler;
mod middleware;

pub use compiler::compile_program;
