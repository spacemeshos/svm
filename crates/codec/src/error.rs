#![allow(missing_docs)]

use std::string::FromUtf8Error;

use thiserror::Error;

use crate::Field;

#[allow(missing_docs)]
#[derive(Debug, Clone, Error, PartialEq)]
pub enum ParseError {
    #[error("Reached EOF")]
    Eof,

    #[error("Expected a certain byte value; found one which is illegal.")]
    BadByte(u8),

    #[error("Found some UTF-8 invalid string. Can't continue.")]
    String(#[from] FromUtf8Error),

    #[error("Unexpected Wasm value layout for field `{0}`")]
    UnexpectedLayout(Field),

    #[error("Invalid section kind")]
    InvalidSection,

    #[error("Generic error")]
    Other,
}
