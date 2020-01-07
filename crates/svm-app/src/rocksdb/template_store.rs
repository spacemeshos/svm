use std::marker::PhantomData;
use std::path::Path;

use crate::{
    error::StoreError,
    traits::{AppTemplateDeserializer, AppTemplateSerializer, AppTemplateStore},
    types::{AppTemplate, AppTemplateHash},
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
        addr: &Address,
        hash: &AppTemplateHash,
    ) -> Result<(), StoreError> {
        info!("Storing `AppTemplate`: \n{:?}", template);
        info!("     `AppTemplate` Account Address: {:?}", addr);
        info!("     `AppTemplate` Hash: {:?}", hash);

        let bytes: Vec<u8> = S::serialize(template);

        let addr_hash = (addr.as_slice(), &hash.0[..]);
        let hash_wasm = (&hash.0[..], &bytes[..]);
        self.db.store(&[addr_hash, hash_wasm]);

        Ok(())
    }

    fn load(&self, addr: &Address) -> Option<AppTemplate> {
        info!("loading `AppTemplate` account {:?}", addr);

        match self.db.get(addr.as_slice()) {
            None => None,
            Some(hash) => match self.db.get(&hash) {
                None => panic!(format!(
                    "`AppTemplate` associated with `AppTemplate = {:?}` not found",
                    hash
                )),
                Some(bytes) => {
                    let template = D::deserialize(bytes.to_vec());
                    info!("loaded template: \n{:?}", template);

                    template
                }
            },
        }
    }
}
