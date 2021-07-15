use rocksdb::{RocksAccountStore, RocksTemplateStore};

use crate::env::{default, rocksdb};
use crate::EnvTypes;

/// `RocksTemplateStore` with a default serialization.
pub type DefaultRocksTemplateStore =
    RocksTemplateStore<default::DefaultTemplateSerializer, default::DefaultTemplateDeserializer>;

/// `RocksAccountStore` with a default serialization.
pub type DefaultRocksAccountStore =
    RocksAccountStore<default::DefaultAccountSerializer, default::DefaultAccountDeserializer>;

pub struct DefaultRocksEnvTypes;

impl EnvTypes for DefaultRocksEnvTypes {
    type TemplateStore = DefaultRocksTemplateStore;

    type AccountStore = DefaultRocksAccountStore;

    type TemplateAddressCompute = default::DefaultTemplateAddressCompute;

    type AccountAddressCompute = default::DefaultAccountAddressCompute;

    type TemplateHasher = default::DefaultTemplateHasher;

    type Pricer = svm_gas::resolvers::V0PriceResolver;
}
