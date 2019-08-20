use crate::traits::ContractAddressCompute;
use crate::wasm::WasmContract;
use svm_common::Address;

#[allow(dead_code)]
pub struct DefaultContractAddressCompute;

impl ContractAddressCompute for DefaultContractAddressCompute {
    fn compute(_contract: &WasmContract) -> Address {
        Address([0; 32])
    }
}
