use std::fmt;

use crate::raw::Field;

#[allow(missing_docs)]
#[derive(PartialEq, Clone)]
pub enum ParseError {
    ExpectedEOF,
    EmptyField(Field),
    NotEnoughBytes(Field),
    TooManyBytes(Field),
    NotSupported(Field),
    InvalidUTF8String(Field),
    InvalidProtocolVersion(u32),
    FuncArgValueIncomplete {
        arg_idx: usize,
        expected_nibbles: usize,
        actual_read: usize,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::ExpectedEOF => write!(fmt, "Expected EOF but there are more left bytes"),
            ParseError::EmptyField(f) => write!(fmt, "Field `{}` must not be empty", f),
            ParseError::NotEnoughBytes(f) => write!(fmt, "Not enough bytes for field `{}`", f),
            ParseError::TooManyBytes(f) => write!(fmt, "Too many bytes for field `{}`", f),
            ParseError::InvalidProtocolVersion(msg) => write!(fmt, "{}", msg),
            ParseError::NotSupported(f) => write!(fmt, "Feature `{}` is not supported yet", f),
            ParseError::InvalidUTF8String(f) => {
                write!(fmt, "Field `{}` must be a valid UTF-8 string", f)
            }
            ParseError::FuncArgValueIncomplete { .. } => {
                write!(fmt, "Function argument is incomplete (missing data)")
            }
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}
