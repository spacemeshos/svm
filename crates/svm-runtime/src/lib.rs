#!allow(missing_docs)]
#![allow(unused)]

//! `svm-runtime` crate is the glue between `svm` constract-storage to `wasmer` live instances.

/// Implements the most high-level API of `svm`.
#[macro_use]
pub mod runtime;

/// Implements `SvmCtx`. Used for running `svm` instances.
pub mod ctx;

/// Implements register abstraction to ease interfacing
/// with the contract-storage / `wasmer` instance memory.
pub mod register;

/// `macros` implements the high-level macros to be consumed by `svm` vmcalls.
#[macro_use]
pub mod macros;

/// Implements the `svm` vmcalls (a.k.a libcalls / hostcalls / syscalls)
/// to be injected into `wasmer` instances running in the `svm`.
pub mod vmcalls;

pub mod opts;
