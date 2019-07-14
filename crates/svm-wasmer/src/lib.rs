// #![deny(missing_docs)]
// #![deny(unused)]
// #![allow(dead_code)]
//! `svm-wasmer` crate is the glue between `svm` constract storage to `wasmer` live instances

/// Implements the `SvmCtx` use for running `svm-wasmer` instances
mod ctx;

/// Implements a `svm-wasmer` register abstraction to ease interfacing
/// with the contract-storage / `wasmer` instance memory
mod register;

/// `macros` implements the high-level macros to be consumed by `svm-wasmer` libcalls when dealing
/// with `storage` / `registers`
#[macro_use]
mod macros;

/// Implements the `svm` vmcalls (a.k.a libcalls / hostcalls / syscalls)
/// to be intergrated into `wasmer` instances running in the `svm`
mod vmcalls;
