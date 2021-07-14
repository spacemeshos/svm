use std::collections::HashMap;
use std::marker::PhantomData;

use crate::env::{self, traits};

use env::ExtApp;
use traits::{AppDeserializer, AppSerializer, AppStore};

use svm_types::{AccountAddr, Address, TemplateAddr};

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
    fn store(&mut self, app: &ExtApp, addr: &AccountAddr) {
        let bytes = S::serialize(app);

        self.app_bytes.insert(addr.inner().clone(), bytes);
    }

    fn load(&self, addr: &AccountAddr) -> Option<ExtApp> {
        let bytes = self.app_bytes.get(addr.inner());

        bytes.and_then(|bytes| D::deserialize(&bytes[..]))
    }

    fn resolve_template_addr(&self, addr: &AccountAddr) -> Option<TemplateAddr> {
        let app = self.load(addr);

        app.map(|x| x.template_addr().clone())
    }
}
