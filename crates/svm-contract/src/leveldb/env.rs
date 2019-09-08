use crate::default::{DefaultCodeHasher, DefaultContractAddressCompute};
use crate::env::{ContractEnv, ContractEnvTypes};
use crate::leveldb::LDBContractStore;
use crate::traits::{
    ContractAddressCompute, ContractCodeHasher, ContractDeserializer, ContractSerializer,
    ContractStore,
};
use crate::wasm::{WasmContractJsonDeserializer, WasmContractJsonSerializer};

pub struct LDBEnvTypes {}

impl ContractEnvTypes for LDBEnvTypes {
    type Serializer = WasmContractJsonSerializer;

    type Deserializer = WasmContractJsonDeserializer;

    type Store = LDBContractStore<Self::Serializer, Self::Deserializer>;

    type AddressCompute = DefaultContractAddressCompute;

    type CodeHasher = DefaultCodeHasher;
}

pub struct LDBEnv {
    store: <LDBEnvTypes as ContractEnvTypes>::Store,
}

impl LDBEnv {
    pub fn new(store: <LDBEnvTypes as ContractEnvTypes>::Store) -> Self {
        dbg!("creating a new `LDBEnv` environment.");

        Self { store }
    }
}

impl ContractEnv for LDBEnv {
    type Types = LDBEnvTypes;

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

impl Drop for LDBEnv {
    fn drop(&mut self) {
        dbg!("dropping `LDBEnv`");

        self.close_store();
    }
}
