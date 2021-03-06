use crate::env::{default, memory, traits};

use default::DefaultSerializers as S;
use memory::{MemAppStore, MemTemplateStore, MemoryEnv};
use traits::EnvSerializers;

/// `MemoryEnv` with default serialization.
pub type DefaultMemoryEnv = MemoryEnv<S>;

/// `MemTemplateStore` with default serialization.
pub type DefaultMemTemplateStore = MemTemplateStore<
    <S as EnvSerializers>::TemplateSerializer,
    <S as EnvSerializers>::TemplateDeserializer,
>;

/// `MemAppStore` with default serialization.
pub type DefaultMemAppStore = MemAppStore<
    // `AppStore` Serializer
    <S as EnvSerializers>::AppSerializer,
    // `AppStore` Deserializer
    <S as EnvSerializers>::AppDeserializer,
>;
