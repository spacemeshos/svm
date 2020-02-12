use std::fmt;

use crate::raw::Field;

#[allow(missing_docs)]
#[derive(PartialEq, Clone)]
pub enum ParseError {
    InvalidWasm,
    InvalidFuncArgLayout(u8),
    EmptyField(Field),
    EmptyList(Field),
    NotEnoughBytes(Field),
    TooManyBytes(Field),
    NotSupported(Field),
    InvalidUTF8String(Field),
    InvalidProtocolVersion(u32),
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::EmptyField(f) => write!(fmt, "Field `{}` must not be empty", f),
            ParseError::NotEnoughBytes(f) => write!(fmt, "Not enough bytes for field `{}`", f),
            ParseError::TooManyBytes(f) => write!(fmt, "Too many bytes for field `{}`", f),
            ParseError::InvalidProtocolVersion(msg) => write!(fmt, "{}", msg),
            ParseError::NotSupported(f) => write!(fmt, "Feature `{}` is not supported yet", f),
            ParseError::EmptyList(f) => {
                write!(fmt, "`{}`-(s) list must contain at least one item", f)
            }
            ParseError::InvalidUTF8String(f) => {
                write!(fmt, "Field `{}` must be a valid UTF-8 string", f)
            }
            ParseError::InvalidWasm => write!(fmt, "Invalid wasm format"),
            ParseError::InvalidFuncArgLayout(..) => write!(fmt, "Invalid arg type"),
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}
