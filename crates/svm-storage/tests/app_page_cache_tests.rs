use svm_storage::{
    page::PageIndex,
    testing::{app_page_cache_init, default_page_hash},
    traits::PagesStorage,
};

mod asserts;

#[test]
fn page_cache_loading_an_empty_page_into_the_cache() {
    let page_count = 10;
    let (_kv, mut cache) = app_page_cache_init(page_count);

    assert_eq!(None, cache.read_page(PageIndex(0)));
}

#[test]
fn page_cache_write_page_and_then_commit() {
    let page_count = 10;
    let (kv, mut cache) = app_page_cache_init(page_count);

    cache.write_page(PageIndex(0), &[10, 20, 30]);
    assert_eq!(vec![10, 20, 30], cache.read_page(PageIndex(0)).unwrap());

    let ph = default_page_hash(&[10, 20, 30]);
    let key = ph.0;
    let ns = vec![b'p'];

    assert_no_key!(kv, ns, key);
}

#[test]
fn page_cache_writing_a_page_marks_it_as_dirty() {
    let page_count = 10;
    let (_kv, mut cache) = app_page_cache_init(page_count);

    assert_eq!(false, cache.is_dirty(0));
    cache.write_page(PageIndex(0), &[10, 20, 30]);
    assert_eq!(true, cache.is_dirty(0));
}

#[test]
#[ignore]
fn page_cache_commit_persists_each_dirty_page() {
    let page_count = 10;
    let (kv, mut cache) = app_page_cache_init(page_count);

    cache.write_page(PageIndex(0), &[10, 20, 30]);

    // `cache.write_page` doesn't persist the page yet
    let ph = default_page_hash(&[10, 20, 30]);
    let key = ph.0;
    let ns = vec![b'p'];

    assert_no_key!(kv, ns, key);
    cache.commit();
    assert_key_value!(kv, ns, key, [10, 20, 30]);
}
