use std::collections::HashMap;
use std::marker::PhantomData;

use crate::{
    error::StoreError,
    traits::{AppTemplateDeserializer, AppTemplateSerializer, AppTemplateStore},
    types::{AppTemplate, AppTemplateHash},
};

use svm_common::Address;

/// An in-memory implementation of `AppTemplateStore`
pub struct MemAppTemplateStore<S, D> {
    template_bytes: HashMap<AppTemplateHash, Vec<u8>>,
    template_hash: HashMap<Address, AppTemplateHash>,
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
            template_bytes: HashMap::new(),
            template_hash: HashMap::new(),
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
        addr: &Address,
        hash: &AppTemplateHash,
    ) -> Result<(), StoreError> {
        let bytes: Vec<u8> = S::serialize(template);

        self.template_bytes.insert(hash.clone(), bytes);
        self.template_hash.insert(addr.clone(), hash.clone());

        Ok(())
    }

    fn load(&self, addr: &Address) -> Option<AppTemplate> {
        let hash = self.template_hash.get(addr);

        hash.and_then(|h| {
            self.template_bytes
                .get(&h)
                .and_then(|bytes| D::deserialize(bytes.to_vec()))
        })
    }
}
