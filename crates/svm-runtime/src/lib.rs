#![allow(missing_docs)]
#![allow(unused)]

//! `SVM-runtime` crate is the glue between `SVM` to `wasmer`.

/// Implements the most high-level API of `SVM`.
mod runtime;

pub use runtime::Runtime;

/// Wraps the `node data` (of type `*const std::ffi::c_void`) in a thread-safe way
pub mod ctx_data_wrapper;

/// Implements `SvmCtx`. Used for running `SVM` instances.
pub mod ctx;

/// Implements register abstraction to ease interfacing
/// with the contract-storage / `wasmer` instance memory.
pub mod register;

/// `helpers` implements the helpers to be consumed by `SVM` vmcalls.
#[macro_use]
pub mod helpers;

pub mod testing;

/// Implements the `SVM` vmcalls (a.k.a libcalls / hostcalls / syscalls)
pub mod vmcalls;

/// Options when spawning a new `SVM` runtime instance
pub mod opts;
