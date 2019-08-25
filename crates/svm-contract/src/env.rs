use crate::traits::{ContractAddressCompute, ContractStore};
use crate::wasm::WasmContract;

use svm_common::Address;

pub trait ContractEnvTypes {
    type Store: ContractStore;

    type AddressCompute: ContractAddressCompute;
}

pub trait ContractEnv {
    type Types: ContractEnvTypes;

    fn init_store<F: Fn() -> <Self::Types as ContractEnvTypes>::Store>(&mut self, f: F);

    fn get_store(&mut self) -> &<Self::Types as ContractEnvTypes>::Store;

    fn get_store_mut(&mut self) -> &mut <Self::Types as ContractEnvTypes>::Store;

    fn open_store(&mut self);

    fn close_store(&mut self);

    #[inline(always)]
    fn compute_address(contract: &WasmContract) -> Address {
        <Self::Types as ContractEnvTypes>::AddressCompute::compute(contract)
    }
}
