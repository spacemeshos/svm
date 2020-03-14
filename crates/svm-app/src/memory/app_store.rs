use std::{collections::HashMap, marker::PhantomData};

use crate::{
    error::StoreError,
    traits::{AppDeserializer, AppSerializer, AppStore},
    types::{App, AppAddr, CreatorAddr},
};

use svm_common::Address;

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
    fn store(
        &mut self,
        app: &App,
        creator: &CreatorAddr,
        addr: &AppAddr,
    ) -> Result<(), StoreError> {
        let bytes = S::serialize(app, creator);

        self.app_bytes.insert(addr.inner().clone(), bytes);

        Ok(())
    }

    fn load(&self, addr: &AppAddr) -> Option<(App, CreatorAddr)> {
        let bytes = self.app_bytes.get(addr.inner());

        bytes.and_then(|bytes| D::deserialize(&bytes[..]))
    }
}
