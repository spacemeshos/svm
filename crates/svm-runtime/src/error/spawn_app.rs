use svm_app::error::{ParseError, StoreError};
use svm_common::Address;

#[derive(Debug, PartialEq, Clone)]
pub enum SpawnAppError {
    ParseFailed(ParseError),
    StoreFailed(StoreError),
    TemplateNotFound(Address),
}
