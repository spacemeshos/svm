use crate::receipt::{error::ExecAppError, Log};
use crate::{gas::MaybeGas, State, WasmValue};

/// Runtime transaction execution receipt
#[derive(Debug, PartialEq, Clone)]
pub struct ExecReceipt {
    /// Whether transaction succedded or not.
    pub success: bool,

    /// The execution error in case execution failed.
    pub error: Option<ExecAppError>,

    /// The new app `State` if execution succedded.
    pub new_state: Option<State>,

    /// Returned values.
    pub returns: Option<Vec<WasmValue>>,

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
            error: Some(ExecAppError::OOG),
            new_state: None,
            returns: None,
            gas_used: MaybeGas::new(),
            logs,
        }
    }

    pub fn from_err(error: ExecAppError, logs: Vec<Log>) -> Self {
        Self {
            success: false,
            error: Some(error),
            new_state: None,
            returns: None,
            gas_used: MaybeGas::new(),
            logs,
        }
    }

    /// Returns App's new `State``. Panics if transaction has failed.
    pub fn get_new_state(&self) -> &State {
        self.new_state.as_ref().unwrap()
    }

    /// Returns executed transaction results. Panics if transaction has failed.
    pub fn get_returns(&self) -> &Vec<WasmValue> {
        self.returns.as_ref().unwrap()
    }

    /// Take the Receipt's logged entries out
    pub fn take_logs(&mut self) -> Vec<Log> {
        std::mem::take(&mut self.logs)
    }
}
