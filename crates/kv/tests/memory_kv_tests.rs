use svm_types::Address;

use svm_kv::memory::MemRawKV;
use svm_kv::traits::RawKV;

mod asserts;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn raw_kv_keys_do_not_exit_by_default() {
    init();

    let addr = Address::of("@someone");

    let kv: MemRawKV = MemRawKV::new();
    let key = addr.as_slice();

    assert_no_key!(kv, key);
}

#[test]
fn raw_kv_set_and_then_key_get() {
    init();

    let addr = Address::of("someone");

    let mut kv = MemRawKV::new();
    let key = addr.as_slice();
    let val = vec![10, 20, 30];

    let change = (&key[..], &val[..]);
    kv.set(&[change]);

    assert_key_value!(kv, key, val);
}

#[test]
fn key_store_override_existing_entry() {
    init();

    let mut kv = MemRawKV::new();
    let addr = Address::of("someone");

    let key = addr.as_slice();
    let val1 = vec![10, 20, 30];
    let val2 = vec![40, 50, 60];

    let change = (&key[..], &val1[..]);
    kv.set(&[change]);
    assert_key_value!(kv, key, val1);

    let change = (&key[..], &val2[..]);
    kv.set(&[change]);
    assert_key_value!(kv, key, val2);
}

#[test]
fn clear() {
    init();

    let mut kv = MemRawKV::new();
    let addr1 = Address::of("Alice");
    let addr2 = Address::of("Bob");

    let key1 = addr1.as_slice();
    let key2 = addr2.as_slice();

    let val1 = vec![10, 20, 30];
    let val2 = vec![40, 50, 60];

    let changes = [(&key1[..], &val1[..]), (&key2[..], &val2[..])];

    kv.set(&changes);

    assert_key_value!(kv, key1, val1);
    assert_key_value!(kv, key2, val2);

    kv.clear();

    assert_no_key!(kv, key1);
    assert_no_key!(kv, key2);
}
