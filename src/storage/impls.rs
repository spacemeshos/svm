use super::traits::RawStorage;
use super::DefaultHasher;
use crate::Address;
use hash_db::Hasher;
use std::collections::HashMap;
use std::ops::Add;

pub struct SamplePagesKV {
    contract_addr: Address,
    data: HashMap<[u8; 32], Vec<u8>>,
}

impl SamplePagesKV {
    pub fn new(contract_addr: Address) -> Self {
        Self {
            contract_addr,
            data: HashMap::new(),
        }
    }

    fn store_page(&mut self, page: i32, v: &[u8]) {
        let page_hash = self.compute_page_hash(page);
        self.data.insert(page_hash, v.to_vec());
    }

    fn get_page(&self, page: i32) -> Vec<u8> {
        let page_hash = self.compute_page_hash(page);
        let v = self.data.get(&page_hash);

        if let Some(inner) = v {
            inner.to_vec()
        } else {
            Vec::new()
        }
    }

    #[inline(always)]
    fn compute_page_hash(&self, page: i32) -> [u8; 32] {
        let page_addr: [u8; 33] = self.contract_addr.add(page as u32);

        DefaultHasher::hash_key(&page_addr)
    }
}

pub struct SampleContractStorage<'kv> {
    kv: &'kv mut SamplePagesKV,
}

impl<'kv> SampleContractStorage<'kv> {
    pub fn new(kv: &'kv mut SamplePagesKV) -> Self {
        Self { kv }
    }
}

impl<'kv> RawStorage for SampleContractStorage<'kv> {
    fn read_page(&self, page: i32) {}

    fn write_page(&mut self, page: i32) {}
}
