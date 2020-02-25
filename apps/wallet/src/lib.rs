#![no_std]
#![allow(unused)]

//! The `SVM` wallet app is intended to serve as a MultiSig Wallet
//! For managing Spacemesh investors coins.
//!
//! Link to the product specification:
//! https://github.com/spacemeshos/product/blob/master/svm-wallet.md
//!
//!
//!  App Storage Layout
//!  ==================
//!
//!             Page #1
//!  +-----------------------------+
//!  |  pub_key1        (32 bytes) |
//!  |-----------------------------+
//!  |  pub_key2        (32 bytes) |
//!  |-----------------------------+
//!  |  pub_key3        (32 bytes) |
//!  |-----------------------------+
//!  |  last_pub_key    (32 bytes) |
//!  |-----------------------------+
//!  |  vesting_start   (8 bytes)  |
//!  |-----------------------------+
//!  |  last_sync_layer (8 bytes)  |
//!  +-----------------------------+
//!  |  balance         (4 bytes)  |
//!  +-----------------------------+
//!  |  vested          (4 bytes)  |
//!  +-----------------------------+
//!  |  daily_limit     (2 bytes)  |
//!  +-----------------------------+
//!  |  vesting_months  (2 bytes)  |
//!  +-----------------------------+
//!
//! Total storage: 156 bytes.
//!

include!("constants.rs");

// imports
svm_extern::include_storage_vmcalls!();
svm_extern::include_node_vmcalls!();
svm_extern::include_buffer_vmcalls!();
svm_extern::include_host_ctx_vmcalls!();
svm_extern::include_register_vmcalls!();

///  
/// The `init` function assumes the following `func_buf`
/// +-----------------------------------------------------------------+
/// | pub_key1 (32 bytes) | pub_key2 (32 bytes) | pub_key3 (32 bytes) |
/// +-----------------------------------------------------------------+
///
#[no_mangle]
pub extern "C" fn init(daily_limit: i64, vesting_start: i64, vesting_months: i32) {
    // 1) read `func-buf` into app-storage
    // 2) store `init` params into app-storage

    todo!()
}

#[no_mangle]
pub extern "C" fn get_vested() -> u32 {
    auth();

    refresh_vesting();
    read_vested()
}

#[no_mangle]
pub extern "C" fn get_unvested() -> u32 {
    auth();

    // 1  refresh_vesting();
    // 2) calculate maximum vesting
    // 3) vested <- read_vested()
    // 3) calculate `unvested = maximum_vesting - vested`

    todo!()
}

/// Returns the Wallet's balance.
/// The Wallet's balance might be less than the vested amount
/// since vested coins can be transfered to other account.
//
//  See `transfer` method.
#[no_mangle]
pub extern "C" fn get_app_balance() -> u64 {
    auth();

    // 1) refresh_vesting();
    // 2) read_balance();

    todo!()
}

/// The function expects the following func buf:
/// +--------------------------------+
/// | destination address (20 bytes) |
/// +--------------------------------+
///
#[no_mangle]
pub extern "C" fn transfer(amount: u32) {
    let completed = multisig_auth();

    if completed != 0 {
        return;
    }

    // 1) refresh_vesting();
    // 2) let balance = get_balance();
    // 3) if balance >= amount {
    //   3.1) load destination address from `func-buf` to `register` having 160 bits (20 bytes).
    //   3.2) call host vmcall: `add_balance(amount, reg_bits, reg_idx)`
    //}
    // 4) else {
    //   4.1) panic("not enough balance")
    //}

    todo!()
}

#[no_mangle]
pub extern "C" fn replace_pub_key(key_idx: i32) {
    auth();

    todo!()
}

/// Private

#[no_mangle]
fn auth() {
    // store regs
    unsafe {
        reg_push(256, 0);
        reg_push(256, 1);
    }

    // load `HostCtx.public_key` into register `256:0`
    unsafe {
        host_ctx_read_into_reg(PUBLIC_KEY_FIELD_IDX, 256, 0);
    }

    for idx in 0..3 {
        // load `pub_key#{idx}` into register `256:1`
        read_pub_key(idx, 256, 1);

        if pub_key_cmp(0, 1) == 0 {
            // restore regs
            unsafe {
                reg_pop(256, 1);
                reg_pop(256, 0);
            };

            // success
            return;
        }
    }

    panic!("auth failed")
}

#[no_mangle]
fn multisig_auth() -> i32 {
    auth();

    // 1) load `last_pub_key` into register `256:0`
    // 2) if its all zeros:
    //   2.1) `write_last_pub_key();`
    //   2.2)  return `1` signifying `multisig process isn't complete`
    //         else, goto 3)
    // 3) load HostCtx `pub-key` into register `256:1`
    // 4) if registers-equals(256, 0, 1)
    //   4.1) zero `last_pub_key`
    //   4.2) return `0` (meaning: multisig completed)
    // 5) else: `write_last_pub_key()`;

    todo!()
}

#[no_mangle]
extern "C" fn refresh_vesting() -> i32 {
    auth();

    // 1) load `last_sync_layer` from storage.
    // 2) fetch `layer_id` from `HostCtx`.
    // 3) fetch `layer_epoch` from `HostCtx`.
    // 4) fetch `layer_epoch_secs` from `HostCtx`.
    // 5) calculate layer `layer_diff = layer_id - last_sync_layer`
    // 6) translate `layer_diff` to `diff_time`
    //    `diff_time_secs = layer_diff * layer_epoch_secs`
    // 7) `last_sync_layer <- layer_id`
    // 8)

    todo!()
}

#[no_mangle]
fn write_last_pub_key() {
    todo!()
}

#[no_mangle]
fn read_pub_key(key_idx: u32, reg_bits: u32, reg_idx: u32) {
    assert!(key_idx <= 2);

    let offset = 32 * key_idx;

    unsafe { storage_read_to_reg(PAGE_IDX, offset, reg_bits, reg_idx, PUB_KEY_SIZE) }
}

#[no_mangle]
fn read_daily_limit() -> u32 {
    unsafe { storage_read_i32_be(PAGE_IDX, DAILY_LIMIT_OFFSET, DAILY_LIMIT_SIZE) }
}

#[no_mangle]
fn read_vesting_start() -> u64 {
    unsafe { storage_read_i64_be(PAGE_IDX, VESTING_START_OFFSET, VESTING_START_SIZE) }
}

#[no_mangle]
fn read_vesting_months() -> u32 {
    unsafe { storage_read_i32_be(PAGE_IDX, VESTING_MONTHS_OFFSET, VESTING_MONTHS_SIZE) }
}

#[no_mangle]
fn read_last_sync_layer() -> u64 {
    unsafe { storage_read_i64_be(PAGE_IDX, LAST_SYNC_LAYER_OFFSET, LAYER_ID_SIZE) }
}

#[no_mangle]
fn read_balance() -> u32 {
    unsafe { storage_read_i32_be(PAGE_IDX, BALANCE_OFFSET, BALANCE_SIZE) }
}

#[no_mangle]
fn read_vested() -> u32 {
    unsafe { storage_read_i32_be(PAGE_IDX, VESTED_OFFSET, VESTED_SIZE) }
}

#[no_mangle]
fn pub_key_cmp(reg_idx1: u32, reg_idx2: u32) -> u32 {
    unsafe { reg_eql(256, reg_idx1, reg_idx2) }
}
