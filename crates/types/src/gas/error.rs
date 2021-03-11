use std::error::Error;
use std::fmt;

/// Out-of-Gas Error
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OOGError;

impl fmt::Display for OOGError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reached Out-of-Gas")
    }
}

impl Error for OOGError {}
