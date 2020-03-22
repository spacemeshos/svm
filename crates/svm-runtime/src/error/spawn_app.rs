use svm_app::{error::StoreError, types::TemplateAddr};

use crate::error::ExecAppError;

/// Spawning a new app has failed
#[derive(Debug, PartialEq, Clone)]
pub enum SpawnAppError {
    /// Template not found. Returns the template address.
    TemplateNotFound(TemplateAddr),

    /// Spawned app ctor has failed.
    CtorFailed(ExecAppError),

    /// Storing the template has failed (operating-system returned a failure).
    StoreFailed(StoreError),
}

impl ToString for SpawnAppError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
