use std::marker::PhantomData;
use std::path::Path;

use crate::{
    error::StoreError,
    traits::{AppDeserializer, AppSerializer, AppStore},
    types::App,
};

use svm_common::Address;

pub struct RocksdbAppStore<S, D> {
    _phantom: PhantomData<(S, D)>,
}

impl<S, D> RocksdbAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
    pub fn new<P>(path: &P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<S, D> AppStore for RocksdbAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
    fn store(&mut self, app: &App, app_addr: &Address) -> Result<(), StoreError> {
        todo!()
    }

    fn load(&self, app_addr: &Address) -> Option<App> {
        todo!()
    }
}
