use crate::{FuncIndex, Gas};

/// Represents a logic that will give gas estimation for SVM vmcalls
pub trait VMCallsGasEstimator {
    /// Receives vmcall function index and returns its gas estimation
    fn estimate_code(func_idx: FuncIndex) -> Gas;
}
