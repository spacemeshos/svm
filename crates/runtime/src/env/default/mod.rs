mod address_compute;
mod hasher;
mod serialize;

pub use serialize::{
    DefaultAccountDeserializer, DefaultAccountSerializer, DefaultTemplateDeserializer,
    DefaultTemplateSerializer,
};

#[cfg(feature = "default-memory")]
mod memory;

#[cfg(feature = "default-memory")]
pub use memory::{DefaultMemAccountStore, DefaultMemEnvTypes, DefaultMemTemplateStore};

#[cfg(feature = "default-rocksdb")]
mod rocksdb;

#[cfg(feature = "default-rocksdb")]
pub use rocksdb::{DefaultRocksAccountStore, DefaultRocksEnvTypes, DefaultRocksTemplateStore};

pub use address_compute::{DefaultAccountAddressCompute, DefaultTemplateAddressCompute};
pub use hasher::DefaultTemplateHasher;
