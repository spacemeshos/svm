use crate::default::DefaultContractAddressCompute;
use crate::env::{ContractEnv, ContractEnvTypes};
use crate::memory::MemContractStore;
use crate::traits::ContractAddressCompute;

pub struct MemoryEnvTypes {}

impl ContractEnvTypes for MemoryEnvTypes {
    type Store = MemContractStore;

    type AddressCompute = DefaultContractAddressCompute;
}

pub struct MemoryEnv {}

impl ContractEnv for MemoryEnv {
    type Types = MemoryEnvTypes;

    fn init_store<F: Fn() -> <Self::Types as ContractEnvTypes>::Store>(&mut self, _f: F) {
        unimplemented!()
    }

    fn get_store(&mut self) -> &<Self::Types as ContractEnvTypes>::Store {
        unimplemented!()
    }

    fn get_store_mut(&mut self) -> &mut <Self::Types as ContractEnvTypes>::Store {
        unimplemented!()
    }

    fn open_store(&mut self) {
        unimplemented!()
    }

    fn close_store(&mut self) {
        unimplemented!()
    }
}
