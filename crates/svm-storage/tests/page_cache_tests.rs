// #[test]
// fn loading_an_empty_page_into_the_cache() {
//     page_cache_open!(cache, db, 0x11_22_33_44, 0x00_00_00_00, 10);
//
//     assert_eq!(None, cache.read_page(PageIndex(0)));
// }
//
// #[test]
// fn write_page_and_then_commit() {
//     page_cache_open!(cache, kv, 0x11_22_33_44, 0x00_00_00_00, 10);
//
//     let page = vec![10, 20, 30];
//
//     cache.write_page(PageIndex(0), &page);
//     assert_eq!(vec![10, 20, 30], cache.read_page(PageIndex(0)).unwrap());
//
//     let ph = default_page_idx_hash!(0x11_22_33_44, 0);
//     assert_eq!(None, kv.borrow().get(&ph));
// }
//
// #[test]
// #[ignore]
// fn writing_a_page_marks_it_as_dirty() {
//     page_cache_open!(cache, kv, 0x11_22_33_44, 0x00_00_00_00, 10);
//
//     assert_eq!(false, cache.is_dirty(0));
//
//     let page = vec![10, 20, 30];
//     cache.write_page(PageIndex(0), &page);
//
//     assert_eq!(true, cache.is_dirty(0));
// }
//
// #[test]
// #[ignore]
// fn commit_persists_each_dirty_page() {
//     page_cache_open!(cache, kv, 0x11_22_33_44, 0x00_00_00_00, 10);
//
//     let page = vec![10, 20, 30];
//
//     cache.write_page(PageIndex(0), &page);
//
//     // `cache.write_page` doesn't persist the page yet
//     let ph = default_page_idx_hash!(0x11_22_33_44, 0);
//     assert_eq!(None, kv.borrow().get(&ph));
//
//     cache.commit();
//
//     // `cache.commit` persists the page
//     assert_eq!(Some(vec![10, 20, 30]), kv.borrow().get(&ph));
// }
// }
