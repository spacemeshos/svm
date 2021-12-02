use std::collections::HashSet;

use crate::{Address, Gas, State};
use crate::{CallReceipt, ReceiptLog, RuntimeError};

/// Returned Receipt after spawning an [`Account`](crate::Account)
#[derive(Debug, PartialEq, Clone)]
pub struct SpawnReceipt {
    /// The transaction format version
    pub version: u16,

    /// Whether Spawn succeeded or not
    pub success: bool,

    /// The [`RuntimeError`] in case spawning has failed.
    pub error: Option<RuntimeError>,

    /// The spawned `Account Address`
    pub account_addr: Option<Address>,

    /// The spawned [`Account`](crate::Account) initial state (after executing its ctor)
    pub init_state: Option<State>,

    /// Returned `ctor` data
    pub returndata: Option<Vec<u8>>,

    /// The amount of gas used.
    pub gas_used: Gas,

    /// A set of accounts that have changed balance, or *might* have.
    pub touched_accounts: HashSet<Address>,

    /// Logs collected during `Spawning` `ctor` running.
    pub logs: Vec<ReceiptLog>,
}

impl SpawnReceipt {
    /// Creates a [`SpawnReceipt`] for reaching reaching `Out-of-Gas`.
    pub fn new_oog(logs: Vec<ReceiptLog>) -> Self {
        Self::from_err(RuntimeError::OOG, logs)
    }

    /// Creates a new failure Receipt out of the `error` parameter
    pub fn from_err(error: RuntimeError, logs: Vec<ReceiptLog>) -> Self {
        Self {
            version: 0,
            success: false,
            error: Some(error),
            account_addr: None,
            init_state: None,
            returndata: None,
            gas_used: Gas::new(),
            touched_accounts: HashSet::new(),
            logs,
        }
    }

    /// Returns [`RuntimeError`].
    ///
    /// # Panics
    ///
    /// Panics if spawning has **NOT** failed.
    pub fn error(&self) -> &RuntimeError {
        self.error.as_ref().unwrap()
    }

    /// Returns spawned [`Account`](crate::Account) Address.
    ///
    /// # Panics
    ///
    /// Panics if spawning has failed.
    pub fn account_addr(&self) -> &Address {
        self.account_addr.as_ref().unwrap()
    }

    /// Returns spawned [`Account`](crate::Account) initial `State`.
    ///
    /// # Panics
    ///
    /// Panics if spawning has failed.
    pub fn init_state(&self) -> &State {
        self.init_state.as_ref().unwrap()
    }

    /// Returns spawned [`Account`](crate::Account) results. Panics if spawning has failed.
    pub fn returndata(&self) -> &Vec<u8> {
        self.returndata.as_ref().unwrap()
    }

    /// Returns [`Account`](crate::Account) amount of gas-used.
    pub fn gas_used(&self) -> Gas {
        self.gas_used
    }

    /// Returns the collected Logs during the transaction execution.
    pub fn logs(&self) -> &[ReceiptLog] {
        &self.logs
    }

    /// Takes the Receipt's collected logs.
    pub fn take_logs(&mut self) -> Vec<ReceiptLog> {
        std::mem::take(&mut self.logs)
    }
}

#[allow(missing_docs)]
pub fn into_spawn_receipt(mut ctor_receipt: CallReceipt, account_addr: &Address) -> SpawnReceipt {
    let logs = ctor_receipt.take_logs();

    if ctor_receipt.success {
        SpawnReceipt {
            version: 0,
            success: true,
            error: None,
            account_addr: Some(account_addr.clone()),
            init_state: ctor_receipt.new_state,
            returndata: ctor_receipt.returndata,
            gas_used: ctor_receipt.gas_used,
            touched_accounts: ctor_receipt.touched_accounts,
            logs,
        }
    } else {
        let error = ctor_receipt.error.unwrap();

        SpawnReceipt {
            version: 0,
            success: false,
            error: Some(error),
            account_addr: None,
            init_state: None,
            returndata: None,
            gas_used: Gas::new(),
            touched_accounts: ctor_receipt.touched_accounts,
            logs,
        }
    }
}
