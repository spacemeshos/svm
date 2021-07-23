use svm_layout::{FixedLayout, Id};
use svm_storage::{account::AccountStorage, testing};
use svm_types::Address;

fn assert_var<const N: usize>(account: &AccountStorage, var_id: u32, expected: [u8; N]) {
    let var = account.read_var(Id(var_id));
    assert_eq!(&var[..], &expected[..]);
}

fn write_var<const N: usize>(account: &mut AccountStorage, var_id: u32, var: [u8; N]) {
    account.write_var(Id(var_id), var.to_vec());
}

#[test]
fn account_storage_vars_are_persisted_only_on_commit() {
    // `var #0` consumes 4 bytes (offsets: `[0..4)`)
    // `var #1` consumes 2 bytes (offsets: `[4, 6)`)
    let layout = FixedLayout::from(vec![4, 2].as_slice());

    let addr = Address::of("@Account");
    let kv = testing::create_account_kv(addr);

    let account = &mut AccountStorage::new(layout.clone(), kv.clone());

    // vars are initialized with zeros
    assert_var(account, 0, [0, 0, 0, 0]);
    assert_var(account, 1, [0, 0]);

    write_var(account, 0, [10, 20, 30, 40]);
    write_var(account, 1, [50, 60]);

    // vars latest version are in memory (uncommitted yet)
    assert_var(account, 0, [10, 20, 30, 40]);
    assert_var(account, 1, [50, 60]);

    // spin a new account with no in-memory dirty data
    let account2 = &mut AccountStorage::new(layout.clone(), kv.clone());

    // `account`'s' uncommitted changes are not reflected yet
    assert_var(account2, 0, [0, 0, 0, 0]);
    assert_var(account2, 1, [0, 0]);

    // now, we'll commit the dirty changes
    let _state = account.commit();

    // we'll spin a new account with no caching
    let account3 = &mut AccountStorage::new(layout.clone(), kv.clone());

    // asserting that `commit` persisted the data
    assert_var(account3, 0, [10, 20, 30, 40]);
    assert_var(account3, 1, [50, 60]);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn account_storage_write_var_value_should_match_layout_length() {
    // `var #0` consumes 4 bytes (i.e `length = 4`)
    let layout: FixedLayout = vec![4].into();
    let addr = Address::of("@account");
    let kv = testing::create_account_kv(addr);

    let mut account = AccountStorage::new(layout, kv);

    // calling `write_var` with 2-byte value (expected variable's to value to be 4 bytes)
    account.write_var(Id(0), vec![0, 0]);
}
