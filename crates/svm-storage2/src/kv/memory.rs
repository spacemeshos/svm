use super::traits::KV;

use std::collections::HashMap;

#[doc(hidden)]
pub struct MemKV {
    entries: HashMap<Vec<u8>, Vec<u8>>,
}

#[doc(hidden)]
impl MemKV {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

#[doc(hidden)]
impl KV for MemKV {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.entries.get(key).cloned()
    }

    fn set(&mut self, changes: &[(Vec<u8>, Vec<u8>)]) {
        for (k, v) in changes.iter() {
            let k = k.to_vec();
            let v = v.to_vec();

            self.entries.insert(k, v);
        }
    }
}
