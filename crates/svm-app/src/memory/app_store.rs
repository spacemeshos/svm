use std::collections::HashMap;
use std::marker::PhantomData;

use crate::{
    error::StoreError,
    traits::{AppDeserializer, AppSerializer, AppStore},
    types::App,
};

use svm_common::Address;

pub struct MemAppStore<S, D> {
    app_bytes: HashMap<Address, Vec<u8>>,
    _phantom: PhantomData<(S, D)>,
}

impl<S, D> MemAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
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
    fn store(&mut self, app: &App, app_addr: &Address) -> Result<(), StoreError> {
        let bytes: Vec<u8> = S::serialize(app);

        self.app_bytes.insert(app_addr.clone(), bytes);

        Ok(())
    }

    fn load(&self, app_addr: &Address) -> Option<App> {
        let bytes = self.app_bytes.get(app_addr);

        match bytes {
            None => None,
            Some(bytes) => D::deserialize(bytes.to_vec()),
        }
    }
}
