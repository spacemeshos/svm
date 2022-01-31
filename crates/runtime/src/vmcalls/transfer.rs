use crate::FuncEnv;
use svm_state::AccountStorage;
use svm_types::{Address, BytesPrimitive};
use wasmer::WasmPtr;

const NOT_FOUND_ERR: &str = "Not found address at the given memory offsset.";

fn read_addr(env: &FuncEnv, ptr: WasmPtr<Address>) -> Option<Address> {
    let ptr: WasmPtr<[u8; Address::N]> = WasmPtr::new(ptr.offset());
    let bytes = super::read_memory_bytes(env, ptr)?;
    Some(Address::new(bytes))
}

/// Sends coins from the current executing account to a destination account.
///
/// # Panics
///
/// Panics when the destination account does not exist.
pub fn svm_transfer(env: &FuncEnv, src_addr_offset: i32, dst_addr_offset: i32, amount: u64) {
    if amount == 0 {
        return;
    }

    let rt = env.tokio_runtime.clone();
    let src_addr_ptr = WasmPtr::new(src_addr_offset as u32);
    let dst_addr_ptr = WasmPtr::new(dst_addr_offset as u32);

    let src_addr = read_addr(env, src_addr_ptr).expect(NOT_FOUND_ERR);
    let dst_addr = read_addr(env, dst_addr_ptr).expect(NOT_FOUND_ERR);

    let mut borrow = env.borrow_mut();
    let gs = borrow.storage().gs.clone();

    let mut src_account = rt
        .block_on(AccountStorage::load(gs.clone(), &src_addr))
        .unwrap();
    let mut dst_account = rt.block_on(AccountStorage::load(gs, &dst_addr)).unwrap();
    let src_balance = rt.block_on(src_account.balance()).unwrap();
    let dst_balance = rt.block_on(dst_account.balance()).unwrap();

    let amount = amount as u64;

    if src_balance < amount {
        panic!("Not enough balance to execute transfer")
    }

    rt.block_on(src_account.set_balance(src_balance - amount))
        .unwrap();
    rt.block_on(dst_account.set_balance(dst_balance.checked_add(amount).unwrap()))
        .unwrap();

    borrow.touch_account(src_addr);
    borrow.touch_account(dst_addr);
}
