use crate::traits::ContractCodeHasher;
use crate::types::CodeHash;
use svm_common::KeyHasher;

pub struct DefaultCodeHasher;

impl ContractCodeHasher for DefaultCodeHasher {
    #[inline(always)]
    fn hash(bytes: &[u8]) -> CodeHash {
        let hash: [u8; 32] = svm_common::DefaultKeyHasher::hash(bytes);
        CodeHash(hash)
    }
}
