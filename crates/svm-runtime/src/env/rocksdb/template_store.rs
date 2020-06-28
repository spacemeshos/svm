use std::{marker::PhantomData, path::Path};

use svm_kv::{rocksdb::Rocksdb, traits::RawKV};
use svm_types::{AppTemplate, AuthorAddr, TemplateAddr};

use crate::env::traits::AppTemplateStore;
use crate::env::types::AppTemplateHash;

use svm_codec::serializers::{AppTemplateDeserializer, AppTemplateSerializer};

use log::info;

/// `AppTemplate` store backed by `rocksdb`
pub struct RocksdbAppTemplateStore<S, D> {
    db: Rocksdb,
    phantom: PhantomData<(S, D)>,
}

impl<S, D> RocksdbAppTemplateStore<S, D>
where
    S: AppTemplateSerializer,
    D: AppTemplateDeserializer,
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
}

impl<S, D> AppTemplateStore for RocksdbAppTemplateStore<S, D>
where
    S: AppTemplateSerializer,
    D: AppTemplateDeserializer,
{
    fn store(
        &mut self,
        template: &AppTemplate,
        author: &AuthorAddr,
        addr: &TemplateAddr,
        hash: &AppTemplateHash,
    ) {
        info!("Storing `AppTemplate`: \n{:?}", template);
        info!("     Account Address: {:?}", addr.inner());
        info!("     Hash: {:?}", hash);

        let bytes = S::serialize(template, author);

        // template addr -> code-hash
        let entry1 = (addr.inner().as_slice(), &hash.0[..]);

        // code-hash -> code
        let entry2 = (&hash.0[..], &bytes[..]);

        self.db.set(&[entry1, entry2]);
    }

    fn load(&self, addr: &TemplateAddr) -> Option<(AppTemplate, AuthorAddr)> {
        let addr = addr.inner().as_slice();

        info!("Loading `AppTemplate` account {:?}", addr);

        self.db.get(addr).and_then(|hash| {
            self.db
                .get(&hash)
                .and_then(|bytes| D::deserialize(&bytes[..]))
        })
    }
}
