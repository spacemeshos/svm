use svm_hash::{Blake3Hasher, Hasher};
use svm_types::Template;

use super::super::traits::TemplateHasher;
use crate::env::TemplateHash;

/// Default implementation for `TemplateCodeHasher`
pub struct DefaultTemplateHasher;

impl TemplateHasher for DefaultTemplateHasher {
    #[inline]
    fn hash(template: &Template) -> TemplateHash {
        Blake3Hasher::hash(template.code())
    }
}
