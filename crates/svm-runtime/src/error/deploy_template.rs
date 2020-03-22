use svm_app::error::StoreError;

/// Signifies deploy-template failure
#[derive(Debug, PartialEq, Clone)]
pub enum DeployTemplateError {
    /// Storing the template has failed (operating-system returned a failure).
    StoreFailed(StoreError),
}

impl ToString for DeployTemplateError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
