use crate::env::{default, rocksdb, traits};

use default::DefaultSerializers as S;
use rocksdb::{RocksdbAppStore, RocksdbEnv, RocksdbTemplateStore};
use traits::EnvSerializers;

/// `MemoryEnv` with default serialization.
pub type DefaultRocksdbEnv = RocksdbEnv<S>;

/// `MemTemplateStore` with default serialization.
pub type DefaultRocksdbTemplateStore = RocksdbTemplateStore<
    <S as EnvSerializers>::TemplateSerializer,
    <S as EnvSerializers>::TemplateDeserializer,
>;

/// `MemAppStore` with default serialization.
pub type DefaultRocksdbAppStore = RocksdbAppStore<
    // `AppStore` Serializer
    <S as EnvSerializers>::AppSerializer,
    // `AppStore` Deserializer
    <S as EnvSerializers>::AppDeserializer,
>;
