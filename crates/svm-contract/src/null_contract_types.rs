use crate::code_hash::MemCodeHashStore;
use crate::traits::{CodeHashStore, ContractAddressCompute};
use crate::types::ContractTypes;
use crate::wasm::WasmContract;
use svm_common::Address;

pub struct NullContractAddressCompute;

impl ContractAddressCompute for NullContractAddressCompute {
    fn compute(contract: &WasmContract) -> Address {
        Address([0; 32])
    }
}

pub struct NullContractTypes;

impl ContractTypes for NullContractTypes {
    type Store = MemCodeHashStore;

    type AddressCompute = NullContractAddressCompute;
}
