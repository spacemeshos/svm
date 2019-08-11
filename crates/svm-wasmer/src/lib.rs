#![deny(missing_docs)]
// #![deny(unused)]
#![allow(unused)]

//! `svm-wasmer` crate is the glue between `svm` constract storage to `wasmer` live instances.

/// Implements `SvmCtx`. Used for running `svm-wasmer` instances.
pub mod ctx;

/// Implements the `Transaction`. Used for aggregation of the execution data of a contract transaction.
pub mod transaction;

/// Implements a `svm-wasmer` register abstraction to ease interfacing
/// with the contract-storage / `wasmer` instance memory
pub mod register;

/// `macros` implements the high-level macros to be consumed by `svm-wasmer` libcalls when dealing
/// with `storage` / `registers`
#[macro_use]
pub mod macros;

/// Implements the `svm` vmcalls (a.k.a libcalls / hostcalls / syscalls)
/// to be intergrated into `wasmer` instances running in the `svm`
pub mod vmcalls;

/// Setting up the `svm` env
pub mod env;
