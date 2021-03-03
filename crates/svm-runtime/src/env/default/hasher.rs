use crate::env::{traits, hash};

use traits::TemplateHasher;
use hash::TemplateHash;

use svm_hash::{DefaultHasher, Hasher};
use svm_types::Template;

/// Default implementation for `TemplateCodeHasher`
pub struct DefaultTemplateHasher;

impl TemplateHasher for DefaultTemplateHasher {
    #[inline]
    fn hash(template: &Template) -> TemplateHash {
        let bytes = &template.code;

        let hash = DefaultHasher::hash(bytes);

        TemplateHash(hash)
    }
}
