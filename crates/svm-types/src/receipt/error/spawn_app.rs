use crate::receipt::error::ExecAppError;
use crate::TemplateAddr;

/// Spawning a new app has failed
#[derive(Debug, PartialEq, Clone)]
pub enum SpawnAppError {
    /// Reached Out-of-Gas
    OOG,

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
