use crate::default::{DefaultCodeHasher, DefaultContractAddressCompute};
use crate::env::{ContractEnv, ContractEnvTypes};
use crate::rocksdb::RocksdbContractStore;
use crate::wasm::{WasmContractJsonDeserializer, WasmContractJsonSerializer};

pub struct RocksdbContractEnvTypes {}

impl ContractEnvTypes for RocksdbContractEnvTypes {
    type Serializer = WasmContractJsonSerializer;

    type Deserializer = WasmContractJsonDeserializer;

    type Store = RocksdbContractStore<Self::Serializer, Self::Deserializer>;

    type AddressCompute = DefaultContractAddressCompute;

    type CodeHasher = DefaultCodeHasher;
}

/// Contract environment backed by `rocksdb` for persistence.
pub struct RocksdbEnv {
    store: <RocksdbContractEnvTypes as ContractEnvTypes>::Store,
}

impl RocksdbEnv {
    /// Creates a new `RocksdbEnv`. Injects externally the `ContractStore`
    pub fn new(store: <RocksdbContractEnvTypes as ContractEnvTypes>::Store) -> Self {
        Self { store }
    }
}

impl ContractEnv for RocksdbEnv {
    type Types = RocksdbContractEnvTypes;

    fn get_store(&self) -> &<Self::Types as ContractEnvTypes>::Store {
        &self.store
    }

    fn get_store_mut(&mut self) -> &mut <Self::Types as ContractEnvTypes>::Store {
        &mut self.store
    }
}
