use thiserror::Error;

use crate::Field;

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParseError {
    #[error("Reached EOF")]
    ReachedEOF,

    #[error("Expected EOF but there are more left bytes")]
    ExpectedEOF,

    #[error("Field `{0}` must not be empty")]
    EmptyField(Field),

    #[error("Not enough bytes for field `{0}`")]
    NotEnoughBytes(Field),

    #[error("Too enough bytes for field `{0}`")]
    TooManyBytes(Field),

    #[error("Field `{0}` is not supported yet")]
    NotSupported(Field),

    #[error("Field `{0}` must be a valid UTF-8 string")]
    InvalidUTF8String(Field),

    #[error("Unexpected Wasm value layout for field `{0}`")]
    UnexpectedLayout(Field),

    #[error("Invalid section kind")]
    InvalidSection,
}
