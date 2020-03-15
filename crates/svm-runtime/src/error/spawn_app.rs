use svm_app::{
    error::{ParseError, StoreError},
    types::TemplateAddr,
};

use crate::error::ExecAppError;

/// Spawning a new app has failed
#[derive(Debug, PartialEq, Clone)]
pub enum SpawnAppError {
    /// Parsing raw data has failed (invalid format).
    ParseFailed(ParseError),

    /// Storing the template has failed (operating-system returned a failure).
    StoreFailed(StoreError),

    /// Template not found. Returns the template address.
    TemplateNotFound(TemplateAddr),

    /// Spawned app ctor has failed.
    CtorFailed(ExecAppError),
}

impl ToString for SpawnAppError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
