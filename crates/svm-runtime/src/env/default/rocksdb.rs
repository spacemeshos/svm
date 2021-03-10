use crate::env::{default, rocksdb, traits};

use default::memory::DefaultMemAppStore;
use default::DefaultSerializers as S;
use rocksdb::{RocksAppStore, RocksTemplateStore};
use traits::EnvSerializers;
use traits::EnvTypes;

/// `MemTemplateStore` with default serialization.
pub type DefaultRocksTemplateStore = RocksTemplateStore<
    <S as EnvSerializers>::TemplateSerializer,
    <S as EnvSerializers>::TemplateDeserializer,
>;

/// `MemAppStore` with default serialization.
pub type DefaultRocksAppStore = RocksAppStore<
    // `AppStore` Serializer
    <S as EnvSerializers>::AppSerializer,
    // `AppStore` Deserializer
    <S as EnvSerializers>::AppDeserializer,
>;

pub struct DefaultRocksEnvTypes;

impl EnvTypes for DefaultRocksEnvTypes {
    type TemplateStore = DefaultRocksTemplateStore;

    type AppStore = DefaultRocksAppStore;

    type TemplateAddressCompute = default::DefaultTemplateAddressCompute;

    type AppAddressCompute = default::DefaultAppAddressCompute;

    type TemplateHasher = default::DefaultTemplateHasher;
}
