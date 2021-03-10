use crate::env::{default, rocksdb, traits};

use rocksdb::{RocksAppStore, RocksTemplateStore};
use traits::EnvTypes;

/// `MemTemplateStore` with default serialization.
pub type DefaultRocksTemplateStore =
    RocksTemplateStore<default::DefaultTemplateSerializer, default::DefaultTemplateDeserializer>;

/// `MemAppStore` with default serialization.
pub type DefaultRocksAppStore =
    RocksAppStore<default::DefaultAppSerializer, default::DefaultAppDeserializer>;

pub struct DefaultRocksEnvTypes;

impl EnvTypes for DefaultRocksEnvTypes {
    type TemplateStore = DefaultRocksTemplateStore;

    type AppStore = DefaultRocksAppStore;

    type TemplateAddressCompute = default::DefaultTemplateAddressCompute;

    type AppAddressCompute = default::DefaultAppAddressCompute;

    type TemplateHasher = default::DefaultTemplateHasher;
}
