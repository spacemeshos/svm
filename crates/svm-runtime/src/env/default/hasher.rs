use crate::env::traits::AppTemplateHasher;
use crate::env::types::AppTemplateHash;

use svm_common::{DefaultKeyHasher, KeyHasher};
use svm_types::Template;

/// Default implementation for `AppTemplateCodeHasher`
pub struct DefaultTemplateHasher;

impl AppTemplateHasher for DefaultTemplateHasher {
    #[inline]
    fn hash(template: &Template) -> AppTemplateHash {
        let bytes = &template.code[..];

        let hash = DefaultKeyHasher::hash(bytes);

        AppTemplateHash(hash)
    }
}
