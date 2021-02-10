use std::{collections::HashMap, marker::PhantomData};

use svm_codec::serializers::{TemplateDeserializer, TemplateSerializer};
use svm_types::{AuthorAddr, Template, TemplateAddr};

use crate::env::default::DefaultSerializerTypes as DSer;
use crate::env::traits::{EnvSerializerTypes, TemplateStore};
use crate::env::types::TemplateHash;

/// An in-memory implementation of `TemplateStore`
pub struct MemTemplateStore<S, D> {
    bytes: HashMap<TemplateHash, Vec<u8>>,
    hash: HashMap<TemplateAddr, TemplateHash>,
    phantom: PhantomData<(S, D)>,
}

impl<S, D> MemTemplateStore<S, D>
where
    S: TemplateSerializer,
    D: TemplateDeserializer,
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

impl<S, D> TemplateStore for MemTemplateStore<S, D>
where
    S: TemplateSerializer,
    D: TemplateDeserializer,
{
    fn store(
        &mut self,
        template: &Template,
        author: &AuthorAddr,
        addr: &TemplateAddr,
        hash: &TemplateHash,
    ) {
        self.hash.insert(addr.clone(), hash.clone());

        let bytes = S::serialize(template, author);
        self.bytes.insert(hash.clone(), bytes);
    }

    fn load(&self, addr: &TemplateAddr) -> Option<(Template, AuthorAddr)> {
        let hash = self.hash.get(addr);

        hash.and_then(|h| {
            self.bytes
                .get(&h)
                .and_then(|bytes| D::deserialize(&bytes[..]))
        })
    }
}

/// `MemTemplateStore` with default serialization.
pub type DefaultMemTemplateStore = MemTemplateStore<
    <DSer as EnvSerializerTypes>::TemplateSerializer,
    <DSer as EnvSerializerTypes>::TemplateDeserializer,
>;
