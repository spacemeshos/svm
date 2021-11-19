use crate::FuncEnv;
use svm_state::AccountStorage;
use svm_types::{Address, BytesPrimitive};

/// Sends coins from the current executing account to a destination account.
///
/// # Panics
///
/// Panics when the destination account does not exist.
pub fn svm_transfer(env: &FuncEnv, src_addr: u8, dst_addr: u8, amount: u64) {
    let borrow = env.borrow();
    let storage = borrow.storage();

    let src_addr = Address::repeat(src_addr);
    let dst_addr = Address::repeat(dst_addr);

    let mut src_account = AccountStorage::load(storage.gs.clone(), &src_addr).unwrap();
    let mut dst_account = if let Ok(dst) = AccountStorage::load(storage.gs.clone(), &dst_addr) {
        dst
    } else {
        panic!("Destination account does not exist")
    };

    let src_bal = src_account.balance().unwrap();
    let dst_bal = dst_account.balance().unwrap();

    if src_bal < amount {
        panic!("Not enough balance to execute transfer")
    }
    src_account
        .set_balance(src_bal.checked_sub(amount).unwrap())
        .unwrap();
    dst_account
        .set_balance(dst_bal.checked_add(amount).unwrap())
        .unwrap();
}
