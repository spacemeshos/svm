use std::marker::PhantomData;

use crate::env::{default, rocksdb, traits};

use default::{DefaultAppAddressCompute, DefaultTemplateAddressCompute, DefaultTemplateHasher};
use rocksdb::{RocksdbAppStore, RocksdbTemplateStore};
use traits::{EnvSerializers, EnvTypes};

/// Aggregates `rocksdb` environment types
pub struct RocksdbEnvTypes<S>(PhantomData<S>);

impl<S> EnvTypes for RocksdbEnvTypes<S>
where
    S: EnvSerializers,
{
    type TemplateStore = RocksdbTemplateStore<S::TemplateSerializer, S::TemplateDeserializer>;

    type AppStore = RocksdbAppStore<S::AppSerializer, S::AppDeserializer>;

    type TemplateAddressCompute = DefaultTemplateAddressCompute;

    type AppAddressCompute = DefaultAppAddressCompute;

    type TemplateHasher = DefaultTemplateHasher;
}
