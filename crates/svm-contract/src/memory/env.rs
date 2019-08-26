use crate::default::{DefaultCodeHasher, DefaultContractAddressCompute};
use crate::env::{ContractEnv, ContractEnvTypes};
use crate::memory::MemContractStore;
use crate::traits::{
    ContractAddressCompute, ContractCodeHasher, ContractDeserializer, ContractSerializer,
    ContractStore,
};
use crate::wasm::{WasmContractJsonDeserializer, WasmContractJsonSerializer};

pub struct MemoryEnvTypes {}

impl ContractEnvTypes for MemoryEnvTypes {
    type Serializer = WasmContractJsonSerializer;

    type Deserializer = WasmContractJsonDeserializer;

    type Store = MemContractStore<Self::Serializer, Self::Deserializer>;

    type AddressCompute = DefaultContractAddressCompute;

    type CodeHasher = DefaultCodeHasher;
}

pub struct MemoryEnv {
    store: <MemoryEnvTypes as ContractEnvTypes>::Store,
}

impl MemoryEnv {
    pub fn new(store: <MemoryEnvTypes as ContractEnvTypes>::Store) -> Self {
        Self { store }
    }
}

impl ContractEnv for MemoryEnv {
    type Types = MemoryEnvTypes;

    fn get_store(&self) -> &<Self::Types as ContractEnvTypes>::Store {
        &self.store
    }

    fn get_store_mut(&mut self) -> &mut <Self::Types as ContractEnvTypes>::Store {
        &mut self.store
    }

    fn close_store(&mut self) {
        self.store.close()
    }
}
