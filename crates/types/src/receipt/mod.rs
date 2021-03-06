mod deploy_template;
mod exec_app;
mod log;
mod spawn_app;

pub use deploy_template::TemplateReceipt;
pub use exec_app::ExecReceipt;
pub use log::ReceiptLog;
pub use spawn_app::{into_spawn_app_receipt, SpawnAppReceipt};

use crate::gas::Gas;
use crate::RuntimeError;

/// Borrowed Receipt
pub enum ReceiptRef<'a> {
    /// Borrows a `TemplateReceipt`.
    DeployTemplate(&'a TemplateReceipt),

    /// Borrows a `SpawnAppReceipt`.
    SpawnApp(&'a SpawnAppReceipt),

    /// Borrows a `ExecReceipt`.
    ExecApp(&'a ExecReceipt),
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
    SpawnApp(SpawnAppReceipt),

    /// Exec-App
    ExecApp(ExecReceipt),
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
    pub fn into_spawn_app(self) -> SpawnAppReceipt {
        match self {
            Receipt::SpawnApp(r) => r,
            _ => unreachable!(),
        }
    }

    /// Returns the inner `exec-app` receipt
    pub fn into_exec_app(self) -> ExecReceipt {
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
