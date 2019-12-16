use svm_common::Address;
use svm_kv::{memory::MemKVStore, traits::KVStore};

mod asserts;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn a_key_does_not_exit_by_default() {
    init();

    let kv = MemKVStore::new();
    let addr = Address::from(0x11_22_33_44 as u32);

    assert_no_key!(kv, addr.as_slice());
}

#[test]
fn key_store_and_then_key_get() {
    init();

    let mut kv = MemKVStore::new();
    let addr = Address::from(0x11_22_33_44 as u32);
    kv.store(&[(addr.as_slice(), &[10, 20, 30])]);

    assert_key_value!(kv, addr.as_slice(), vec![10, 20, 30]);
}

#[test]
fn key_store_override_existing_entry() {
    init();

    let mut kv = MemKVStore::new();
    let addr = Address::from(0x11_22_33_44 as u32);

    kv.store(&[(addr.as_slice(), &[10, 20, 30])]);
    assert_key_value!(kv, addr.as_slice(), vec![10, 20, 30]);

    kv.store(&[(addr.as_slice(), &[40, 50, 60])]);
    assert_key_value!(kv, addr.as_slice(), vec![40, 50, 60]);
}

#[test]
fn clear() {
    init();

    let mut kv = MemKVStore::new();
    let addr1 = Address::from(0x11_22_33_44 as u32);
    let addr2 = Address::from(0x55_66_77_88 as u32);

    kv.store(&[
        (addr1.as_slice(), &[10, 20, 30]),
        (addr2.as_slice(), &[40, 50, 60]),
    ]);

    assert_key_value!(kv, addr1.as_slice(), vec![10, 20, 30]);
    assert_key_value!(kv, addr2.as_slice(), vec![40, 50, 60]);

    kv.clear();

    assert_no_key!(kv, addr1.as_slice());
    assert_no_key!(kv, addr2.as_slice());
}
