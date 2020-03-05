use std::marker::PhantomData;
use std::path::Path;

use crate::{
    error::StoreError,
    traits::{AppDeserializer, AppSerializer, AppStore},
    types::{App, SpawnApp},
};

use svm_common::Address;

/// `AppStore` implementation backed-by `rocksdb`
pub struct RocksdbAppStore<S, D> {
    _phantom: PhantomData<(S, D)>,
}

impl<S, D> RocksdbAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
    /// New `RocksdbAppStore` instance
    pub fn new<P>(_path: &P) -> Self
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
    fn store(&mut self, _app: &SpawnApp, _app_addr: &Address) -> Result<(), StoreError> {
        todo!()
    }

    fn load(&self, _app_addr: &Address) -> Option<App> {
        todo!()
    }
}
