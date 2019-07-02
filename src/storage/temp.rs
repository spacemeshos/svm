// use super::traits::StoragePageHasher;
// use super::DefaultPageHasher;
// use crate::common::Address;
// use std::collections::HashMap;
//
// pub struct SamplePagesKV {
//     contract_addr: Address,
//
//     data: HashMap<[u8; 32], Vec<u8>>,
// }
//
// impl SamplePagesKV {
//     pub fn new(contract_addr: Address) -> Self {
//         Self {
//             contract_addr,
//             data: HashMap::new(),
//         }
//     }
//
//     fn store_page(&mut self, page: i32, v: &[u8]) {
//         let page_hash = self.compute_page_hash(page);
//         self.data.insert(page_hash, v.to_vec());
//     }
//
//     fn get_page(&self, page: i32) -> Vec<u8> {
//         let page_hash = self.compute_page_hash(page);
//         let v = self.data.get(&page_hash);
//
//         if let Some(inner) = v {
//             inner.to_vec()
//         } else {
//             Vec::new()
//         }
//     }
//
//     #[inline(always)]
//     fn compute_page_hash(&self, page: i32) -> [u8; 32] {
//         DefaultPageHasher::hash(self.contract_addr, page as u32)
//     }
// }
