use crate::env::traits::TemplateHasher;
use crate::env::types::TemplateHash;

use svm_common::{DefaultKeyHasher, KeyHasher};
use svm_types::Template;

/// Default implementation for `TemplateCodeHasher`
pub struct DefaultTemplateHasher;

impl TemplateHasher for DefaultTemplateHasher {
    #[inline]
    fn hash(template: &Template) -> TemplateHash {
        let bytes = &template.code[..];

        let hash = DefaultKeyHasher::hash(bytes);

        TemplateHash(hash)
    }
}
