mod asserts;

use svm_kv::traits::KVStore;
use svm_storage::{
    page::{zero_page, PageIndex, PageOffset, PageSliceLayout},
    testing::{app_storage_init, app_storage_open, default_page_hash, fill_page},
};

#[test]
fn app_storage_loading_an_empty_slice_into_the_cache() {
    let page_count = 10;
    let (_kv, mut storage) = app_storage_init(page_count);

    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 200);

    assert_eq!(vec![0; 200], storage.read_page_slice(&layout));
}

#[test]
fn app_storage_read_an_empty_slice_then_override_it_and_then_commit() {
    let page_count = 10;

    let (kv, mut storage) = app_storage_init(page_count);

    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);

    assert_eq!(vec![0, 0, 0], storage.read_page_slice(&layout));

    storage.write_page_slice(&layout, &vec![10, 20, 30]);

    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout));

    // page is not persisted though since we didn't `commit`
    let ph = default_page_hash(&[10, 20, 30]);
    let ns = vec![b'p'];
    let key = ph.0;

    assert_no_key!(kv, ns, key);
}

#[test]
fn app_storage_write_slice_without_loading_it_first_and_commit() {
    let page_count = 2;
    let (kv, mut storage) = app_storage_init(page_count);

    // page #1, cells: `100, 1001, 1002`
    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);

    storage.write_page_slice(&layout, &[10, 20, 30]);
    let new_state = storage.commit();

    // asserting persisted data. when viewed in the context of `new_state`.
    app_storage_open(&new_state, &kv, page_count);

    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout));

    let mut expected_page = zero_page();
    fill_page(&mut expected_page, &[(100, 10), (101, 20), (102, 30)]);

    let ph = default_page_hash(&expected_page);
    let ns = vec![b'p'];
    let key = ph.0;

    assert_key_value!(kv, ns, key, expected_page);
}

#[test]
fn app_storage_read_an_existing_slice_then_overriding_it_and_commit() {
    let page_count = 2;
    let ns = vec![b'p'];
    let (kv, mut storage) = app_storage_init(page_count);

    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);

    storage.write_page_slice(&layout, &vec![10, 20, 30]);
    let _ = storage.commit();

    let mut expected_page = zero_page();
    fill_page(&mut expected_page, &[(100, 10), (101, 20), (102, 30)]);
    let ph1 = default_page_hash(&expected_page);
    fill_page(&mut expected_page, &[(100, 40), (101, 50), (102, 60)]);
    let ph2 = default_page_hash(&expected_page);

    let page = kv.borrow().get(&ns, &ph1.0).unwrap();
    assert_eq!(vec![10, 20, 30], &page[100..103]);
    storage.write_page_slice(&layout, &vec![40, 50, 60]);

    // new page is on the page-storage, but not persisted yet
    assert_eq!(vec![40, 50, 60], storage.read_page_slice(&layout));

    let page = kv.borrow().get(&ns, &ph1.0).unwrap();
    assert_eq!(vec![10, 20, 30], &page[100..103]);

    assert_eq!(None, kv.borrow().get(&ns, &ph2.0));

    // now we also persist the new page version
    let _ = storage.commit();

    let page = kv.borrow().get(&ns, &ph2.0).unwrap();
    assert_eq!(vec![40, 50, 60], &page[100..103]);
}

#[test]
fn app_storage_write_slice_and_commit_then_load_it_override_it_and_commit() {
    let page_count = 2;
    let ns = vec![b'p'];
    let (kv, mut storage) = app_storage_init(page_count);

    let layout = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);

    let mut expected_page = zero_page();
    fill_page(&mut expected_page, &[(100, 10), (101, 20), (102, 30)]);
    let ph1 = default_page_hash(&expected_page);
    fill_page(&mut expected_page, &[(100, 40), (101, 50), (102, 60)]);
    let ph2 = default_page_hash(&expected_page);

    // 1) first page write
    storage.write_page_slice(&layout, &vec![10, 20, 30]);

    // 2) commit
    let state = storage.commit();

    // 3) re-load persisted page (we do a `clear` first to make sure we load from the pages-storage)
    let mut storage = app_storage_open(&state, &kv, page_count);
    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout));

    // 4) page override
    storage.write_page_slice(&layout, &vec![40, 50, 60]);
    assert_eq!(vec![40, 50, 60], storage.read_page_slice(&layout));

    // 5) commit again
    let page = kv.borrow().get(&ns, &ph1.0).unwrap();
    assert_eq!(vec![10, 20, 30], &page[100..103]);

    let _ = storage.commit();

    let page = kv.borrow().get(&ns, &ph2.0).unwrap();
    assert_eq!(vec![40, 50, 60], &page[100..103]);
}

#[test]
fn app_storage_write_two_slices_under_same_page_and_commit() {
    let page_count = 2;
    let (kv, mut storage) = app_storage_init(page_count);

    let layout1 = PageSliceLayout::new(PageIndex(1), PageOffset(100), 3);
    let layout2 = PageSliceLayout::new(PageIndex(1), PageOffset(200), 2);

    let mut expected_page = zero_page();
    fill_page(
        &mut expected_page,
        &[(100, 10), (101, 20), (102, 30), (200, 40), (201, 50)],
    );

    let ph = default_page_hash(&expected_page);

    storage.write_page_slice(&layout1, &vec![10, 20, 30]);
    storage.write_page_slice(&layout2, &vec![40, 50]);

    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout1));
    assert_eq!(vec![40, 50], storage.read_page_slice(&layout2));

    // commiting two slices under the same page
    let ns = vec![b'p'];
    let key = ph.0;
    assert_no_key!(kv, ns, key);

    let state = storage.commit();

    // asserting persisted data. when viewing in the context of `new_state`.
    let mut storage = app_storage_open(&state, &kv, page_count);

    assert_eq!(vec![10, 20, 30], storage.read_page_slice(&layout1));
    assert_eq!(vec![40, 50], storage.read_page_slice(&layout2));

    // querying the key-value store directly
    let page = kv.borrow().get(&ns, &ph.0).unwrap();
    assert_eq!(vec![10, 20, 30], &page[100..103]);
    assert_eq!(vec![40, 50], &page[200..202]);
}
