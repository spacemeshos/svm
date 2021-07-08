use crate::gas::Gas;
use crate::receipt::{ReceiptLog, RuntimeError};

use crate::TemplateAddr;

/// Returned Receipt after deploying a Template.
#[derive(Debug, PartialEq, Clone)]
pub struct TemplateReceipt {
    /// Transaction format version
    pub version: u16,

    /// whether spawn succedded or not
    pub success: bool,

    /// the error in case spawning failed
    pub error: Option<RuntimeError>,

    /// The deployed template `Address`
    pub addr: Option<TemplateAddr>,

    /// The amount of gas used for template deployment
    pub gas_used: Gas,

    /// generated logs during transaction execution.
    pub logs: Vec<ReceiptLog>,
}

impl TemplateReceipt {
    /// Creates a new `TemplateReceipt` struct.
    pub fn new(addr: TemplateAddr, gas_used: Gas) -> Self {
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
        Self::from_err(RuntimeError::OOG, Vec::new())
    }

    /// Creates a new failure Receipt out of the `error` parameter
    pub fn from_err(error: RuntimeError, logs: Vec<ReceiptLog>) -> Self {
        Self {
            version: 0,
            success: false,
            error: Some(error),
            addr: None,
            gas_used: Gas::new(),
            logs,
        }
    }

    /// Returns the deployed template address. Panics if deploy has failed.
    pub fn template_addr(&self) -> &TemplateAddr {
        self.addr.as_ref().unwrap()
    }

    /// Returns the error within the Receipt (for failing Receipts)
    pub fn error(&self) -> &RuntimeError {
        self.error.as_ref().unwrap()
    }

    /// Returns the logs generated during the transaction execution
    pub fn logs(&self) -> &[ReceiptLog] {
        &self.logs
    }

    /// Taking the `logs` out
    pub fn take_logs(&mut self) -> Vec<ReceiptLog> {
        std::mem::take(&mut self.logs)
    }
}
