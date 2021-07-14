mod deploy_template;
mod execute;
mod log;
mod spawn;

pub use deploy_template::TemplateReceipt;
pub use execute::CallReceipt;
pub use log::ReceiptLog;
pub use spawn::{into_spawn_receipt, SpawnReceipt};

use crate::gas::Gas;
use crate::RuntimeError;

/// Borrowed Receipt
pub enum ReceiptRef<'a> {
    /// Borrows a `TemplateReceipt`.
    DeployTemplate(&'a TemplateReceipt),

    /// Borrows a `SpawnAppReceipt`.
    SpawnApp(&'a SpawnReceipt),

    /// Borrows a `ExecReceipt`.
    ExecApp(&'a CallReceipt),
}

impl<'a> ReceiptRef<'a> {
    /// Returns whether the transaction succeeded.
    pub fn is_success(&self) -> bool {
        match self {
            Self::DeployTemplate(r) => r.success,
            Self::SpawnApp(r) => r.success,
            Self::ExecApp(r) => r.success,
        }
    }

    /// Returns the executed transaction results.
    pub fn returndata(&self) -> &Vec<u8> {
        match self {
            Self::DeployTemplate(..) => unreachable!(),
            Self::SpawnApp(r) => r.returndata(),
            Self::ExecApp(r) => r.returndata(),
        }
    }

    /// Returns the gas used for the transaction.
    pub fn get_gas_used(&self) -> Gas {
        match self {
            Self::DeployTemplate(r) => r.gas_used,
            Self::SpawnApp(r) => r.gas_used,
            Self::ExecApp(r) => r.gas_used,
        }
    }

    /// Returns a `ReceiptError`
    pub fn error(&self) -> &RuntimeError {
        match self {
            Self::DeployTemplate(r) => r.error.as_ref().unwrap(),
            Self::SpawnApp(r) => r.error.as_ref().unwrap(),
            Self::ExecApp(r) => r.error.as_ref().unwrap(),
        }
    }
}

/// Holds some Receipt-type
#[derive(Debug, PartialEq)]
pub enum Receipt {
    /// Deploy-Template
    DeployTemplate(TemplateReceipt),

    /// Spawn-App
    SpawnApp(SpawnReceipt),

    /// Exec-App
    ExecApp(CallReceipt),
}

impl Receipt {
    /// Returns whether the transaction succedded.
    /// A transaction counts as a `success` when it didn't panic.
    pub fn success(&self) -> bool {
        match self {
            Receipt::DeployTemplate(receipt) => receipt.success,
            Receipt::SpawnApp(receipt) => receipt.success,
            Receipt::ExecApp(receipt) => receipt.success,
        }
    }

    /// Returns the inner `deploy-template` receipt
    pub fn into_deploy_template(self) -> TemplateReceipt {
        match self {
            Receipt::DeployTemplate(r) => r,
            _ => unreachable!(),
        }
    }

    /// Returns the inner `spawn-app` receipt
    pub fn into_spawn_app(self) -> SpawnReceipt {
        match self {
            Receipt::SpawnApp(r) => r,
            _ => unreachable!(),
        }
    }

    /// Returns the inner `exec-app` receipt
    pub fn into_exec_app(self) -> CallReceipt {
        match self {
            Receipt::ExecApp(r) => r,
            _ => unreachable!(),
        }
    }

    /// Returns the logs generated during the transaction execution
    pub fn logs(&self) -> &[ReceiptLog] {
        match self {
            Receipt::DeployTemplate(receipt) => receipt.logs(),
            Receipt::SpawnApp(receipt) => receipt.logs(),
            Receipt::ExecApp(receipt) => receipt.logs(),
        }
    }

    /// Returns the error within the inner receipt (for failing receipts)
    pub fn error(&self) -> &RuntimeError {
        match self {
            Receipt::DeployTemplate(receipt) => receipt.error(),
            Receipt::SpawnApp(receipt) => receipt.error(),
            Receipt::ExecApp(receipt) => receipt.error(),
        }
    }
}
