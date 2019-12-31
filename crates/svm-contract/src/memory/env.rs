use crate::{
    default::{DefaultAppTemplateAddressCompute, DefaultCodeHasher},
    env::{AppTemplateEnv, AppTemplateEnvTypes},
    memory::MemAppTemplateStore,
    wasm::{AppTemplateJsonDeserializer, AppTemplateJsonSerializer},
};

pub struct MemoryEnvTypes;

impl AppTemplateEnvTypes for MemoryEnvTypes {
    type Serializer = AppTemplateJsonSerializer;

    type Deserializer = AppTemplateJsonDeserializer;

    type Store = MemAppTemplateStore<Self::Serializer, Self::Deserializer>;

    type AddressCompute = DefaultAppTemplateAddressCompute;

    type Hasher = DefaultCodeHasher;
}

/// An in-memory implementation for `AppTemplateEnv`
pub struct MemoryEnv {
    store: <MemoryEnvTypes as AppTemplateEnvTypes>::Store,
}

impl MemoryEnv {
    /// Creates a new in-memory environment.
    pub fn new(store: <MemoryEnvTypes as AppTemplateEnvTypes>::Store) -> Self {
        Self { store }
    }
}

impl AppTemplateEnv for MemoryEnv {
    type Types = MemoryEnvTypes;

    fn get_store(&self) -> &<Self::Types as AppTemplateEnvTypes>::Store {
        &self.store
    }

    fn get_store_mut(&mut self) -> &mut <Self::Types as AppTemplateEnvTypes>::Store {
        &mut self.store
    }
}
