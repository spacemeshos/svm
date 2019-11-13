use crate::function::FuncIndex;
use crate::gas::Gas;

pub trait VMCallsGasEstimator {
    fn estimate_gas(func_idx: FuncIndex) -> Gas;
}
