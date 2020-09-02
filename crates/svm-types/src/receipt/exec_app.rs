use crate::receipt::{Log, ReceiptError};
use crate::{gas::MaybeGas, State, WasmValue};

/// Runtime transaction execution receipt
#[derive(Debug, PartialEq, Clone)]
pub struct ExecReceipt {
    /// Whether transaction succedded or not.
    pub success: bool,

    /// The execution error in case execution failed.
    pub error: Option<ReceiptError>,

    /// The new app `State` if execution succedded.
    pub new_state: Option<State>,

    /// Returned the data
    pub returndata: Option<Vec<u8>>,

    /// The amount of gas used.
    pub gas_used: MaybeGas,

    /// logged entries during execution of app's transaction
    pub logs: Vec<Log>,
}

impl ExecReceipt {
    /// Creates a `ExecReceipt` for reaching reaching `Out-of-Gas`.
    pub fn new_oog(logs: Vec<Log>) -> Self {
        Self {
            success: false,
            error: Some(ReceiptError::OOG),
            new_state: None,
            returndata: None,
            gas_used: MaybeGas::new(),
            logs,
        }
    }

    pub fn from_err(error: ReceiptError, logs: Vec<Log>) -> Self {
        Self {
            success: false,
            error: Some(error),
            new_state: None,
            returndata: None,
            gas_used: MaybeGas::new(),
            logs,
        }
    }

    /// Returns App's new `State``. Panics if transaction has failed.
    pub fn get_new_state(&self) -> &State {
        self.new_state.as_ref().unwrap()
    }

    /// Returns executed transaction results. Panics if transaction has failed.
    pub fn get_returndata(&self) -> &Vec<u8> {
        self.returndata.as_ref().unwrap()
    }

    pub fn get_error(&self) -> &ReceiptError {
        self.error.as_ref().unwrap()
    }

    pub fn get_logs(&self) -> &[Log] {
        &self.logs
    }

    /// Take the Receipt's logged entries out
    pub fn take_logs(&mut self) -> Vec<Log> {
        std::mem::take(&mut self.logs)
    }
}
