use svm_app::error::{ParseError, StoreError};

/// Signifies deploy-template failure
#[derive(Debug, PartialEq, Clone)]
pub enum DeployTemplateError {
    /// Parsing raw data has failed (invalid format).
    ParseFailed(ParseError),

    /// Storing the template has failed (operating-system returned a failure).
    StoreFailed(StoreError),
}
