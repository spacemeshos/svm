use crate::{error::ExecAppError, value::Value};

use svm_app::types::AppTransaction;
use svm_common::State;

/// Runtime transaction execution receipt
#[derive(Debug)]
pub struct Receipt {
    /// whether transaction succedded or not
    pub success: bool,

    /// the execution error in case execution failed
    pub error: Option<ExecAppError>,

    /// executed `AppTransaction
    pub tx: AppTransaction,

    /// the new app `State` if execution succedded
    pub new_state: Option<State>,

    /// returned values
    pub results: Vec<Value>,
}
