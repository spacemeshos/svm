use crate::{error::DeployTemplateError, gas::MaybeGas};

use svm_types::TemplateAddr;

/// Returned Receipt after deploying a Template.
#[derive(Debug, PartialEq, Clone)]
pub struct TemplateReceipt {
    /// whether spawn succedded or not
    pub success: bool,

    /// the error in case spawning failed
    pub error: Option<DeployTemplateError>,

    /// The deployed template `Address`
    pub addr: Option<TemplateAddr>,

    /// The amount of gas used for template deployment
    pub gas_used: MaybeGas,
}

impl TemplateReceipt {
    /// Creates a new `TemplateReceipt` struct.
    pub fn new(addr: TemplateAddr, gas_used: MaybeGas) -> Self {
        Self {
            success: true,
            error: None,
            addr: Some(addr),
            gas_used,
        }
    }

    /// Creates a `TemplateReceipt` for reaching reaching `Out-of-Gas`.
    pub fn new_oog() -> Self {
        Self {
            success: false,
            error: Some(DeployTemplateError::OOG),
            addr: None,
            gas_used: MaybeGas::new(),
        }
    }

    /// Returns the deployed template address. Panics if deploy has failed.
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
            gas_used: MaybeGas::new(),
        }
    }
}
