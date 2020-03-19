use svm_app::types::WasmValue;
use svm_common::State;

use crate::error::ExecAppError;

/// Runtime transaction execution receipt
#[derive(Debug)]
pub struct ExecReceipt {
    /// whether transaction succedded or not
    pub success: bool,

    /// the execution error in case execution failed
    pub error: Option<ExecAppError>,

    /// the new app `State` if execution succedded
    pub new_state: Option<State>,

    /// returned values
    pub returns: Option<Vec<WasmValue>>,

    /// The amount of gas used
    pub gas_used: Option<u64>,
}

impl ExecReceipt {
    /// Returns App's new `State``. Panics if transaction has failed.
    pub fn get_new_state(&self) -> &State {
        self.new_state.as_ref().unwrap()
    }

    /// Returns executed transaction results. Panics if transaction has failed.
    pub fn get_returns(&self) -> &Vec<WasmValue> {
        self.returns.as_ref().unwrap()
    }
}

impl From<ExecAppError> for ExecReceipt {
    fn from(error: ExecAppError) -> Self {
        Self {
            success: false,
            error: Some(error),
            new_state: None,
            returns: None,
            gas_used: None,
        }
    }
}
