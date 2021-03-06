use crate::env::{hash, traits};

use hash::TemplateHash;
use traits::TemplateHasher;

use svm_hash::{DefaultHasher, Hasher};
use svm_types::Template;

/// Default implementation for `TemplateCodeHasher`
pub struct DefaultTemplateHasher;

impl TemplateHasher for DefaultTemplateHasher {
    #[inline]
    fn hash(template: &Template) -> TemplateHash {
        let bytes = template.code();

        let hash = DefaultHasher::hash(bytes);

        TemplateHash::new(hash)
    }
}
