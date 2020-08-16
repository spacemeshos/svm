mod deploy_template;
mod exec_app;
mod log;
mod spawn_app;

mod error;
pub use error::ReceiptError;

pub use deploy_template::TemplateReceipt;
pub use exec_app::ExecReceipt;
pub use log::Log;
pub use spawn_app::{make_spawn_app_receipt, SpawnAppReceipt};

use crate::{gas::MaybeGas, WasmValue};

/// Borrowed Receipt
pub enum Receipt<'a> {
    /// Borrows a `TemplateReceipt`.
    DeployTemplate(&'a TemplateReceipt),

    /// Borrows a `SpawnAppReceipt`.
    SpawnApp(&'a SpawnAppReceipt),

    /// Borrows a `ExecReceipt`.
    ExecApp(&'a ExecReceipt),
}

impl<'a> Receipt<'a> {
    /// Returns whether the transaction succeeded.
    pub fn is_success(&self) -> bool {
        match self {
            Self::DeployTemplate(r) => r.success,
            Self::SpawnApp(r) => r.success,
            Self::ExecApp(r) => r.success,
        }
    }

    /// Returns the executed transaction results.
    pub fn get_returns(&self) -> &Vec<u8> {
        match self {
            Self::DeployTemplate(..) => unreachable!(),
            Self::SpawnApp(r) => r.get_returns(),
            Self::ExecApp(r) => r.get_returns(),
        }
    }

    /// Returns the gas used for the transaction.
    pub fn get_gas_used(&self) -> MaybeGas {
        match self {
            Self::DeployTemplate(r) => r.gas_used,
            Self::SpawnApp(r) => r.gas_used,
            Self::ExecApp(r) => r.gas_used,
        }
    }

    /// Returns a `ReceiptError`
    pub fn get_error(&self) -> &ReceiptError {
        match self {
            Self::DeployTemplate(r) => r.error.as_ref().unwrap(),
            Self::SpawnApp(r) => r.error.as_ref().unwrap(),
            Self::ExecApp(r) => r.error.as_ref().unwrap(),
        }
    }
}

/// Owned Receipt
#[derive(Debug, PartialEq)]
pub enum ReceiptOwned {
    DeployTemplate(TemplateReceipt),

    SpawnApp(SpawnAppReceipt),

    ExecApp(ExecReceipt),
}

impl ReceiptOwned {
    pub fn success(&self) -> bool {
        match self {
            ReceiptOwned::DeployTemplate(receipt) => receipt.success,
            ReceiptOwned::SpawnApp(receipt) => receipt.success,
            ReceiptOwned::ExecApp(receipt) => receipt.success,
        }
    }

    pub fn into_deploy_template(self) -> TemplateReceipt {
        match self {
            ReceiptOwned::DeployTemplate(r) => r,
            _ => unreachable!(),
        }
    }

    pub fn into_spawn_app(self) -> SpawnAppReceipt {
        match self {
            ReceiptOwned::SpawnApp(r) => r,
            _ => unreachable!(),
        }
    }

    pub fn into_exec_app(self) -> ExecReceipt {
        match self {
            ReceiptOwned::ExecApp(r) => r,
            _ => unreachable!(),
        }
    }

    pub fn get_logs(&self) -> &[Log] {
        match self {
            ReceiptOwned::DeployTemplate(receipt) => receipt.get_logs(),
            ReceiptOwned::SpawnApp(receipt) => receipt.get_logs(),
            ReceiptOwned::ExecApp(receipt) => receipt.get_logs(),
        }
    }

    pub fn get_error(&self) -> &ReceiptError {
        match self {
            ReceiptOwned::DeployTemplate(receipt) => receipt.get_error(),
            ReceiptOwned::SpawnApp(receipt) => receipt.get_error(),
            ReceiptOwned::ExecApp(receipt) => receipt.get_error(),
        }
    }
}
