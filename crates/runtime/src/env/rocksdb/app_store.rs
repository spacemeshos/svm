use std::marker::PhantomData;
use std::path::Path;

use svm_kv::rocksdb::Rocksdb;
use svm_kv::traits::RawKV;

use crate::env::{self, traits};

use env::ExtApp;
use traits::{AppDeserializer, AppSerializer, AppStore};

use log::info;

use svm_types::{Address, AppAddr, TemplateAddr};

const APP_KEY_PREFIX: &'static [u8] = b"app:";
const APP_TEMPLATE_KEY_PREFIX: &'static [u8] = b"app-template:";

/// `AppStore` implementation backed-by `rocksdb`
pub struct RocksAppStore<S, D> {
    db: Rocksdb,

    phantom: PhantomData<(S, D)>,
}

impl<S, D> AppStore for RocksAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
    fn store(&mut self, app: &ExtApp, addr: &AppAddr) {
        let addr = addr.inner();

        info!("Storing `App`: \n{:?}", addr);

        // 1) `App Address` -> serialized `App`
        let key = self.app_key(addr);
        let bytes = S::serialize(app);
        let entry1 = (&key[..], &bytes[..]);

        // 2) `App Address` -> `Template Address`
        let key = self.app_template_key(addr);
        let addr = self.app_template_addr(app);
        let entry2 = (&key[..], addr.as_slice());

        self.db.set(&[entry1, entry2]);
    }

    fn load(&self, addr: &AppAddr) -> Option<ExtApp> {
        let addr = addr.inner().as_slice();

        info!("Loading `App` {:?}", addr);

        self.db.get(addr).and_then(|hash| {
            self.db
                .get(&hash)
                .and_then(|bytes| D::deserialize(&bytes[..]))
        })
    }

    fn find_template_addr(&self, addr: &AppAddr) -> Option<TemplateAddr> {
        let addr = addr.inner().as_slice();

        self.db.get(addr).and_then(|hash| {
            self.db
                .get(&hash)
                .and_then(|bytes| D::deserialize_template_addr(&bytes[..]))
        })
    }
}

impl<S, D> RocksAppStore<S, D>
where
    S: AppSerializer,
    D: AppDeserializer,
{
    /// New instance
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            db: Rocksdb::new(path),
            phantom: PhantomData,
        }
    }

    #[inline]
    fn app_key(&self, addr: &Address) -> Vec<u8> {
        // Keys mapping from an `App Address` to `Template Address`
        // are of the pattern "app:APP_ADDRESS"

        let mut key = Vec::with_capacity(Address::len() + APP_KEY_PREFIX.len());

        key.extend_from_slice(APP_KEY_PREFIX);
        key.extend_from_slice(addr.as_slice());

        key
    }

    #[inline]
    fn app_template_key(&self, addr: &Address) -> Vec<u8> {
        // Keys mapping from an `App Address` to `Template Address`
        // are of the pattern "app-template:APP_ADDRESS"

        let mut key = Vec::with_capacity(Address::len() + APP_TEMPLATE_KEY_PREFIX.len());
        key.extend_from_slice(APP_TEMPLATE_KEY_PREFIX);
        key.extend_from_slice(addr.as_slice());

        key
    }

    #[inline]
    fn app_template_addr<'a>(&self, app: &'a ExtApp) -> &'a Address {
        let addr = app.template_addr();

        addr.inner()
    }
}
