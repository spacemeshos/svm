use svm_app::error::{ParseError, StoreError};
use svm_common::Address;

#[derive(Debug, PartialEq, Clone)]
pub enum DeployTemplateError {
    ParseFailed(ParseError),
    StoreFailed(StoreError),
}
