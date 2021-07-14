use svm_codec::ParseError;
use svm_gas::ProgramError;
use thiserror::Error;

/// The error type that can arise when validating SVM-dialect WASM files.
#[derive(Debug, PartialEq, Clone, Error)]
pub enum ValidateError {
    /// An unexpected condition was found when decoding from the SVM ABI.
    #[error("{0}")]
    Parse(#[from] ParseError),
    /// The given WASM is invalid or it uses specific features which are not
    /// allowed by SVM.
    #[error("{0}")]
    Program(#[from] ProgramError),
}
