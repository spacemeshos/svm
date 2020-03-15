use svm_app::types::TemplateAddr;

// use crate::error::SpawnAppError;

#[derive(Debug)]
pub struct TemplateReceipt {
    /// whether spawn succedded or not
    pub success: bool,

    /// the error in case spawning failed
    // pub error: Option<SpawnAppError>,

    /// The deployed template `Address`
    pub addr: Option<TemplateAddr>,
}
