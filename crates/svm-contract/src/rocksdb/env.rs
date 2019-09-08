use crate::default::{DefaultCodeHasher, DefaultContractAddressCompute};
use crate::env::{ContractEnv, ContractEnvTypes};
use crate::rocksdb::RocksContractStore;
use crate::traits::{
    ContractAddressCompute, ContractCodeHasher, ContractDeserializer, ContractSerializer,
    ContractStore,
};
use crate::wasm::{WasmContractJsonDeserializer, WasmContractJsonSerializer};

pub struct RocksEnvTypes {}

impl ContractEnvTypes for RocksEnvTypes {
    type Serializer = WasmContractJsonSerializer;

    type Deserializer = WasmContractJsonDeserializer;

    type Store = RocksContractStore<Self::Serializer, Self::Deserializer>;

    type AddressCompute = DefaultContractAddressCompute;

    type CodeHasher = DefaultCodeHasher;
}

pub struct RocksEnv {
    store: <RocksEnvTypes as ContractEnvTypes>::Store,
}

impl RocksEnv {
    pub fn new(store: <RocksEnvTypes as ContractEnvTypes>::Store) -> Self {
        dbg!("creating a new `RocksEnv` environment.");

        Self { store }
    }
}

impl ContractEnv for RocksEnv {
    type Types = RocksEnvTypes;

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

impl Drop for RocksEnv {
    fn drop(&mut self) {
        dbg!("dropping `RocksEnv`");

        self.close_store();
    }
}
