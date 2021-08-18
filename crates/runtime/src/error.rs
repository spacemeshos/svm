use thiserror::Error;

use svm_codec::ParseError;
use svm_gas::FixedGasError;
use svm_program::ProgramError;

/// The error type that can arise when validating SVM-dialect WASM files.
#[derive(Debug, PartialEq, Clone, Error)]
pub enum ValidateError {
    /// An unexpected condition was found when decoding from the SVM ABI.
    #[error("{0}")]
    Parse(#[from] ParseError),

    /// The given smWasm is invalid or it uses specific features which are not
    /// allowed by SVM.
    #[error("{0}")]
    Program(#[from] ProgramError),

    /// The given smWasm code is valid, but it doesn't pass the requirements to
    /// run in fixed-gas mode.
    #[error("{0}")]
    FixedGas(#[from] FixedGasError),
}
