use crate::{traits::AppTemplateHasher, types::CodeHash};
use svm_common::{DefaultKeyHasher, KeyHasher};

/// Default implementation for `ContractCodeHasher`
pub struct DefaultCodeHasher;

impl AppTemplateHasher for DefaultCodeHasher {
    #[inline(always)]
    fn hash(bytes: &[u8]) -> CodeHash {
        let hash: [u8; 32] = DefaultKeyHasher::hash(bytes);
        CodeHash(hash)
    }
}
