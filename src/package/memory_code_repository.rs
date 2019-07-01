use super::traits::{CodeHash, CodeRepository};
use std::collections::HashMap;

pub struct MemoryCodeRepository {
    db: HashMap<CodeHash, Vec<u8>>,
}

impl MemoryCodeRepository {
    fn new() -> Self {
        Self { db: HashMap::new() }
    }
}

impl CodeRepository for MemoryCodeRepository {
    fn exists(&self, code_hash: &CodeHash) -> bool {
        let entry = self.db.get(code_hash);
        entry.is_some()
    }

    fn try_get(&self, code_hash: &CodeHash) -> Option<Vec<u8>> {
        let entry = self.db.get(code_hash);

        if entry.is_some() {
            let cloned_vec = entry.unwrap().to_vec();
            Some(cloned_vec)
        } else {
            None
        }
    }

    fn store<H: hash_db::Hasher<Out = CodeHash>>(&mut self, code: &[u8]) -> CodeHash {
        let code_hash = H::hash(code);

        self.db.insert(code_hash.clone(), code.to_vec());

        code_hash
    }
}
