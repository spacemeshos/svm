mod deploy_template;
mod exec_app;
mod spawn_app;

use crate::gas::MaybeGas;

use svm_app::types::WasmValue;

pub use deploy_template::TemplateReceipt;
pub use exec_app::ExecReceipt;
pub use spawn_app::{make_spawn_app_receipt, SpawnAppReceipt};

/// Borrowed Receipt
pub enum Receipt<'a> {
    /// Borrow for a `TemplateReceipt`.
    DeployTemplate(&'a TemplateReceipt),

    /// Borrow for a `SpawnAppReceipt`.
    SpawnApp(&'a SpawnAppReceipt),

    /// Borrow for a `ExecReceipt`.
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
    pub fn get_returns(&self) -> &Vec<WasmValue> {
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

    /// Returns a failed transaction error as a `String`.
    pub fn error_string(&self) -> String {
        match self {
            Self::DeployTemplate(r) => r.error.as_ref().unwrap().to_string(),
            Self::SpawnApp(r) => r.error.as_ref().unwrap().to_string(),
            Self::ExecApp(r) => r.error.as_ref().unwrap().to_string(),
        }
    }
}
