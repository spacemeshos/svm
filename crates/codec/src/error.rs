use thiserror::Error;

use std::fmt;

use crate::Field;

#[allow(missing_docs)]
#[derive(PartialEq, Clone, Error)]
pub enum ParseError {
    ReachedEOF,
    ExpectedEOF,
    EmptyField(Field),
    NotEnoughBytes(Field),
    TooManyBytes(Field),
    NotSupported(Field),
    InvalidUTF8String(Field),
    UnexpectedLayout(Field),
    InvalidSection,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::ReachedEOF => write!(f, "Reached EOF"),
            ParseError::ExpectedEOF => write!(f, "Expected EOF but there are more left bytes"),
            ParseError::EmptyField(field) => write!(f, "Field `{}` must not be empty", field),
            ParseError::NotEnoughBytes(field) => {
                write!(f, "Not enough bytes for field `{}`", field)
            }
            ParseError::TooManyBytes(field) => write!(f, "Too many bytes for field `{}`", field),
            ParseError::NotSupported(field) => {
                write!(f, "Feature `{}` is not supported yet", field)
            }
            ParseError::InvalidUTF8String(field) => {
                write!(f, "Field `{}` must be a valid UTF-8 string", field)
            }
            ParseError::UnexpectedLayout(field) => {
                write!(f, "Unexpected Wasm value layout for field `{}`", field)
            }
            ParseError::InvalidSection => write!(f, "Invalid section kind"),
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}
