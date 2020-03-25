use svm_common::Address;
use svm_kv::{memory::MemKVStore, traits::KVStore};

mod asserts;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn key_store_keys_do_not_exit_by_default() {
    init();

    let kv = MemKVStore::new();
    let addr = Address::of("@someone");
    let ns = vec![0xFF, 0xFF];

    assert_no_key!(kv, ns, addr.as_slice());
}

#[test]
fn key_store_and_then_key_get() {
    init();

    let mut kv = MemKVStore::new();
    let addr = Address::of("someone");
    let ns = vec![0xFF, 0xFF];

    kv.store(&ns, &[(addr.as_slice(), &[10, 20, 30])]);

    assert_key_value!(kv, ns, addr.as_slice(), vec![10, 20, 30]);
}

#[test]
fn key_store_override_existing_entry() {
    init();

    let mut kv = MemKVStore::new();
    let addr = Address::of("someone");
    let ns = vec![0xFF, 0xFF];

    kv.store(&ns, &[(addr.as_slice(), &[10, 20, 30])]);
    assert_key_value!(kv, ns, addr.as_slice(), vec![10, 20, 30]);

    kv.store(&ns, &[(addr.as_slice(), &[40, 50, 60])]);
    assert_key_value!(kv, ns, addr.as_slice(), vec![40, 50, 60]);
}

#[test]
fn clear() {
    init();

    let mut kv = MemKVStore::new();
    let addr1 = Address::of("Alice");
    let addr2 = Address::of("Bob");
    let ns = vec![0xFF, 0xFF];

    kv.store(
        &ns,
        &[
            (addr1.as_slice(), &[10, 20, 30]),
            (addr2.as_slice(), &[40, 50, 60]),
        ],
    );

    assert_key_value!(kv, ns, addr1.as_slice(), vec![10, 20, 30]);
    assert_key_value!(kv, ns, addr2.as_slice(), vec![40, 50, 60]);

    kv.clear();

    assert_no_key!(kv, ns, addr1.as_slice());
    assert_no_key!(kv, ns, addr2.as_slice());
}
