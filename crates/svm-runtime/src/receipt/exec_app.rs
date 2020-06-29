use svm_common::State;
use svm_types::WasmValue;

use crate::{error::ExecAppError, gas::MaybeGas};

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
}

impl ExecReceipt {
    /// Creates a `ExecReceipt` for reaching reaching `Out-of-Gas`.
    pub fn new_oog() -> Self {
        Self {
            success: false,
            error: Some(ExecAppError::OOG),
            new_state: None,
            returns: None,
            gas_used: MaybeGas::new(),
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
}

impl From<ExecAppError> for ExecReceipt {
    fn from(error: ExecAppError) -> Self {
        Self {
            success: false,
            error: Some(error),
            new_state: None,
            returns: None,
            gas_used: MaybeGas::new(),
        }
    }
}
