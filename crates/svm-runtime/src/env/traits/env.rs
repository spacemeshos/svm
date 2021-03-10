use std::io::Cursor;

use crate::env::{hash, traits};
use crate::env::{ExtApp, ExtSpawnApp, ExtTemplate};

use hash::TemplateHash;
use traits::{
    AppAddressCompute, AppDeserializer, AppSerializer, AppStore, TemplateAddressCompute,
    TemplateDeserializer, TemplateHasher, TemplateSerializer, TemplateStore,
};

use svm_codec::ParseError;
use svm_codec::{app, template, transaction};

use svm_types::{AppAddr, SpawnApp, Template, TemplateAddr, Transaction};

/// `Env` storage serialization types
pub trait EnvSerializerTypes {
    /// `Template`'s Serializer
    type TemplateSerializer: TemplateSerializer;

    /// `Template`'s Deserializer
    type TemplateDeserializer: TemplateDeserializer;

    /// `App`'s Serializer
    type AppSerializer: AppSerializer;

    /// `App`'s Deserializer
    type AppDeserializer: AppDeserializer;
}

/// Aggregates types that are required by `Env`
pub trait EnvTypes {
    /// `Template` store type.
    type TemplateStore: TemplateStore;

    /// `AppStore` store type.
    type AppStore: AppStore;

    /// Compute `Template` address type.
    type TemplateAddressCompute: TemplateAddressCompute;

    /// Compute `App` address type.
    type AppAddressCompute: AppAddressCompute;

    /// `Template` content Hasher type.
    type TemplateHasher: TemplateHasher;
}
