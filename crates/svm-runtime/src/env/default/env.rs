use crate::env::default::DefaultSerializerTypes as Ser;
use crate::env::memory::{MemAppStore, MemTemplateStore, MemoryEnv};
use crate::env::traits::EnvSerializerTypes;

/// `MemAppStore` with default serialization.
pub type DefaultMemAppStore = MemAppStore<
    <Ser as EnvSerializerTypes>::AppSerializer,
    <Ser as EnvSerializerTypes>::AppDeserializer,
>;

/// `MemTemplateStore` with default serialization.
pub type DefaultMemTemplateStore = MemTemplateStore<
    <Ser as EnvSerializerTypes>::TemplateSerializer,
    <Ser as EnvSerializerTypes>::TemplateDeserializer,
>;

/// `MemoryEnv` with default serialization.
pub type DefaultMemoryEnv = MemoryEnv<Ser>;
