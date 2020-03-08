use std::{collections::HashMap, marker::PhantomData};

use crate::{
    error::StoreError,
    traits::{AppTemplateDeserializer, AppTemplateSerializer, AppTemplateStore},
    types::{AppTemplate, AppTemplateHash, HostCtx},
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
        host_ctx: &HostCtx,
        addr: &Address,
        hash: &AppTemplateHash,
    ) -> Result<(), StoreError> {
        let bytes: Vec<u8> = S::serialize(template);

        self.bytes.insert(hash.clone(), bytes);
        self.hash.insert(addr.clone(), hash.clone());

        Ok(())
    }

    fn load(&self, addr: &Address) -> Option<AppTemplate> {
        let hash = self.hash.get(addr);

        hash.and_then(|h| {
            self.bytes
                .get(&h)
                .and_then(|bytes| D::deserialize(&bytes[..]))
        })
    }
}
