use crate::default::{DefaultCodeHasher, DefaultContractAddressCompute};
use crate::env::{ContractEnv, ContractEnvTypes};
use crate::rocksdb::RocksContractStore;
use crate::wasm::{WasmContractJsonDeserializer, WasmContractJsonSerializer};

pub struct RocksEnvTypes {}

impl ContractEnvTypes for RocksEnvTypes {
    type Serializer = WasmContractJsonSerializer;

    type Deserializer = WasmContractJsonDeserializer;

    type Store = RocksContractStore<Self::Serializer, Self::Deserializer>;

    type AddressCompute = DefaultContractAddressCompute;

    type CodeHasher = DefaultCodeHasher;
}

/// Contract environment backed by `rocksdb` for persistence.
pub struct RocksEnv {
    store: <RocksEnvTypes as ContractEnvTypes>::Store,
}

impl RocksEnv {
    /// Creates a new `RocksEnv`. Injects externally the `ContractStore`
    pub fn new(store: <RocksEnvTypes as ContractEnvTypes>::Store) -> Self {
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
}
