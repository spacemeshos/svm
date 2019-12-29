use crate::{runtime::ContractExecError, value::Value};

use svm_common::State;
use svm_contract::transaction::Transaction;

/// Runtime transaction execution receipt
#[derive(Debug)]
pub struct Receipt {
    /// whether transaction succedded or not
    pub success: bool,

    /// the execution error in case execution failed
    pub error: Option<ContractExecError>,

    /// executed transaction
    pub tx: Transaction,

    /// the new contract `State` if execution succedded
    pub new_state: Option<State>,

    /// returned values
    pub results: Vec<Value>,
}
