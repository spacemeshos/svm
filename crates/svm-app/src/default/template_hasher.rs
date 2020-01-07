use crate::{traits::AppTemplateHasher, types::AppTemplateHash};
use svm_common::{DefaultKeyHasher, KeyHasher};

/// Default implementation for `AppTemplateCodeHasher`
pub struct DefaultTemplateHasher;

impl AppTemplateHasher for DefaultTemplateHasher {
    #[inline(always)]
    fn hash(bytes: &[u8]) -> AppTemplateHash {
        let hash: [u8; 32] = DefaultKeyHasher::hash(bytes);
        AppTemplateHash(hash)
    }
}
