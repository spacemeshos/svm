#![deny(missing_docs)]
#![deny(unused)]

//! `svm-runtime` crate is the glue between `svm` constract-storage to `wasmer` live instances.

/// Implements the most high-level API of `svm`.
#[macro_use]
pub mod runtime;

/// Wraps the `node data` (of type `*const std::ffi::c_void`) in a thread-safe way
pub mod ctx_data_wrapper;

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

/// Options when spawning a new `svm` runtime instance
pub mod opts;
