use std::marker::PhantomData;
use std::path::Path;

use svm_kv::rocksdb::Rocksdb;
use svm_kv::traits::RawKV;

use svm_types::{Address, AuthorAddr, Template, TemplateAddr};

use crate::env;

use env::{default, hash, traits};

use default::DefaultSerializers as S;
use env::ExtTemplate;
use hash::TemplateHash;
use traits::{EnvSerializers, TemplateDeserializer, TemplateSerializer, TemplateStore};

use log::info;

const TEMPLATE_KEY_PREFIX: &'static [u8] = b"template:";
const TEMPLATE_HASH_KEY_PREFIX: &'static [u8] = b"template-hash:";

/// `Template` store backed by `rocksdb`
pub struct RocksdbTemplateStore<S, D> {
    db: Rocksdb,

    phantom: PhantomData<(S, D)>,
}

impl<S, D> TemplateStore for RocksdbTemplateStore<S, D>
where
    S: TemplateSerializer,
    D: TemplateDeserializer,
{
    fn store(&mut self, template: &ExtTemplate, addr: &TemplateAddr, hash: &TemplateHash) {
        let addr = addr.inner();

        info!("Storing `Template`: \n{:?}", addr);
        info!("     Account Address: {:?}", addr);
        info!("     Hash: {:?}", hash);

        // 1) Template `Address` -> `TemplateHash`
        let mut key = self.template_key(addr);
        let entry1 = (&key[..], hash.as_slice());

        // 2) `TemplateHash` -> serialized `Template`
        let mut key = self.template_hash_key(hash);
        let bytes = S::serialize(template);
        let entry2 = (&key[..], bytes.as_slice());

        self.db.set(&[entry1, entry2]);
    }

    fn load(&self, addr: &TemplateAddr) -> Option<ExtTemplate> {
        let addr = addr.inner().as_slice();

        info!("Loading `Template` {:?}", addr);

        self.db.get(addr).and_then(|hash| {
            self.db
                .get(&hash)
                .and_then(|bytes| D::deserialize(&bytes[..]))
        })
    }
}

impl<S, D> RocksdbTemplateStore<S, D>
where
    S: TemplateSerializer,
    D: TemplateDeserializer,
{
    /// Creates a new template store at the given path
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
    fn template_key(&self, addr: &Address) -> Vec<u8> {
        // Keys mapping from an `Template Address` to `Template Hash`
        // are of the pattern "template:TEMPLATE_ADDRESS"

        let mut key = Vec::with_capacity(Address::len() + TEMPLATE_KEY_PREFIX.len());

        key.extend_from_slice(TEMPLATE_KEY_PREFIX);
        key.extend_from_slice(addr.as_slice());

        key
    }

    #[inline]
    fn template_hash_key(&self, hash: &TemplateHash) -> Vec<u8> {
        // Keys mapping from an `Template Hash` to `Template`
        // are of the pattern "template-hash:TEMPLATE_ADDRESS"

        let mut key = Vec::with_capacity(TemplateHash::len() + TEMPLATE_HASH_KEY_PREFIX.len());

        key.extend_from_slice(TEMPLATE_HASH_KEY_PREFIX);
        key.extend_from_slice(hash.as_slice());

        key
    }
}
