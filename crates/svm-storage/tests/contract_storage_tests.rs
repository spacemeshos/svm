// fn fill_page(page: &mut [u8], items: &[(usize, u8)]) {
//     for (i, b) in items {
//         page[*i] = *b;
//     }
// }
//
// macro_rules! contract_storage_open {
//     ($storage_ident: ident, $kv_ident: ident, $addr: expr, $state: expr, $max_pages: expr) => {
//         use crate::default::DefaultPageCache;
//         use crate::memory::MemContractPages;
//         use svm_kv::memory::MemKVStore;
//
//         use std::cell::RefCell;
//         use std::rc::Rc;
//
//         let $kv_ident = Rc::new(RefCell::new(MemKVStore::new()));
//         let kv_gen = || Rc::clone(&$kv_ident);
//
//         let pages = contract_pages_open!($addr, $state, kv_gen, $max_pages);
//         let cache = DefaultPageCache::<MemContractPages>::new(pages, $max_pages);
//
//         let mut $storage_ident = ContractStorage::new(Box::new(cache));
//     };
// }
//
// macro_rules! contract_pages_open {
//     ($addr: expr, $state: expr, $kv_gen: expr, $max_pages: expr) => {{
//         use crate::memory::MemContractPages;
//         use svm_common::{Address, State};
//
//         let addr = Address::from($addr as u32);
//         let state = State::from($state as u32);
//
//         MemContractPages::new(addr, $kv_gen(), state, $max_pages)
//     }};
// }
//
// macro_rules! reopen_pages_storage {
//     ($kv_ident: ident, $addr: expr, $state: expr, $max_pages: expr) => {{
//         use crate::memory::MemContractPages;
//         use svm_common::Address;
//
//         use std::rc::Rc;
//
//         let addr = Address::from($addr as u32);
//         MemContractPages::new(addr, Rc::clone(&$kv_ident), $state, $max_pages)
//     }};
// }
//
// macro_rules! contract_storage_reopen {
//     ($storage_ident: ident, $kv_ident: ident, $addr: expr, $state: expr, $max_pages: expr) => {
//         let pages = reopen_pages_storage!($kv_ident, $addr, $state, $max_pages);
//
//         let cache =
//             crate::default::DefaultPageCache::<MemContractPages>::new(pages, $max_pages);
//
//         let mut $storage_ident = ContractStorage::new(Box::new(cache));
//     };
// }
//
// #[test]
// fn loading_an_empty_slice_into_the_cache() {
//     contract_storage_open!(cache, kv, 0x11_22_33_44, 0x00_00_00_00, 10);
//
//     let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 200);
//
//     assert_eq!(vec![0; 200], cache.read_page_slice(&layout));
// }
//
// #[test]
// fn read_an_empty_slice_then_override_it_and_then_commit() {
//     contract_storage_open!(cache, kv, 0x11_22_33_44, 0x00_00_00_00, 10);
//
//     let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);
//
//     assert_eq!(vec![0, 0, 0], cache.read_page_slice(&layout));
//
//     cache.write_page_slice(&layout, &vec![10, 20, 30]);
//
//     assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout));
//
//     // page is not persisted though since we didn't `commit`
//     let ph = default_page_hash!(0x11_22_33_44, 0, &[10, 20, 30]);
//     assert_eq!(None, kv.borrow().get(&ph.0));
// }
//
// #[test]
// fn write_slice_without_loading_it_first_and_commit() {
//     let addr = 0x11_22_33_44;
//     contract_storage_open!(cache, kv, addr, 0x00_00_00_00, 2);
//
//     let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);
//
//     cache.write_page_slice(&layout, &[10, 20, 30]);
//     let new_state = cache.commit();
//
//     // asserting persisted data. when viewing in the context of `new_state`.
//     contract_storage_reopen!(cache, kv, addr, new_state, 2);
//
//     assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout));
//
//     let mut expected_page = page::zero_page();
//     fill_page(&mut expected_page, &[(100, 10), (101, 20), (102, 30)]);
//
//     let ph = default_page_hash!(addr, 1, &expected_page);
//     let actual_page = kv.borrow().get(&ph.0).unwrap();
//
//     assert_eq!(expected_page, actual_page);
// }
//
// #[test]
// fn read_an_existing_slice_then_overriding_it_and_commit() {
//     let addr = 0x11_22_33_44;
//     contract_storage_open!(cache, kv, addr, 0x00_00_00_00, 2);
//
//     let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);
//
//     cache.write_page_slice(&layout, &vec![10, 20, 30]);
//     let _ = cache.commit();
//
//     let mut expected_page = page::zero_page();
//     fill_page(&mut expected_page, &[(100, 10), (101, 20), (102, 30)]);
//     let ph1 = default_page_hash!(addr, 1, &expected_page);
//     fill_page(&mut expected_page, &[(100, 40), (101, 50), (102, 60)]);
//     let ph2 = default_page_hash!(addr, 1, &expected_page);
//
//     let page = kv.borrow().get(&ph1.0).unwrap();
//     assert_eq!(vec![10, 20, 30], &page[100..103]);
//     &cache.write_page_slice(&layout, &vec![40, 50, 60]);
//
//     // new page is on the page-cache, but not persisted yet
//     assert_eq!(vec![40, 50, 60], cache.read_page_slice(&layout));
//
//     let page = kv.borrow().get(&ph1.0).unwrap();
//     assert_eq!(vec![10, 20, 30], &page[100..103]);
//
//     assert_eq!(None, kv.borrow().get(&ph2.0));
//
//     // now we also persist the new page version
//     let _ = cache.commit();
//
//     let page = kv.borrow().get(&ph2.0).unwrap();
//     assert_eq!(vec![40, 50, 60], &page[100..103]);
// }
//
// #[test]
// fn write_slice_and_commit_then_load_it_override_it_and_commit() {
//     let addr = 0x11_22_33_44;
//     contract_storage_open!(cache, kv, addr, 0x00_00_00_00, 2);
//
//     let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);
//
//     let mut expected_page = page::zero_page();
//     fill_page(&mut expected_page, &[(100, 10), (101, 20), (102, 30)]);
//     let ph1 = default_page_hash!(addr, 1, &expected_page);
//     fill_page(&mut expected_page, &[(100, 40), (101, 50), (102, 60)]);
//     let ph2 = default_page_hash!(addr, 1, &expected_page);
//
//     // 1) first page write
//     cache.write_page_slice(&layout, &vec![10, 20, 30]);
//
//     // 2) commit
//     let _ = cache.commit();
//
//     // 3) load persisted page (we do a `clear` first to make sure we load from the page cache)
//     cache.clear();
//
//     assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout));
//
//     // 4) page override
//     cache.write_page_slice(&layout, &vec![40, 50, 60]);
//     assert_eq!(vec![40, 50, 60], cache.read_page_slice(&layout));
//
//     // 5) commit again
//     let page = kv.borrow().get(&ph1.0).unwrap();
//     assert_eq!(vec![10, 20, 30], &page[100..103]);
//
//     let _ = cache.commit();
//
//     let page = kv.borrow().get(&ph2.0).unwrap();
//     assert_eq!(vec![40, 50, 60], &page[100..103]);
// }
//
// #[test]
// fn write_two_slices_under_same_page_and_commit() {
//     let addr = 0x11_22_33_44;
//     contract_storage_open!(cache, kv, addr, 0x00_00_00_00, 2);
//
//     let layout1 = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);
//
//     let layout2 = PageSliceLayout::new(PageIndex(1), PageOffset(200), 2);
//
//     let mut expected_page = page::zero_page();
//     fill_page(
//         &mut expected_page,
//         &[(100, 10), (101, 20), (102, 30), (200, 40), (201, 50)],
//         );
//
//     let ph = default_page_hash!(addr, 1, &expected_page);
//
//     cache.write_page_slice(&layout1, &vec![10, 20, 30]);
//     cache.write_page_slice(&layout2, &vec![40, 50]);
//
//     assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout1));
//     assert_eq!(vec![40, 50], cache.read_page_slice(&layout2));
//
//     // commiting two slices under the same page
//     assert_eq!(None, kv.borrow().get(&ph.0));
//
//     let new_state = cache.commit();
//
//     // asserting persisted data. when viewing in the context of `new_state`.
//     contract_storage_reopen!(cache, kv, addr, new_state, 2);
//
//     assert_eq!(vec![10, 20, 30], cache.read_page_slice(&layout1));
//     assert_eq!(vec![40, 50], cache.read_page_slice(&layout2));
//
//     // queryind the key-value store directly
//     let page = kv.borrow().get(&ph.0).unwrap();
//     assert_eq!(vec![10, 20, 30], &page[100..103]);
//     assert_eq!(vec![40, 50], &page[200..202]);
// }
// }
