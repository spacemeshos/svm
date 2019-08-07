use crate::types::CodeHash;

pub trait CodeHashStore {
    fn store(&mut self, code: &[u8], hash: CodeHash);

    fn get(&self, hash: CodeHash) -> Option<Vec<u8>>;
}
