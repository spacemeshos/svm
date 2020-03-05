use crate::{
    traits::AppTemplateHasher,
    types::{AppTemplateHash, DeployAppTemplate},
};

use svm_common::{DefaultKeyHasher, KeyHasher};

/// Default implementation for `AppTemplateCodeHasher`
pub struct DefaultTemplateHasher;

impl AppTemplateHasher for DefaultTemplateHasher {
    #[inline]
    fn hash(template: &DeployAppTemplate) -> AppTemplateHash {
        let bytes = &template.template.code[..];

        let hash = DefaultKeyHasher::hash(bytes);

        AppTemplateHash(hash)
    }
}
