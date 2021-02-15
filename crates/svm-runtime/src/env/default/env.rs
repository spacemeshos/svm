use crate::env::default::DefaultSerializerTypes as Ser;
use crate::env::memory::{MemAppStore, MemTemplateStore, MemoryEnv};
use crate::env::traits::EnvSerializerTypes;

/// `MemAppStore` with default serialization.
pub type DefaultMemAppStore = MemAppStore<
    // `AppStore` Serializer
    <Ser as EnvSerializerTypes>::AppSerializer,
    // `AppStore` Deserializer
    <Ser as EnvSerializerTypes>::AppDeserializer,
>;

/// `MemTemplateStore` with default serialization.
pub type DefaultMemTemplateStore = MemTemplateStore<
    <Ser as EnvSerializerTypes>::TemplateSerializer,
    <Ser as EnvSerializerTypes>::TemplateDeserializer,
>;

/// `MemoryEnv` with default serialization.
pub type DefaultMemoryEnv = MemoryEnv<Ser>;
