use crate::{
    default::{DefaultAppTemplateAddressCompute, DefaultCodeHasher},
    env::{AppTemplateEnv, AppTemplateEnvTypes},
    rocksdb::RocksdbAppTemplateStore,
    wasm::{AppTemplateJsonDeserializer, AppTemplateJsonSerializer},
};

pub struct RocksdbAppTemplateEnvTypes {}

impl AppTemplateEnvTypes for RocksdbAppTemplateEnvTypes {
    type Serializer = AppTemplateJsonSerializer;

    type Deserializer = AppTemplateJsonDeserializer;

    type Store = RocksdbAppTemplateStore<Self::Serializer, Self::Deserializer>;

    type AddressCompute = DefaultAppTemplateAddressCompute;

    type Hasher = DefaultCodeHasher;
}

/// AppTemplate environment backed-by `rocksdb`
pub struct RocksdbAppTemplateEnv {
    store: <RocksdbAppTemplateEnvTypes as AppTemplateEnvTypes>::Store,
}

impl RocksdbAppTemplateEnv {
    /// Creates a new `RocksdbAppTemplateEnv`. Injects externally the `AppTemplateStore`
    pub fn new(store: <RocksdbAppTemplateEnvTypes as AppTemplateEnvTypes>::Store) -> Self {
        Self { store }
    }
}

impl AppTemplateEnv for RocksdbAppTemplateEnv {
    type Types = RocksdbAppTemplateEnvTypes;

    fn get_store(&self) -> &<Self::Types as AppTemplateEnvTypes>::Store {
        &self.store
    }

    fn get_store_mut(&mut self) -> &mut <Self::Types as AppTemplateEnvTypes>::Store {
        &mut self.store
    }
}
