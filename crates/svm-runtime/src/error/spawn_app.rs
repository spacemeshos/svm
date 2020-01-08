use svm_app::error::{ParseError, StoreError};
use svm_common::Address;

/// Spawning a new app has failed
#[derive(Debug, PartialEq, Clone)]
pub enum SpawnAppError {
    /// Parsing raw data has failed (invalid format).
    ParseFailed(ParseError),

    /// Storing the template has failed (operating-system returned a failure).
    StoreFailed(StoreError),

    /// Template not found. Returns the template address.
    TemplateNotFound(Address),
}
