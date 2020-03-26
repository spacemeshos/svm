use svm_common::State;
use svm_kv::key::concat_ns_to_key;
use svm_storage::{
    page::{zero_page, PageIndex},
    testing::{
        app_pages_init, app_pages_open, compute_pages_state, concat_pages_hash, default_page_hash,
    },
    traits::{PagesStorage, StateAwarePagesStorage},
};

mod asserts;

#[test]
fn app_pages_first_time_run_with_no_modifications_no_commit() {
    let page_count = 3;
    let (_kv, mut pages) = app_pages_init(page_count);

    assert_eq!(0, pages.dirty_page_count());
    assert_eq!(None, pages.read_page(PageIndex(0)));
    assert_eq!(State::empty(), pages.get_state());
}

#[test]
fn app_pages_first_time_run_with_no_modifications_then_commit() {
    let page_count = 3;
    let (kv, mut pages) = app_pages_init(page_count);
    assert_eq!(0, pages.dirty_page_count());

    pages.commit();

    let ph = default_page_hash(&zero_page());

    let expected_state = compute_pages_state(&[ph, ph, ph]);
    let actual_state = pages.get_state();

    assert_eq!(expected_state, actual_state);

    let expected_key = concat_ns_to_key(&[b's'], expected_state.as_slice());
    assert_same_keys!(vec![expected_key], kv_keys_vec!(kv));

    let ns = vec![b'p'];
    let key = concat_ns_to_key(&ns, ph.0);
    assert_no_key!(kv, ns, key);

    assert_page_content!(pages, 0, None);
    assert_page_content!(pages, 1, None);
    assert_page_content!(pages, 2, None);

    assert_eq!(0, pages.dirty_page_count());
}

#[test]
fn app_pages_first_time_run_with_one_modified_page() {
    let page_count = 2;
    let (kv, mut pages) = app_pages_init(page_count);

    pages.write_page(PageIndex(0), &[10, 20, 30]);
    assert_eq!(1, pages.dirty_page_count());
    pages.commit();

    let ph0 = default_page_hash(&[10, 20, 30]);
    let ph1 = default_page_hash(&zero_page());

    let expected_state = compute_pages_state(&[ph0, ph1]);
    let actual_state = pages.get_state();
    assert_eq!(expected_state, actual_state);

    let state = expected_state.bytes();
    let page_ns = vec![b'p'];
    let state_ns = vec![b's'];
    let page_key = concat_ns_to_key(&page_ns, ph0.0);
    let state_key = concat_ns_to_key(&state_ns, state);
    let expected_keys = vec![page_key.clone(), state_key.clone()];

    assert_same_keys!(expected_keys, kv_keys_vec!(kv));
    assert_key_value!(kv, state_ns, state, concat_pages_hash(&[ph0, ph1]));
    assert_key_value!(kv, page_ns, ph0.0, [10, 20, 30]);

    assert_page_content!(pages, 0, Some(vec![10, 20, 30]));
    assert_page_content!(pages, 1, None);

    assert_eq!(0, pages.dirty_page_count());
}

#[test]
fn app_pages_first_time_run_with_two_modified_pages() {
    let page_count = 2;
    let (kv, mut pages) = app_pages_init(page_count);

    pages.write_page(PageIndex(0), &[10, 20, 30]);
    pages.write_page(PageIndex(1), &[40, 50, 60]);
    assert_eq!(2, pages.dirty_page_count());
    pages.commit();

    let ph0 = default_page_hash(&[10, 20, 30]);
    let ph1 = default_page_hash(&[40, 50, 60]);

    let expected_state = compute_pages_state(&[ph0, ph1]);
    let actual_state = pages.get_state();
    assert_eq!(expected_state, actual_state);

    let state = expected_state.bytes();
    let page_ns = vec![b'p'];
    let state_ns = vec![b's'];
    let page0_key = concat_ns_to_key(&page_ns, ph0.0);
    let page1_key = concat_ns_to_key(&page_ns, ph1.0);
    let state_key = concat_ns_to_key(&state_ns, state);
    let expected_keys = vec![page0_key.clone(), page1_key.clone(), state_key.clone()];

    assert_same_keys!(expected_keys, kv_keys_vec!(kv));
    assert_key_value!(kv, state_ns, state, concat_pages_hash(&[ph0, ph1]));
    assert_key_value!(kv, page_ns, ph0.0, [10, 20, 30]);
    assert_key_value!(kv, page_ns, ph1.0, [40, 50, 60]);

    assert_page_content!(pages, 0, Some(vec![10, 20, 30]));
    assert_page_content!(pages, 1, Some(vec![40, 50, 60]));

    assert_eq!(0, pages.dirty_page_count());
}

#[test]
fn app_pages_second_run_after_first_run_with_no_modifications() {
    // 1st run
    let page_count = 3;

    let (kv, mut pages) = app_pages_init(page_count);
    pages.commit();
    let old_state = pages.get_state();

    // 2nd run
    let mut pages = app_pages_open(&old_state, &kv, page_count);
    pages.write_page(PageIndex(0), &[10, 20, 30]);
    pages.write_page(PageIndex(1), &[40, 50, 60]);
    pages.commit();

    // modifying pages `0` and `1`
    let ph0 = default_page_hash(&[10, 20, 30]);
    let ph1 = default_page_hash(&[40, 50, 60]);
    let ph2 = default_page_hash(&zero_page());

    let expected_state = compute_pages_state(&[ph0, ph1, ph2]);
    let new_state = pages.get_state();
    assert_eq!(expected_state, new_state);

    let old_state = old_state.bytes();
    let new_state = new_state.bytes();
    let page_ns = vec![b'p'];
    let state_ns = vec![b's'];
    let old_state_key = concat_ns_to_key(&state_ns, old_state);
    let new_state_key = concat_ns_to_key(&state_ns, new_state);
    let page0_key = concat_ns_to_key(&page_ns, ph0.0);
    let page1_key = concat_ns_to_key(&page_ns, ph1.0);

    let expected_keys = vec![
        page0_key.clone(),
        page1_key.clone(),
        old_state_key.clone(),
        new_state_key.clone(),
    ];

    assert_same_keys!(expected_keys, kv_keys_vec!(kv));
    assert_key_value!(kv, state_ns, new_state, concat_pages_hash(&[ph0, ph1, ph2]));
    assert_key_value!(kv, page_ns, ph0.0, [10, 20, 30]);
    assert_key_value!(kv, page_ns, ph1.0, [40, 50, 60]);
    assert_no_key!(kv, page_ns, ph2.0);
}

#[test]
fn app_pages_second_run_after_first_run_with_modifications() {
    // 1st run
    let page_count = 3;
    let (kv, mut pages) = app_pages_init(page_count);

    pages.write_page(PageIndex(0), &[11, 22, 33]);
    pages.commit();
    let old_state = pages.get_state();

    // 2nd run
    let mut pages = app_pages_open(&old_state, &kv, page_count);
    pages.write_page(PageIndex(0), &[10, 20, 30]);
    pages.write_page(PageIndex(1), &[40, 50, 60]);
    pages.commit();

    // modifying pages `0` and `1`
    let ph0_old = default_page_hash(&[11, 22, 33]);
    let ph0 = default_page_hash(&[10, 20, 30]);
    let ph1 = default_page_hash(&[40, 50, 60]);
    let ph2 = default_page_hash(&zero_page());

    let expected_state = compute_pages_state(&[ph0, ph1, ph2]);
    let new_state = pages.get_state();
    assert_eq!(expected_state, new_state);

    let old_state = old_state.bytes();
    let new_state = new_state.bytes();
    let page_ns = vec![b'p'];
    let state_ns = vec![b's'];
    let old_state_key = concat_ns_to_key(&state_ns, old_state);
    let new_state_key = concat_ns_to_key(&state_ns, new_state);
    let page0_old_key = concat_ns_to_key(&page_ns, ph0_old.0);
    let page0_key = concat_ns_to_key(&page_ns, ph0.0);
    let page1_key = concat_ns_to_key(&page_ns, ph1.0);

    let expected_keys = vec![
        page0_old_key.clone(),
        page0_key.clone(),
        page1_key.clone(),
        old_state_key.clone(),
        new_state_key.clone(),
    ];

    assert_same_keys!(expected_keys, kv_keys_vec!(kv));
    assert_key_value!(kv, state_ns, new_state, concat_pages_hash(&[ph0, ph1, ph2]));
    assert_key_value!(kv, page_ns, ph0_old.0, [11, 22, 33]);
    assert_key_value!(kv, page_ns, ph0.0, [10, 20, 30]);
    assert_key_value!(kv, page_ns, ph1.0, [40, 50, 60]);
    assert_no_key!(kv, page_ns, ph2.0);
}

#[test]
fn app_pages_third_run_rollbacks_to_after_first_run() {
    // 1st run
    let page_count = 3;
    let (kv, mut pages) = app_pages_init(page_count);

    pages.write_page(PageIndex(0), &[11, 22, 33]);
    pages.commit();

    let state1 = pages.get_state();
    let ph0_1 = default_page_hash(&[11, 22, 33]);
    let ph1_1 = default_page_hash(&zero_page());
    let ph2_1 = default_page_hash(&zero_page());

    // 2nd run
    let mut pages = app_pages_open(&state1, &kv, page_count);
    pages.write_page(PageIndex(0), &[10, 20, 30]);
    pages.write_page(PageIndex(1), &[40, 50, 60]);
    pages.commit();
    let state2 = pages.get_state();

    // modifying pages `0` and `1`
    let ph0_2 = default_page_hash(&[10, 20, 30]);
    let ph1_2 = default_page_hash(&[40, 50, 60]);
    let ph2_2 = default_page_hash(&zero_page());

    // 3rd run (rollbacks to `state1` initial state)
    let pages = app_pages_open(&state1, &kv, page_count);

    let page_ns = vec![b'p'];
    let state_ns = vec![b's'];

    let state1_key = concat_ns_to_key(&state_ns, state1.bytes());
    let state2_key = concat_ns_to_key(&state_ns, state2.bytes());
    let page0_1_key = concat_ns_to_key(&page_ns, ph0_1.0);
    let page0_2_key = concat_ns_to_key(&page_ns, ph0_2.0);
    let page1_2_key = concat_ns_to_key(&page_ns, ph1_2.0);

    let expected_keys = vec![
        state1_key.clone(),
        state2_key.clone(),
        page0_1_key.clone(),
        page0_2_key.clone(),
        page1_2_key.clone(),
    ];

    assert_same_keys!(expected_keys, kv_keys_vec!(kv));

    let expected_state = compute_pages_state(&[ph0_1, ph1_1, ph2_1]);
    assert_eq!(expected_state, pages.get_state());

    assert_key_value!(
        kv,
        state_ns,
        state1.bytes(),
        concat_pages_hash(&[ph0_1, ph1_1, ph2_1])
    );

    // 4th run (rollbacks to `state2` state)
    let pages = app_pages_open(&state2, &kv, page_count);

    assert_same_keys!(expected_keys, kv_keys_vec!(kv));

    assert_key_value!(
        kv,
        state_ns,
        state2.bytes(),
        concat_pages_hash(&[ph0_2, ph1_2, ph2_2])
    );

    assert_eq!(state2, pages.get_state());
}
