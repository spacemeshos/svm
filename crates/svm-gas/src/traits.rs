use crate::function::FuncIndex;
use crate::gas::Gas;

/// Represents logicc that will give gas estimation for SVM vmcalls
pub trait VMCallsGasEstimator {
    /// Receives vmcall function index and returns its gas estimation
    fn estimate_gas(func_idx: FuncIndex) -> Gas;
}
