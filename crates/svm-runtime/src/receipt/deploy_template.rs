use crate::error::DeployTemplateError;

use svm_app::types::TemplateAddr;

// use crate::error::SpawnAppError;

#[derive(Debug)]
pub struct TemplateReceipt {
    /// whether spawn succedded or not
    pub success: bool,

    /// the error in case spawning failed
    pub error: Option<DeployTemplateError>,

    /// The deployed template `Address`
    pub addr: Option<TemplateAddr>,
}

impl From<TemplateAddr> for TemplateReceipt {
    fn from(addr: TemplateAddr) -> Self {
        Self {
            success: true,
            error: None,
            addr: Some(addr),
        }
    }
}

impl From<DeployTemplateError> for TemplateReceipt {
    fn from(error: DeployTemplateError) -> Self {
        Self {
            success: false,
            error: Some(error),
            addr: None,
        }
    }
}
