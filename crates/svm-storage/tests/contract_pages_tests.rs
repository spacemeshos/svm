extern crate svm_storage;

use std::cell::RefCell;
use std::rc::Rc;

use svm_common::{Address, State};

use svm_kv::memory::MemKVStore;
use svm_kv::traits::KVStore;

use svm_storage::page::{zero_page, PageIndex};
use svm_storage::testing::{
    compute_pages_state, concat_pages_hash, contract_pages_init, contract_pages_open,
    default_page_hash,
};
use svm_storage::traits::{PagesStorage, StateAwarePagesStorage};

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

macro_rules! assert_page_content {
    ($pages: ident, $page_idx: expr, $expected: expr) => {{
        assert_eq!($expected, $pages.read_page(PageIndex($page_idx)));
    }};
}

macro_rules! kv_keys_vec {
    ($kv: ident) => {{
        let keys: Vec<Vec<u8>> = $kv.borrow().keys().map(|key| key.clone()).collect();
        keys
    }};
}

macro_rules! assert_same_keys {
    ($expected: expr, $actual: expr) => {{
        let mut expected = $expected
            .iter()
            .map(|k| k.to_vec())
            .collect::<Vec<Vec<u8>>>();
        let mut actual = $actual.to_vec();

        expected.sort();
        actual.sort();

        assert_eq!(&expected[..], &actual[..]);
    }};
}

#[test]
fn first_time_run_with_no_modifications_no_commit() {
    let pages_count = 3;
    let addr = 0x11_22_33_44;

    let (_addr, _kv, mut pages) = contract_pages_init(addr, pages_count);

    assert_eq!(0, pages.dirty_pages_count());
    assert_eq!(None, pages.read_page(PageIndex(0)));
    assert_eq!(State::empty(), pages.get_state());
}

#[test]
fn first_time_run_with_no_modifications_then_commit() {
    let pages_count = 3;
    let addr = 0x11_22_33_44;

    let (addr, kv, mut pages) = contract_pages_init(addr, pages_count);
    assert_eq!(0, pages.dirty_pages_count());

    pages.commit();

    let ph0 = default_page_hash(&addr, 0, &zero_page());
    let ph1 = default_page_hash(&addr, 1, &zero_page());
    let ph2 = default_page_hash(&addr, 2, &zero_page());

    let expected_state = compute_pages_state(&[ph0, ph1, ph2]);
    let actual_state = pages.get_state();

    assert_eq!(expected_state, actual_state);

    assert_same_keys!(vec![actual_state.bytes()], kv_keys_vec!(kv));

    assert_no_key!(&kv, ph0.0);
    assert_no_key!(&kv, ph1.0);
    assert_no_key!(&kv, ph2.0);

    assert_page_content!(pages, 0, None);
    assert_page_content!(pages, 1, None);
    assert_page_content!(pages, 2, None);

    assert_eq!(0, pages.dirty_pages_count());
}

#[test]
fn first_time_run_with_one_modified_page() {
    let pages_count = 3;
    let addr = 0x11_22_33_44;

    let (addr, kv, mut pages) = contract_pages_init(addr, pages_count);

    pages.write_page(PageIndex(0), &[10, 20, 30]);
    assert_eq!(1, pages.dirty_pages_count());
    pages.commit();

    let ph0 = default_page_hash(&addr, 0, &[10, 20, 30]);
    let ph1 = default_page_hash(&addr, 1, &zero_page());
    let ph2 = default_page_hash(&addr, 2, &zero_page());

    let expected_state = compute_pages_state(&[ph0, ph1, ph2]);
    let actual_state = pages.get_state();
    assert_eq!(expected_state, actual_state);

    assert_same_keys!(vec![actual_state.bytes(), ph0.0], kv_keys_vec!(kv));

    assert_key_value!(
        kv,
        actual_state.bytes(),
        concat_pages_hash(&[ph0, ph1, ph2])
    );
    assert_key_value!(kv, ph0.0, [10, 20, 30]);

    assert_page_content!(pages, 0, Some(vec![10, 20, 30]));
    assert_page_content!(pages, 1, None);

    assert_eq!(0, pages.dirty_pages_count());
}

#[test]
fn first_time_run_with_two_modified_pages() {
    let pages_count = 2;
    let addr = 0x11_22_33_44;

    let (addr, kv, mut pages) = contract_pages_init(addr, pages_count);

    pages.write_page(PageIndex(0), &[10, 20, 30]);
    pages.write_page(PageIndex(1), &[40, 50, 60]);
    assert_eq!(2, pages.dirty_pages_count());
    pages.commit();

    let ph0 = default_page_hash(&addr, 0, &[10, 20, 30]);
    let ph1 = default_page_hash(&addr, 1, &[40, 50, 60]);

    let expected_state = compute_pages_state(&[ph0, ph1]);
    let actual_state = pages.get_state();
    assert_eq!(expected_state, actual_state);

    assert_same_keys!(vec![actual_state.bytes(), ph0.0, ph1.0], kv_keys_vec!(kv));
    assert_key_value!(kv, actual_state.bytes(), concat_pages_hash(&[ph0, ph1]));
    assert_key_value!(kv, ph0.0, [10, 20, 30]);
    assert_key_value!(kv, ph1.0, [40, 50, 60]);
    assert_page_content!(pages, 0, Some(vec![10, 20, 30]));
    assert_page_content!(pages, 1, Some(vec![40, 50, 60]));
    assert_eq!(0, pages.dirty_pages_count());
}

#[test]
fn second_run_after_first_run_with_no_modifications() {
    // 1st run
    let pages_count = 3;
    let addr = 0x11_22_33_44;

    let (addr, kv, mut pages) = contract_pages_init(addr, pages_count);
    pages.commit();
    let old_state = pages.get_state();

    // 2nd run
    let mut pages = contract_pages_open(&addr, &old_state, &kv, pages_count);
    pages.write_page(PageIndex(0), &[10, 20, 30]);
    pages.write_page(PageIndex(1), &[40, 50, 60]);
    pages.commit();

    // modifying pages `0` and `1`
    let ph0 = default_page_hash(&addr, 0, &[10, 20, 30]);
    let ph1 = default_page_hash(&addr, 1, &[40, 50, 60]);
    let ph2 = default_page_hash(&addr, 2, &zero_page());

    let expected_state = compute_pages_state(&[ph0, ph1, ph2]);
    let new_state = pages.get_state();
    assert_eq!(expected_state, new_state);

    assert_same_keys!(
        vec![old_state.bytes(), new_state.bytes(), ph0.0, ph1.0],
        kv_keys_vec!(kv)
    );

    assert_key_value!(kv, new_state.bytes(), concat_pages_hash(&[ph0, ph1, ph2]));
    assert_key_value!(kv, ph0.0, [10, 20, 30]);
    assert_key_value!(kv, ph0.0, [10, 20, 30]);
    assert_key_value!(kv, ph1.0, [40, 50, 60]);
    assert_no_key!(kv, ph2.0);
}

#[test]
fn second_run_after_first_run_with_modifications() {
    // 1st run
    let pages_count = 3;
    let addr = 0x11_22_33_44;

    let (addr, kv, mut pages) = contract_pages_init(addr, pages_count);

    pages.write_page(PageIndex(0), &[11, 22, 33]);
    pages.commit();
    let old_state = pages.get_state();

    // 2nd run
    let mut pages = contract_pages_open(&addr, &old_state, &kv, pages_count);
    pages.write_page(PageIndex(0), &[10, 20, 30]);
    pages.write_page(PageIndex(1), &[40, 50, 60]);
    pages.commit();

    // modifying pages `0` and `1`
    let ph0_old = default_page_hash(&addr, 0, &[11, 22, 33]);
    let ph0 = default_page_hash(&addr, 0, &[10, 20, 30]);
    let ph1 = default_page_hash(&addr, 1, &[40, 50, 60]);
    let ph2 = default_page_hash(&addr, 2, &zero_page());

    let expected_state = compute_pages_state(&[ph0, ph1, ph2]);
    let new_state = pages.get_state();
    assert_eq!(expected_state, new_state);

    assert_same_keys!(
        vec![
            old_state.bytes(),
            new_state.bytes(),
            ph0_old.0,
            ph0.0,
            ph1.0
        ],
        kv_keys_vec!(kv)
    );

    assert_key_value!(kv, new_state.bytes(), concat_pages_hash(&[ph0, ph1, ph2]));
    assert_key_value!(kv, ph0.0, [10, 20, 30]);
    assert_key_value!(kv, ph1.0, [40, 50, 60]);
    assert_no_key!(kv, ph2.0);
}

#[test]
fn third_run_rollbacks_to_after_first_run() {
    // 1st run
    let pages_count = 3;
    let addr = 0x11_22_33_44;

    let (addr, kv, mut pages) = contract_pages_init(addr, pages_count);

    pages.write_page(PageIndex(0), &[11, 22, 33]);
    pages.commit();
    let state_1 = pages.get_state();

    let ph0_1 = default_page_hash(&addr, 0, &[11, 22, 33]);
    let ph1_1 = default_page_hash(&addr, 1, &zero_page());
    let ph2_1 = default_page_hash(&addr, 2, &zero_page());

    // 2nd run
    let mut pages = contract_pages_open(&addr, &state_1, &kv, pages_count);
    pages.write_page(PageIndex(0), &[10, 20, 30]);
    pages.write_page(PageIndex(1), &[40, 50, 60]);
    pages.commit();
    let state_2 = pages.get_state();

    // modifying pages `0` and `1`
    let ph0_2 = default_page_hash(&addr, 0, &[10, 20, 30]);
    let ph1_2 = default_page_hash(&addr, 1, &[40, 50, 60]);
    let ph2_2 = default_page_hash(&addr, 2, &zero_page());

    // 3rd run (rollbacks to `state_1` initial state)
    let pages = contract_pages_open(&addr, &state_1, &kv, pages_count);

    assert_same_keys!(
        vec![state_1.bytes(), state_2.bytes(), ph0_1.0, ph0_2.0, ph1_2.0],
        kv_keys_vec!(kv)
    );

    let expected_state = compute_pages_state(&[ph0_1, ph1_1, ph2_1]);
    assert_eq!(expected_state, pages.get_state());

    assert_key_value!(
        kv,
        state_1.bytes(),
        concat_pages_hash(&[ph0_1, ph1_1, ph2_1])
    );

    // 4th run (rollbacks to `state_2` state)
    let pages = contract_pages_open(&addr, &state_2, &kv, pages_count);

    assert_same_keys!(
        vec![state_1.bytes(), state_2.bytes(), ph0_1.0, ph0_2.0, ph1_2.0],
        kv_keys_vec!(kv)
    );

    assert_key_value!(
        kv,
        state_2.bytes(),
        concat_pages_hash(&[ph0_2, ph1_2, ph2_2])
    );

    assert_eq!(state_2, pages.get_state());
}
