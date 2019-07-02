use super::traits::RawStorage;
use std::collections::HashMap;

pub struct SampleKV {
    data: HashMap<[usize; 32], Vec<u8>>,
}

impl SampleKV {
    fn store(&mut self, k: &[usize; 32], v: &[u8]) {
        self.data.insert(*k, v.to_vec());
    }

    fn get(&self, k: &[usize; 32]) -> Vec<u8> {
        let v = self.data.get(k);

        if let Some(inner) = v {
            inner.to_vec()
        } else {
            Vec::new()
        }
    }
}

pub struct SampleContractStorage<'kv> {
    kv: &'kv mut SampleKV,
}

impl<'kv> SampleContractStorage<'kv> {
    pub fn new(kv: &'kv mut SampleKV) -> Self {
        Self { kv: kv }
    }
}

impl<'kv> RawStorage for SampleContractStorage<'kv> {
    fn read_page(page: i32) {}

    fn write_page(page: i32) {}
}
