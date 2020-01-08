use std::fmt;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum StoreError {
    OsFailure(String),
    DataCorruption(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
