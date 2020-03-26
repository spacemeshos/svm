use svm_common::Address;
use svm_kv::{memory::MemKVStore, traits::KVStore};

mod asserts;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn key_store_keys_do_not_exit_by_default() {
    init();

    let addr = Address::of("@someone");

    let kv: MemKVStore = MemKVStore::new();
    let ns = vec![0xFF, 0xFF];
    let key = addr.as_slice();

    assert_no_key!(kv, ns, key);
}

#[test]
fn key_store_and_then_key_get() {
    init();

    let addr = Address::of("someone");

    let mut kv = MemKVStore::new();
    let ns = vec![0xFF, 0xFF];
    let key = addr.as_slice();
    let val = vec![10, 20, 30];

    let change = (&ns[..], &key[..], &val[..]);
    kv.store(&[change]);

    assert_key_value!(kv, ns, key, val);
}

#[test]
fn key_store_override_existing_entry() {
    init();

    let mut kv = MemKVStore::new();
    let addr = Address::of("someone");

    let ns = vec![0xFF, 0xFF];
    let key = addr.as_slice();
    let val1 = vec![10, 20, 30];
    let val2 = vec![40, 50, 60];

    let change = (&ns[..], &key[..], &val1[..]);
    kv.store(&[change]);
    assert_key_value!(kv, ns, key, val1);

    let change = (&ns[..], &key[..], &val2[..]);
    kv.store(&[change]);
    assert_key_value!(kv, ns, key, val2);
}

#[test]
fn clear() {
    init();

    let mut kv = MemKVStore::new();
    let addr1 = Address::of("Alice");
    let addr2 = Address::of("Bob");

    let key1 = addr1.as_slice();
    let key2 = addr2.as_slice();

    let val1 = vec![10, 20, 30];
    let val2 = vec![40, 50, 60];
    let ns = vec![0xFF, 0xFF];

    let changes = [
        (&ns[..], &key1[..], &val1[..]),
        (&ns[..], &key2[..], &val2[..]),
    ];

    kv.store(&changes);

    assert_key_value!(kv, ns, key1, val1);
    assert_key_value!(kv, ns, key2, val2);

    kv.clear();

    assert_no_key!(kv, ns, key1);
    assert_no_key!(kv, ns, key2);
}
