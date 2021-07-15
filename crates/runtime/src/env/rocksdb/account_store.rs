use log::info;

use std::marker::PhantomData;
use std::path::Path;

use svm_kv::rocksdb::Rocksdb;
use svm_kv::traits::RawKV;

use crate::env::{self, traits};

use env::ExtApp;
use svm_types::{AccountAddr, Address, TemplateAddr};
use traits::{AccountDeserializer, AccountSerializer, AccountStore};

const ACCOUNT_KEY_PREFIX: &'static [u8] = b"acc:";
const ACCOUNT_TEMPLATE_KEY_PREFIX: &'static [u8] = b"acc-temp:";

/// `AppStore` implementation backed-by `rocksdb`
pub struct RocksAccountStore<S, D> {
    db: Rocksdb,
    phantom: PhantomData<(S, D)>,
}

impl<S, D> AccountStore for RocksAccountStore<S, D>
where
    S: AccountSerializer,
    D: AccountDeserializer,
{
    fn store(&mut self, account: &ExtApp, addr: &AccountAddr) {
        let addr = addr.inner();

        info!("Storing an `Account`: \n{:?}", addr);

        // 1) `Account Address` -> serialized `Account`
        let key = self.account_key(addr);
        let bytes = S::serialize(account);
        let entry1 = (&key[..], &bytes[..]);

        // 2) `Account Address` -> `Template Address`
        let key = self.account_template_key(addr);
        let addr = self.account_template_addr(account);
        let entry2 = (&key[..], addr.as_slice());

        self.db.set(&[entry1, entry2]);
    }

    fn load(&self, addr: &AccountAddr) -> Option<ExtApp> {
        let addr = addr.inner().as_slice();

        info!("Loading an `Account` {:?}", addr);

        self.db.get(addr).and_then(|hash| {
            self.db
                .get(&hash)
                .and_then(|bytes| D::deserialize(&bytes[..]))
        })
    }

    fn find_template_addr(&self, addr: &AccountAddr) -> Option<TemplateAddr> {
        let addr = addr.inner().as_slice();

        self.db.get(addr).and_then(|hash| {
            self.db
                .get(&hash)
                .and_then(|bytes| D::deserialize_template_addr(&bytes[..]))
        })
    }
}

impl<S, D> RocksAccountStore<S, D>
where
    S: AccountSerializer,
    D: AccountDeserializer,
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
    fn account_key(&self, addr: &Address) -> Vec<u8> {
        // Keys mapping from an `Account Address` to `Template Address`
        // are of the pattern "account:ADDRESS"

        let mut key = Vec::with_capacity(Address::len() + ACCOUNT_KEY_PREFIX.len());

        key.extend_from_slice(ACCOUNT_KEY_PREFIX);
        key.extend_from_slice(addr.as_slice());

        key
    }

    #[inline]
    fn account_template_key(&self, addr: &Address) -> Vec<u8> {
        // Keys mapping from an `Account Address` to `Template Address`
        // are of the pattern "acc-temp:ADDRESS"

        let mut key = Vec::with_capacity(Address::len() + ACCOUNT_TEMPLATE_KEY_PREFIX.len());
        key.extend_from_slice(ACCOUNT_TEMPLATE_KEY_PREFIX);
        key.extend_from_slice(addr.as_slice());

        key
    }

    #[inline]
    fn account_template_addr<'a>(&self, account: &'a ExtApp) -> &'a Address {
        let addr = account.template_addr();
        addr.inner()
    }
}
