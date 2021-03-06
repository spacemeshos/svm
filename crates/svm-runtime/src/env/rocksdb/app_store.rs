use std::path::Path;
use std::{marker::PhantomData, todo};

use crate::env::{self, default, traits};

use default::DefaultSerializers as S;
use env::ExtApp;
use svm_types::{Address, AppAddr, TemplateAddr};

use traits::{AppDeserializer, AppSerializer, AppStore, EnvSerializers};

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
    fn store(&mut self, app: &ExtApp, addr: &AppAddr) {
        todo!()
    }

    fn load(&self, addr: &AppAddr) -> Option<ExtApp> {
        todo!()
    }

    fn find_template_addr(&self, addr: &AppAddr) -> Option<TemplateAddr> {
        todo!()
    }
}
