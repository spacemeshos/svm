use std::collections::HashMap;
use std::marker::PhantomData;

use crate::env::ExtApp;
use crate::env::{default, traits};

use default::DefaultSerializers as S;
use traits::{AppDeserializer, AppSerializer, AppStore, EnvSerializers};

use svm_types::{Address, AppAddr, SpawnerAddr};

/// In-memory `AppStore` implementation.
/// Should be used for testing purposes only.
pub struct MemAppStore<S, D> {
    app_bytes: HashMap<Address, Vec<u8>>,

    phantom: PhantomData<(S, D)>,
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

            phantom: PhantomData,
        }
    }
}

impl<S, D> AppStore for MemAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
    fn store(&mut self, app: &ExtApp, addr: &AppAddr) {
        let bytes = S::serialize(app);

        self.app_bytes.insert(addr.inner().clone(), bytes);
    }

    fn load(&self, addr: &AppAddr) -> Option<ExtApp> {
        let bytes = self.app_bytes.get(addr.inner());

        bytes.and_then(|bytes| D::deserialize(&bytes[..]))
    }
}

/// `MemAppStore` with default serialization.
pub type DefaultMemAppStore =
    MemAppStore<<S as EnvSerializers>::AppSerializer, <S as EnvSerializers>::AppDeserializer>;
