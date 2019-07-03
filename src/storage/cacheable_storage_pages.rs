use super::traits::{KVStore, StoragePageHasher, StoragePages};
use super::StoragePagesImpl;
use crate::Address;

type Page = Vec<u8>;

type PageKey = [u8; 32];

pub struct CacheableStoragePages<SPH: StoragePageHasher, KV: KVStore<K = PageKey>> {
    /// The `ith` item will say whether the `ith-page` is dirty
    dirty_pages: Vec<bool>,

    /// The `ith` item will say whether the `ith-page` is cached
    cached_pages: Vec<Option<Page>>,

    /// The underlying database that interacts with the storage
    pages_storage: StoragePagesImpl<SPH, KV>,
}

// impl<SPH: StoragePageHasher, KV: KVStore<K = PageKey>> CacheableStoragePages<SPH, KV> {
//     fn new(pages_storage: StoragePagesImpl<SPH, KV>, max_pages: usize) -> Self {
//         Self {
//             dirty_pages: vec![false; max_pages],
//
//             cached_pages: vec![None; max_pages],
//
//             pages_storage,
//         }
//     }
// }
//
// impl<SPH: StoragePageHasher, KV: KVStore<K = PageKey>> StoragePages
//     for CacheableStoragePages<SPH, KV>
// {
//     fn read_page(&mut self, page_idx: u32) -> Vec<u8> {
//         // we are given the maximum storage-pages upon initialization
//         assert!(self.cached_pages.len() > page_idx as usize);
//
//         let page = &self.cached_pages[page_idx as usize];
//
//         if page.is_some() {
//             page.as_ref().unwrap().to_vec()
//         } else {
//             let page = self.pages_storage.read_page(page_idx);
//
//             std::mem::replace(
//                 &mut self.cached_pages[page_idx as usize],
//                 Some(page.clone()),
//             );
//
//             page
//         }
//     }
//
//     fn write_page(&mut self, page: u32, data: &[u8]) {
//         // make sure pages is read
//         // mark as dirty
//     }
//
//     fn clear(&mut self) {}
//
//     fn commit(&mut self) {}
// }
