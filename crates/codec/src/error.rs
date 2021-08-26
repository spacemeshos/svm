use thiserror::Error;

use crate::Field;

/// A type alias for [`Result<..., ParseError>`](std::result::Result).
pub type Result<T> = std::result::Result<T, ParseError>;

#[allow(missing_docs)]
#[derive(Debug, Clone, Error, PartialEq)]
pub enum ParseError {
    #[error("Reached EOF")]
    Eof(String),

    #[error("Field `{0}` must be a valid UTF-8 string")]
    InvalidUTF8String(String),

    #[error("Unexpected Wasm value layout for field `{0}`")]
    UnexpectedLayout(Field),

    #[error("Invalid section kind")]
    InvalidSection,

    #[error("Generic error")]
    Other,
}
