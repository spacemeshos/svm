use svm_common::State;

use crate::{error::ExecAppError, value::Value};

/// Runtime transaction execution receipt
#[derive(Debug)]
pub struct Receipt {
    /// whether transaction succedded or not
    pub success: bool,

    /// the execution error in case execution failed
    pub error: Option<ExecAppError>,

    /// the new app `State` if execution succedded
    pub new_state: Option<State>,

    /// returned values
    pub returns: Option<Vec<Value>>,
}

impl Receipt {
    pub fn get_new_state(&self) -> &State {
        self.new_state.as_ref().unwrap()
    }
}

impl From<ExecAppError> for Receipt {
    fn from(error: ExecAppError) -> Self {
        Self {
            success: false,
            error: Some(error),
            new_state: None,
            returns: None,
        }
    }
}
