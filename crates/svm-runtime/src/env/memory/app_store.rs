use std::{collections::HashMap, marker::PhantomData};

use crate::env::default::DefaultSerializerTypes as DSer;
use crate::env::traits::{AppStore, EnvSerializerTypes};

use svm_codec::serializers::{AppDeserializer, AppSerializer};
use svm_common::Address;
use svm_types::{App, AppAddr, CreatorAddr};

/// In-memory `AppStore` implementation.
/// Should be used for testing purposes only.
pub struct MemAppStore<S, D> {
    app_bytes: HashMap<Address, Vec<u8>>,

    _phantom: PhantomData<(S, D)>,
}

impl<S, D> MemAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
    /// Initializes a new `MemAppStore`
    pub fn new() -> Self {
        Self {
            app_bytes: HashMap::new(),

            _phantom: PhantomData,
        }
    }
}

impl<S, D> AppStore for MemAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
    fn store(&mut self, app: &App, creator: &CreatorAddr, addr: &AppAddr) {
        let bytes = S::serialize(app, creator);

        self.app_bytes.insert(addr.inner().clone(), bytes);
    }

    fn load(&self, addr: &AppAddr) -> Option<(App, CreatorAddr)> {
        let bytes = self.app_bytes.get(addr.inner());

        bytes.and_then(|bytes| D::deserialize(&bytes[..]))
    }
}

/// `MemAppStore` with default serialization.
pub type DefaultMemAppStore = MemAppStore<
    <DSer as EnvSerializerTypes>::AppSerializer,
    <DSer as EnvSerializerTypes>::AppDeserializer,
>;
