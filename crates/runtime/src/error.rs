use svm_codec::ParseError;
use svm_gas::ProgramError;
use thiserror::Error;

/// The error type that can arise when validating SVM-dialect WASM files.
#[derive(Debug, PartialEq, Clone, Error)]
pub enum ValidateError {
    #[error("{0}")]
    Parse(#[from] ParseError),
    #[error("{0}")]
    Program(#[from] ProgramError),
}
