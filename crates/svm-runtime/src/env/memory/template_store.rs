use std::{collections::HashMap, marker::PhantomData};

use svm_codec::serializers::{AppTemplateDeserializer, AppTemplateSerializer};
use svm_types::{AppTemplate, AuthorAddr, TemplateAddr};

use crate::env::default::DefaultSerializerTypes as DSer;
use crate::env::traits::{AppTemplateStore, EnvSerializerTypes};
use crate::env::types::AppTemplateHash;

/// An in-memory implementation of `AppTemplateStore`
pub struct MemAppTemplateStore<S, D> {
    bytes: HashMap<AppTemplateHash, Vec<u8>>,
    hash: HashMap<TemplateAddr, AppTemplateHash>,
    phantom: PhantomData<(S, D)>,
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
            phantom: PhantomData,
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
    ) {
        self.hash.insert(addr.clone(), hash.clone());

        let bytes = S::serialize(template, author);
        self.bytes.insert(hash.clone(), bytes);
    }

    fn load(&self, addr: &TemplateAddr) -> Option<(AppTemplate, AuthorAddr)> {
        let hash = self.hash.get(addr);

        hash.and_then(|h| {
            self.bytes
                .get(&h)
                .and_then(|bytes| D::deserialize(&bytes[..]))
        })
    }
}

/// `MemAppTemplateStore` with default serialization.
pub type DefaultMemAppTemplateStore = MemAppTemplateStore<
    <DSer as EnvSerializerTypes>::TemplateSerializer,
    <DSer as EnvSerializerTypes>::TemplateDeserializer,
>;
