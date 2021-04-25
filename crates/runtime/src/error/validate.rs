use std::fmt;

use svm_codec::ParseError;
use svm_gas::ProgramError;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum ValidateError {
    Parse(ParseError),

    Program(ProgramError),
}

impl From<ParseError> for ValidateError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}

impl From<ProgramError> for ValidateError {
    fn from(err: ProgramError) -> Self {
        Self::Program(err)
    }
}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidateError::Program(err) => err.fmt(f),
            ValidateError::Parse(err) => err.fmt(f),
        }
    }
}
