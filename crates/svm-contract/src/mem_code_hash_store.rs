use crate::traits::CodeHashStore;
use crate::types::CodeHash;

use std::collections::HashMap;

pub struct MemCodeHashStore {
    map: HashMap<CodeHash, Vec<u8>>,
}

#[allow(dead_code)]
impl MemCodeHashStore {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl CodeHashStore for MemCodeHashStore {
    fn store(&mut self, code: &[u8], hash: CodeHash) {
        self.map.insert(hash, code.to_owned());
    }

    fn get(&self, hash: CodeHash) -> Option<Vec<u8>> {
        match self.map.get(&hash) {
            Some(code) => Some(code.to_owned()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_code() {
        let mut store = MemCodeHashStore::new();

        let hash = CodeHash([10; 32]);
        assert_eq!(None, store.get(hash));

        // 1st store
        store.store(&[10, 20, 30], hash);
        assert_eq!(vec![10, 20, 30], store.get(hash).unwrap());

        // 2nd store, no change
        store.store(&[10, 20, 30], hash);
        assert_eq!(vec![10, 20, 30], store.get(hash).unwrap());
    }

    #[test]
    fn two_codes() {
        let mut store1 = MemCodeHashStore::new();
        let mut store2 = MemCodeHashStore::new();

        let hash1 = CodeHash([10; 32]);
        let hash2 = CodeHash([20; 32]);
        assert_eq!(None, store1.get(hash1));
        assert_eq!(None, store2.get(hash2));

        store1.store(&[10, 20, 30], hash1);
        store2.store(&[40, 50, 60], hash2);
        assert_eq!(vec![10, 20, 30], store1.get(hash1).unwrap());
        assert_eq!(vec![40, 50, 60], store2.get(hash2).unwrap());
    }
}
