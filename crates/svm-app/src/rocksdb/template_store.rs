use std::{marker::PhantomData, path::Path};

use crate::{
    error::StoreError,
    traits::{AppTemplateDeserializer, AppTemplateSerializer, AppTemplateStore},
    types::{AppTemplate, AppTemplateHash, AuthorAddr, TemplateAddr},
};

use svm_kv::{rocksdb::Rocksdb, traits::KVStore};

use lazy_static::lazy_static;
use log::info;

lazy_static! {
    static ref TEMPLATE_NS: Vec<u8> = vec![b't', b'e', b'm', b'p'];
    static ref CODE_NS: Vec<u8> = vec![b'c', b'o', b'd', b'e'];
}

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
    ) -> Result<(), StoreError> {
        info!("Storing `AppTemplate`: \n{:?}", template);
        info!("     Account Address: {:?}", addr.inner());
        info!("     Hash: {:?}", hash);

        let bytes = S::serialize(template, author);

        // template addr -> code-hash
        let entry1 = (&TEMPLATE_NS[..], addr.inner().as_slice(), &hash.0[..]);

        // code-hash -> code
        let entry2 = (&CODE_NS[..], &hash.0[..], &bytes[..]);

        self.db.store(&[entry1, entry2]);

        Ok(())
    }

    fn load(&self, addr: &TemplateAddr) -> Option<(AppTemplate, AuthorAddr)> {
        let addr = addr.inner().as_slice();

        info!("Loading `AppTemplate` account {:?}", addr);

        self.db.get(&TEMPLATE_NS, addr).and_then(|hash| {
            self.db
                .get(&CODE_NS[..], &hash)
                .and_then(|bytes| D::deserialize(&bytes[..]))
        })
    }
}
