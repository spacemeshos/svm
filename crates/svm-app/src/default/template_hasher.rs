use crate::{
    traits::AppTemplateHasher,
    types::{AppTemplate, AppTemplateHash},
};

use svm_common::{DefaultKeyHasher, KeyHasher};

/// Default implementation for `AppTemplateCodeHasher`
pub struct DefaultTemplateHasher;

impl AppTemplateHasher for DefaultTemplateHasher {
    #[inline]
    fn hash(template: &AppTemplate) -> AppTemplateHash {
        let bytes = &template.code[..];

        let hash = DefaultKeyHasher::hash(bytes);

        AppTemplateHash(hash)
    }
}
