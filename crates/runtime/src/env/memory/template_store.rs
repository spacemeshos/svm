use std::collections::HashMap;
use std::marker::PhantomData;

use svm_types::TemplateAddr;

use crate::env::ExtTemplate;
use crate::env::{hash, traits};

use hash::TemplateHash;
use traits::{TemplateDeserializer, TemplateSerializer, TemplateStore};

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
    fn store(&mut self, template: &ExtTemplate, addr: &TemplateAddr, hash: &TemplateHash) {
        self.hash.insert(addr.clone(), hash.clone());

        let bytes = S::serialize(template);
        self.bytes.insert(hash.clone(), bytes);
    }

    fn load(&self, addr: &TemplateAddr, include_extra: bool) -> Option<ExtTemplate> {
        let hash = self.hash.get(addr);

        hash.and_then(|h| {
            self.bytes
                .get(&h)
                .and_then(|bytes| D::deserialize(&bytes[..], include_extra))
        })
    }
}
