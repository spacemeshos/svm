use std::{marker::PhantomData, path::Path};

use crate::{
    error::StoreError,
    traits::{AppTemplateDeserializer, AppTemplateSerializer, AppTemplateStore},
    types::{AppAddr, AppTemplate, AppTemplateHash, AuthorAddr, CreatorAddr, TemplateAddr},
};

use svm_common::Address;
use svm_kv::{rocksdb::Rocksdb, traits::KVStore};

use log::info;

/// `AppTemplate` store backed by `rocksdb`
pub struct RocksdbAppTemplateStore<S, D> {
    db: Rocksdb,
    _phantom: PhantomData<(S, D)>,
}

impl<S, D> RocksdbAppTemplateStore<S, D>
where
    S: AppTemplateSerializer,
    D: AppTemplateDeserializer,
{
    /// Creates a new template store at the given path
    pub fn new<P>(path: &P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            db: Rocksdb::new(path),
            _phantom: PhantomData,
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
    ) -> Result<(), StoreError> {
        info!("Storing `AppTemplate`: \n{:?}", template);
        info!("     Account Address: {:?}", addr.inner());
        info!("     Hash: {:?}", hash);

        let bytes = S::serialize(template, author);

        let addr_hash = (addr.inner().as_slice(), &hash.0[..]);
        let hash_wasm = (&hash.0[..], &bytes[..]);
        self.db.store(&[addr_hash, hash_wasm]);

        Ok(())
    }

    fn load(&self, addr: &TemplateAddr) -> Option<(AppTemplate, AuthorAddr)> {
        let addr = addr.inner().as_slice();

        info!("loading `AppTemplate` account {:?}", addr);

        self.db.get(addr).and_then(|hash| {
            self.db
                .get(&hash)
                .and_then(|bytes| D::deserialize(&bytes[..]))
        })
    }
}
