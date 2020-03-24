use std::{marker::PhantomData, path::Path};

use crate::{
    error::StoreError,
    traits::{AppDeserializer, AppSerializer, AppStore},
    types::{App, AppAddr, CreatorAddr},
};

/// `AppStore` implementation backed-by `rocksdb`
pub struct RocksdbAppStore<S, D> {
    phantom: PhantomData<(S, D)>,
}

impl<S, D> RocksdbAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
    /// New `RocksdbAppStore` instance
    pub fn new<P>(_path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<S, D> AppStore for RocksdbAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
    fn store(
        &mut self,
        _app: &App,
        _creator: &CreatorAddr,
        _addr: &AppAddr,
    ) -> Result<(), StoreError> {
        todo!()
    }

    fn load(&self, _addr: &AppAddr) -> Option<(App, CreatorAddr)> {
        todo!()
    }
}
