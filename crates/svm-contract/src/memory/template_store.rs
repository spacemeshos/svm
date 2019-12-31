use std::collections::HashMap;
use std::marker::PhantomData;

use crate::{
    traits::{AppTemplateDeserializer, AppTemplateSerializer, AppTemplateStore},
    types::CodeHash,
    wasm::AppTemplate,
};

use svm_common::Address;

/// An in-memory implementation of `AppTemplateStore`
pub struct MemAppTemplateStore<S, D> {
    template_bytes: HashMap<CodeHash, Vec<u8>>,
    template_hash: HashMap<Address, CodeHash>,
    _phantom: PhantomData<(S, D)>,
}

#[allow(dead_code)]
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

impl<S, D> AppTemplateStore<S, D> for MemAppTemplateStore<S, D>
where
    S: AppTemplateSerializer,
    D: AppTemplateDeserializer,
{
    fn store(&mut self, template: &AppTemplate, addr: &Address, hash: CodeHash) {
        let serialized: Vec<u8> = S::serialize(template);

        self.template_bytes.insert(hash, serialized);
        self.template_hash.insert(addr.clone(), hash);
    }

    fn load(&self, addr: &Address) -> Option<AppTemplate> {
        match self.template_hash.get(addr) {
            None => None,
            Some(hash) => match self.template_bytes.get(&hash) {
                None => panic!(format!(
                    "`AppTemplate` associated with `Hash = {:?}` not found",
                    hash
                )),
                Some(template_bytes) => {
                    let template = D::deserialize(template_bytes.to_vec());
                    Some(template)
                }
            },
        }
    }
}
