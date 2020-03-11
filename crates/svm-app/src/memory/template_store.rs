use std::{collections::HashMap, marker::PhantomData};

use crate::{
    error::StoreError,
    traits::{AppTemplateDeserializer, AppTemplateSerializer, AppTemplateStore},
    types::{AppTemplate, AppTemplateHash, AuthorAddr, TemplateAddr},
};

use svm_common::Address;

/// An in-memory implementation of `AppTemplateStore`
pub struct MemAppTemplateStore<S, D> {
    bytes: HashMap<AppTemplateHash, Vec<u8>>,

    hash: HashMap<Address, AppTemplateHash>,

    _phantom: PhantomData<(S, D)>,
}

impl<S, D> MemAppTemplateStore<S, D>
where
    S: AppTemplateSerializer,
    D: AppTemplateDeserializer,
{
    #[allow(clippy::new_without_default)]
    /// Create a new store
    pub fn new() -> Self {
        Self {
            bytes: HashMap::new(),
            hash: HashMap::new(),
            _phantom: PhantomData,
        }
    }
}

impl<S, D> AppTemplateStore for MemAppTemplateStore<S, D>
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
        let mut bytes = S::serialize(template, author);

        self.bytes.insert(hash.clone(), bytes);

        let addr = addr.inner().clone();
        self.hash.insert(addr, hash.clone());

        Ok(())
    }

    fn load(&self, addr: &TemplateAddr) -> Option<(AppTemplate, AuthorAddr)> {
        let hash = self.hash.get(addr.inner());

        hash.and_then(|h| {
            self.bytes
                .get(&h)
                .and_then(|bytes| D::deserialize(&bytes[..]))
        })
    }
}
