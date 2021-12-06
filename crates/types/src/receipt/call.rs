use std::collections::HashSet;

use crate::gas::Gas;
use crate::receipt::{ReceiptLog, RuntimeError};
use crate::{Address, State};

/// Runtime transaction execution receipt
#[derive(Debug, PartialEq, Clone)]
pub struct CallReceipt {
    /// Transaction format version.
    pub version: u16,

    /// Whether transaction succeeded or not.
    pub success: bool,

    /// The execution error in case execution failed.
    pub error: Option<RuntimeError>,

    /// The new [`Account`](crate::Account) `State` if execution succeeded.
    pub new_state: Option<State>,

    /// Returned the data.
    pub returndata: Option<Vec<u8>>,

    /// The amount of gas used.
    pub gas_used: Gas,

    /// A set of accounts that have changed balance, or *might* have.
    pub touched_accounts: HashSet<Address>,

    /// Logs generated during execution of the transaction.
    pub logs: Vec<ReceiptLog>,
}

impl From<RuntimeError> for CallReceipt {
    fn from(err: RuntimeError) -> Self {
        Self::from_err(err, Vec::new())
    }
}

impl CallReceipt {
    /// Creates a `ExecReceipt` for reaching reaching `Out-of-Gas`.
    pub fn new_oog(logs: Vec<ReceiptLog>) -> Self {
        Self::from_err(RuntimeError::OOG, logs)
    }

    /// Creates a new failure Receipt out of the `err` parameter
    pub fn from_err(err: RuntimeError, logs: Vec<ReceiptLog>) -> Self {
        Self {
            version: 0,
            success: false,
            error: Some(err),
            new_state: None,
            returndata: None,
            gas_used: Gas::new(),
            touched_accounts: HashSet::new(),
            logs,
        }
    }

    /// Returns [`Account`](crate::Account)'s new `State``
    ///
    /// # Panics
    ///
    /// Panics if transaction has failed.
    pub fn new_state(&self) -> &State {
        self.new_state.as_ref().unwrap()
    }

    /// Returns executed transaction results.
    ///
    /// # Panics
    ///
    // Panics if transaction has failed.
    pub fn returndata(&self) -> &Vec<u8> {
        self.returndata.as_ref().unwrap()
    }

    /// Returns the error within the Receipt (for failing Receipts)
    pub fn error(&self) -> &RuntimeError {
        self.error.as_ref().unwrap()
    }

    /// Returns the logs generated during the transaction execution
    pub fn logs(&self) -> &[ReceiptLog] {
        &self.logs
    }

    /// Take the Receipt's logged entries out
    pub fn take_logs(&mut self) -> Vec<ReceiptLog> {
        std::mem::take(&mut self.logs)
    }
}
