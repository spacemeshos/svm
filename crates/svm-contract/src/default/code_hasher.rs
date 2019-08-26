use crate::traits::ContractCodeHasher;
use crate::types::CodeHash;

pub struct DefaultCodeHasher;

impl ContractCodeHasher for DefaultCodeHasher {
    fn hash(bytes: &[u8]) -> CodeHash {
        // svm_common::DefaultKeyHasher;
        unimplemented!()
    }
}
