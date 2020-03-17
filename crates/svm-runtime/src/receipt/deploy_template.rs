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

    /// The amount of gas used for template deployment
    pub gas_used: Option<u64>,
}

impl TemplateReceipt {
    pub fn new(addr: TemplateAddr, gas_used: u64) -> Self {
        Self {
            success: true,
            error: None,
            addr: Some(addr),
            gas_used: Some(gas_used),
        }
    }

    pub fn get_template_addr(&self) -> &TemplateAddr {
        self.addr.as_ref().unwrap()
    }
}

impl From<DeployTemplateError> for TemplateReceipt {
    fn from(error: DeployTemplateError) -> Self {
        Self {
            success: false,
            error: Some(error),
            addr: None,
            gas_used: None,
        }
    }
}
