extern crate svm_storage;

use std::cell::RefCell;
use std::rc::Rc;

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;

use svm_storage::page::{zero_page, PageIndex};
use svm_storage::testing::{compute_pages_state, contract_pages_init, default_page_hash};
use svm_storage::traits::{PagesStorage, StateAwarePagesStorage};

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

    let (addr, _kv, mut pages) = contract_pages_init(addr, pages_count);
    assert_eq!(0, pages.dirty_pages_count());
    pages.commit();

    let ph0 = default_page_hash(&addr, 0, &zero_page());
    let ph1 = default_page_hash(&addr, 1, &zero_page());
    let ph2 = default_page_hash(&addr, 2, &zero_page());
    let expected_state = compute_pages_state(&[ph0, ph1, ph2]);

    assert_eq!(expected_state, pages.get_state());

    // assert_same_keys!(vec![state.bytes()], kv_keys_vec!(kv));
    //
    // assert_no_key!(&kv, ph0.0);
    // assert_no_key!(&kv, ph1.0);
    // assert_no_key!(&kv, ph2.0);
    // assert_page!(storage, 0, None);
    // assert_page!(storage, 1, None);
    // assert_page!(storage, 2, None);
    // assert_dirty_pages_count!(storage, 0);
}

// #[test]
// fn first_time_run_with_one_modified_page() {
//     contract_pages_open!(0x11_22_33_44, addr, storage, kv, 3);
//
//     storage.write_page(PageIndex(0), &[10, 20, 30]);
//     assert_dirty_pages_count!(storage, 1);
//     storage.commit();
//
//     let ph0 = compute_page_hash!(addr, 0, &[10, 20, 30]);
//     let ph1 = compute_page_hash!(addr, 1, &zero_page());
//     let ph2 = compute_page_hash!(addr, 2, &zero_page());
//     let jph = join_pages_hash!(&[ph0, ph1, ph2]);
//     let state = compute_state!(jph);
//
//     assert_state!(state, storage);
//     assert_same_keys!(vec![state.bytes(), ph0.0], kv_keys_vec!(kv));
//     assert_key_value!(kv, state.bytes(), jph);
//     assert_key_value!(kv, ph0.0, [10, 20, 30]);
//     assert_page!(storage, 0, Some(vec![10, 20, 30]));
//     assert_page!(storage, 1, None);
//     assert_dirty_pages_count!(storage, 0);
// }
//
// #[test]
// fn first_time_run_with_two_modified_pages() {
//     contract_pages_open!(0x11_22_33_44, addr, storage, kv, 2);
//
//     storage.write_page(PageIndex(0), &[10, 20, 30]);
//     storage.write_page(PageIndex(1), &[40, 50, 60]);
//     assert_dirty_pages_count!(storage, 2);
//     storage.commit();
//
//     let ph0 = compute_page_hash!(addr, 0, &[10, 20, 30]);
//     let ph1 = compute_page_hash!(addr, 1, &[40, 50, 60]);
//     let jph = join_pages_hash!(&[ph0, ph1]);
//     let state = compute_state!(jph);
//
//     assert_state!(state, storage);
//     assert_same_keys!(vec![state.bytes(), ph0.0, ph1.0], kv_keys_vec!(kv));
//     assert_key_value!(kv, state.bytes(), jph);
//     assert_key_value!(kv, ph0.0, [10, 20, 30]);
//     assert_key_value!(kv, ph1.0, [40, 50, 60]);
//     assert_page!(storage, 0, Some(vec![10, 20, 30]));
//     assert_page!(storage, 1, Some(vec![40, 50, 60]));
//     assert_dirty_pages_count!(storage, 0);
// }
//
// #[test]
// fn second_run_after_first_run_with_no_modifications() {
//     // 1st run
//     contract_pages_open!(0x11_22_33_44, addr, storage, kv, 3);
//     storage.commit();
//     let old_state = storage.get_state();
//
//     // 2nd run
//     init_pages_storage!(0x11_22_33_44, addr, storage, kv, old_state, 3);
//     storage.write_page(PageIndex(0), &[10, 20, 30]);
//     storage.write_page(PageIndex(1), &[40, 50, 60]);
//     storage.commit();
//
//     // modifying pages `0` and `1`
//     let ph0 = compute_page_hash!(addr, 0, &[10, 20, 30]);
//     let ph1 = compute_page_hash!(addr, 1, &[40, 50, 60]);
//     let ph2 = compute_page_hash!(addr, 2, &zero_page());
//     let jph = join_pages_hash!(&[ph0, ph1, ph2]);
//     let new_state = compute_state!(jph);
//
//     assert_same_keys!(
//         vec![old_state.bytes(), new_state.bytes(), ph0.0, ph1.0],
//         kv_keys_vec!(kv)
//     );
//
//     assert_key_value!(kv, new_state.bytes(), jph);
//     assert_key_value!(kv, ph0.0, [10, 20, 30]);
//     assert_key_value!(kv, ph1.0, [40, 50, 60]);
//     assert_no_key!(kv, ph2.0);
// }
//
// #[test]
// fn second_run_after_first_run_with_modifications() {
//     // 1st run
//     contract_pages_open!(0x11_22_33_44, addr, storage, kv, 3);
//     storage.write_page(PageIndex(0), &[11, 22, 33]);
//     storage.commit();
//     let old_state = storage.get_state();
//
//     // 2nd run
//     init_pages_storage!(0x11_22_33_44, addr, storage, kv, old_state, 3);
//     storage.write_page(PageIndex(0), &[10, 20, 30]);
//     storage.write_page(PageIndex(1), &[40, 50, 60]);
//     storage.commit();
//
//     // modifying pages `0` and `1`
//     let ph0_old = compute_page_hash!(addr, 0, &[11, 22, 33]);
//     let ph0 = compute_page_hash!(addr, 0, &[10, 20, 30]);
//     let ph1 = compute_page_hash!(addr, 1, &[40, 50, 60]);
//     let ph2 = compute_page_hash!(addr, 2, &zero_page());
//     let jph = join_pages_hash!(&[ph0, ph1, ph2]);
//     let new_state = compute_state!(jph);
//
//     assert_same_keys!(
//         vec![
//             old_state.bytes(),
//             new_state.bytes(),
//             ph0_old.0,
//             ph0.0,
//             ph1.0
//         ],
//         kv_keys_vec!(kv)
//     );
//
//     assert_key_value!(kv, new_state.bytes(), jph);
//     assert_key_value!(kv, ph0.0, [10, 20, 30]);
//     assert_key_value!(kv, ph1.0, [40, 50, 60]);
//     assert_no_key!(kv, ph2.0);
// }
//
// #[test]
// fn third_run_rollbacks_to_after_first_run() {
//     // 1st run
//     contract_pages_open!(0x11_22_33_44, addr, storage, kv, 3);
//     storage.write_page(PageIndex(0), &[11, 22, 33]);
//     storage.commit();
//     let state_1 = storage.get_state();
//
//     // 2nd run
//     init_pages_storage!(0x11_22_33_44, addr, storage, kv, state_1, 3);
//     storage.write_page(PageIndex(0), &[10, 20, 30]);
//     storage.write_page(PageIndex(1), &[40, 50, 60]);
//     storage.commit();
//     let state_2 = storage.get_state();
//
//     // 3rd run (rollbacks to `state_1` initial state)
//     init_pages_storage!(0x11_22_33_44, addr, storage, kv, state_1, 3);
//
//     let ph0_1 = compute_page_hash!(addr, 0, &[11, 22, 33]);
//     let ph1_1 = compute_page_hash!(addr, 1, &zero_page());
//     let ph2_1 = compute_page_hash!(addr, 2, &zero_page());
//
//     let ph0_2 = compute_page_hash!(addr, 0, &[10, 20, 30]);
//     let ph1_2 = compute_page_hash!(addr, 1, &[40, 50, 60]);
//     let ph2_2 = compute_page_hash!(addr, 2, &zero_page());
//     let jph = join_pages_hash!(&[ph0_1, ph1_1, ph2_1]);
//
//     assert_same_keys!(
//         vec![state_1.bytes(), state_2.bytes(), ph0_1.0, ph0_2.0, ph1_2.0],
//         kv_keys_vec!(kv)
//     );
//
//     assert_state!(state_1, storage);
//     assert_key_value!(kv, state_1.bytes(), jph);
//
//     // 4th run (rollbacks to `state_2` initial state)
//     init_pages_storage!(0x11_22_33_44, addr, storage, kv, state_2, 3);
//     let jph = join_pages_hash!(&[ph0_2, ph1_2, ph2_2]);
//
//     assert_same_keys!(
//         vec![state_1.bytes(), state_2.bytes(), ph0_1.0, ph0_2.0, ph1_2.0],
//         kv_keys_vec!(kv)
//     );
//
//     assert_key_value!(kv, state_2.bytes(), jph);
//     assert_state!(state_2, storage);
// }
