use thiserror::Error;

use std::fmt;

/// Represents error that may occur while doing gas estimation
#[derive(Debug, PartialEq, Clone, Error)]
pub enum ProgramError {
    /// Invalid wasm
    InvalidWasm,
    /// No valid `svm_alloc` function found.
    FunctionNotFound { func_name: String },
    /// Floats not allowed
    FloatsNotAllowed,
    /// Too many function imports
    TooManyFunctionImports,
    /// Function index is too large
    FunctionIndexTooLarge,
    /// `call_indirect` isn't allowed
    CallIndirectNotAllowed,
    /// Wasm has no `code` section
    MissingCodeSection,
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}