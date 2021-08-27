#![allow(missing_docs)]

use std::string::FromUtf8Error;

use thiserror::Error;

use crate::Field;

#[derive(Debug, Copy, Clone, Error, PartialEq)]
pub enum EofError {
    #[error("Unexpected EOF.")]
    Eof,
}

#[derive(Debug, Copy, Clone, Error, PartialEq)]
pub enum BoolError {
    #[error("Unexpected EOF.")]
    Eof(#[from] EofError),
    #[error("Invalid byte, not a bool.")]
    InvalidByte(u8),
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum StringError {
    #[error("Unexpected EOF.")]
    Eof(#[from] EofError),
    #[error("Invalid UTF-8: {0}")]
    Utf8(#[from] FromUtf8Error),
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Error, PartialEq)]
pub enum ParseError {
    #[error("Reached EOF")]
    Eof,

    #[error("Field `{0}` must be a valid UTF-8 string")]
    String(#[from] StringError),

    #[error("Unexpected Wasm value layout for field `{0}`")]
    UnexpectedLayout(Field),

    #[error("Invalid section kind")]
    InvalidSection,

    #[error("Generic error")]
    Other,
}

impl From<BoolError> for ParseError {
    fn from(_: BoolError) -> Self {
        Self::Other
    }
}

impl From<EofError> for ParseError {
    fn from(_: EofError) -> Self {
        Self::Eof
    }
}
