use crate::gas::MaybeGas;
use crate::receipt::{Log, ReceiptError};

use crate::TemplateAddr;

/// Returned Receipt after deploying a Template.
#[derive(Debug, PartialEq, Clone)]
pub struct TemplateReceipt {
    pub version: u16,

    /// whether spawn succedded or not
    pub success: bool,

    /// the error in case spawning failed
    pub error: Option<ReceiptError>,

    /// The deployed template `Address`
    pub addr: Option<TemplateAddr>,

    /// The amount of gas used for template deployment
    pub gas_used: MaybeGas,

    /// generated logs during transaction execution.
    pub logs: Vec<Log>,
}

impl TemplateReceipt {
    /// Creates a new `TemplateReceipt` struct.
    pub fn new(addr: TemplateAddr, gas_used: MaybeGas) -> Self {
        Self {
            version: 0,
            success: true,
            error: None,
            addr: Some(addr),
            gas_used,
            logs: Vec::new(),
        }
    }

    /// Creates a `TemplateReceipt` for reaching reaching `Out-of-Gas`.
    pub fn new_oog() -> Self {
        Self::from_err(ReceiptError::OOG, Vec::new())
    }

    pub fn from_err(error: ReceiptError, logs: Vec<Log>) -> Self {
        Self {
            version: 0,
            success: false,
            error: Some(error),
            addr: None,
            gas_used: MaybeGas::new(),
            logs,
        }
    }

    /// Returns the deployed template address. Panics if deploy has failed.
    pub fn get_template_addr(&self) -> &TemplateAddr {
        self.addr.as_ref().unwrap()
    }

    pub fn get_error(&self) -> &ReceiptError {
        self.error.as_ref().unwrap()
    }

    pub fn get_logs(&self) -> &[Log] {
        &self.logs
    }

    pub fn take_logs(&mut self) -> Vec<Log> {
        std::mem::take(&mut self.logs)
    }
}
