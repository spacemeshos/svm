use crate::FuncEnv;
use svm_state::AccountStorage;
use svm_types::{Address, BytesPrimitive};

fn load_addr(env: &FuncEnv, mem_ptr: u32) -> Address {
    let borrow = env.borrow();
    let wasm_memory = &borrow.memory().view::<u8>();

    let byte_cells = &wasm_memory[mem_ptr as usize..mem_ptr as usize + Address::N];
    let mut bytes = [0u8; Address::N];
    for (i, cell) in byte_cells.iter().enumerate() {
        bytes[i] = cell.get();
    }
    Address::new(bytes)
}

/// Sends coins from the current executing account to a destination account.
///
/// # Panics
///
/// Panics when the destination account does not exist.
pub fn svm_transfer(env: &FuncEnv, src_addr_ptr: i32, dst_addr_ptr: i32, amount: i64) {
    let borrow = env.borrow();
    let gs = borrow.storage().gs.clone();

    let src_addr = load_addr(env, src_addr_ptr as u32);
    let dst_addr = load_addr(env, dst_addr_ptr as u32);

    let mut src_account = AccountStorage::load(gs.clone(), &src_addr).unwrap();
    let mut dst_account = AccountStorage::load(gs, &dst_addr).unwrap();

    let src_bal = src_account.balance().unwrap();
    let dst_bal = dst_account.balance().unwrap();

    let amount = amount as u64;

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
