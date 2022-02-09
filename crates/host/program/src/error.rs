use thiserror::Error;

use std::fmt;

/// Represents error that may occur while doing gas estimation
#[derive(Debug, PartialEq, Clone, Error)]
pub enum ProgramError {
    /// Invalid wasm
    InvalidWasm,

    /// No valid `svm_alloc` function found.
    FunctionNotFound(String),

    /// Floats not allowed
    FloatsNotAllowed,

    /// Too many function imports
    TooManyFunctionImports,

    /// Function index is too large
    FunctionIndexTooLarge,

    /// Wasm has no `code` section
    MissingCodeSection,

    /// Invalid Export Kind
    InvalidExportKind,

    /// Invalid Export Function Signature
    InvalidExportFunctionSignature(String),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
