use svm_kv::traits::KVStore;

use svm_storage::page::PageIndex;
use svm_storage::testing::{contract_page_cache_init, default_page_index_hash};
use svm_storage::traits::PagesStorage;

#[test]
fn page_cache_loading_an_empty_page_into_the_cache() {
    let addr = 0x11_22_33_44;
    let pages_count = 10;

    let (_addr, _kv, mut cache) = contract_page_cache_init(addr, pages_count);

    assert_eq!(None, cache.read_page(PageIndex(0)));
}

macro_rules! assert_no_key {
    ($kv: expr, $key: expr) => {{
        assert!($kv.borrow().get(&$key).is_none());
    }};
}

macro_rules! assert_key_value {
    ($kv: expr, $key: expr, $expected: expr) => {{
        let actual = $kv.borrow().get(&$key).unwrap();
        assert_eq!($expected, &actual[..]);
    }};
}

#[test]
fn page_cache_write_page_and_then_commit() {
    let addr = 0x11_22_33_44;
    let pages_count = 10;

    let (_addr, kv, mut cache) = contract_page_cache_init(addr, pages_count);

    cache.write_page(PageIndex(0), &[10, 20, 30]);
    assert_eq!(vec![10, 20, 30], cache.read_page(PageIndex(0)).unwrap());

    let ph = default_page_index_hash(0x11_22_33_44, 0);
    assert_no_key!(kv, ph);
}

#[test]
fn page_cache_writing_a_page_marks_it_as_dirty() {
    let addr = 0x11_22_33_44;
    let pages_count = 10;

    let (_addr, _kv, mut cache) = contract_page_cache_init(addr, pages_count);

    assert_eq!(false, cache.is_dirty(0));
    cache.write_page(PageIndex(0), &[10, 20, 30]);
    assert_eq!(true, cache.is_dirty(0));
}

#[test]
#[ignore]
fn page_cache_commit_persists_each_dirty_page() {
    let addr = 0x11_22_33_44;
    let pages_count = 10;

    let (_addr, kv, mut cache) = contract_page_cache_init(addr, pages_count);

    cache.write_page(PageIndex(0), &[10, 20, 30]);

    // `cache.write_page` doesn't persist the page yet
    let ph = default_page_index_hash(0x11_22_33_44, 0);
    assert_no_key!(kv, ph);

    cache.commit();

    // `cache.commit` persists the page
    assert_key_value!(kv, ph, [10, 20, 30]);
}
