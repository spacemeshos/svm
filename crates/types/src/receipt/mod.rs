mod deploy_template;
mod execute;
mod log;
mod spawn;

pub use deploy_template::DeployReceipt;
pub use execute::CallReceipt;
pub use log::ReceiptLog;
pub use spawn::{into_spawn_receipt, SpawnReceipt};

use crate::gas::Gas;
use crate::RuntimeError;

/// Borrowed Receipt
pub enum ReceiptRef<'a> {
    /// Borrows a `DeployReceipt`.
    Deploy(&'a DeployReceipt),

    /// Borrows a `SpawnReceipt`.
    Spawn(&'a SpawnReceipt),

    /// Borrows a `CallReceipt`.
    Call(&'a CallReceipt),
}

impl<'a> ReceiptRef<'a> {
    /// Returns whether the transaction succeeded.
    pub fn is_success(&self) -> bool {
        match self {
            Self::Deploy(r) => r.success,
            Self::Spawn(r) => r.success,
            Self::Call(r) => r.success,
        }
    }

    /// Returns the executed transaction results.
    pub fn returndata(&self) -> &Vec<u8> {
        match self {
            Self::Deploy(..) => unreachable!(),
            Self::Spawn(r) => r.returndata(),
            Self::Call(r) => r.returndata(),
        }
    }

    /// Returns the gas used for the transaction.
    pub fn gas_used(&self) -> Gas {
        match self {
            Self::Deploy(r) => r.gas_used,
            Self::Spawn(r) => r.gas_used,
            Self::Call(r) => r.gas_used,
        }
    }

    /// Returns a `ReceiptError`
    pub fn error(&self) -> &RuntimeError {
        match self {
            Self::Deploy(r) => r.error.as_ref().unwrap(),
            Self::Spawn(r) => r.error.as_ref().unwrap(),
            Self::Call(r) => r.error.as_ref().unwrap(),
        }
    }
}

/// Holds a Receipt of kind `Deploy/Spawn/Call`
#[derive(Debug, PartialEq)]
pub enum Receipt {
    /// `Deploy Template`
    Deploy(DeployReceipt),

    /// `Spawn Account`
    Spawn(SpawnReceipt),

    /// `Call Account`
    Call(CallReceipt),
}

impl Receipt {
    /// Returns whether the transaction succeeded.
    /// A transaction counts as a `success` when it didn't panic.
    pub fn success(&self) -> bool {
        match self {
            Receipt::Deploy(receipt) => receipt.success,
            Receipt::Spawn(receipt) => receipt.success,
            Receipt::Call(receipt) => receipt.success,
        }
    }

    /// Returns the inner [`DeployReceipt`]
    pub fn into_deploy(self) -> DeployReceipt {
        match self {
            Receipt::Deploy(r) => r,
            _ => unreachable!(),
        }
    }

    /// Returns the inner [`SpawnReceipt`]
    pub fn into_spawn(self) -> SpawnReceipt {
        match self {
            Receipt::Spawn(r) => r,
            _ => unreachable!(),
        }
    }

    /// Returns the inner [`CallReceipt`]
    pub fn into_call(self) -> CallReceipt {
        match self {
            Receipt::Call(r) => r,
            _ => unreachable!(),
        }
    }

    /// Returns the logs generated during the transaction execution
    pub fn logs(&self) -> &[ReceiptLog] {
        match self {
            Receipt::Deploy(receipt) => receipt.logs(),
            Receipt::Spawn(receipt) => receipt.logs(),
            Receipt::Call(receipt) => receipt.logs(),
        }
    }

    /// Returns the error within the inner receipt (for failing receipts)
    pub fn error(&self) -> &RuntimeError {
        match self {
            Receipt::Deploy(receipt) => receipt.error(),
            Receipt::Spawn(receipt) => receipt.error(),
            Receipt::Call(receipt) => receipt.error(),
        }
    }
}
