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
//!  |  daily_limit     (2 bytes)  |
//!  +-----------------------------+
//!  |  start_vesting   (8 bytes)  |
//!  +-----------------------------+
//!  |  vesting_months  (2 bytes)  |
//!  +-----------------------------+
//!  |  last_sync_layer (8 bytes)  |
//!  +-----------------------------+
//!  |  balance         (4 bytes)  |
//!  +-----------------------------+
//!  |  vested          (4 bytes)  |
//!  +-----------------------------+
//!
//!  Offsets:
//!  * pub_key1        - offset=0,   length=32
//!  * pub_key2        - offset=32,  length=32
//!  * pub_key3        - offset=64,  length=32
//!  * daily_limit     - offset=96,  length=2
//!  * start_vesting   - offset=98,  length=8
//!  * vesting_months  - offset=106, length=2
//!  * last_sync_layer - offset=108, length=8
//!  * balance         - offset=116, length=4
//!  * vested          - offset=120, length=4
//!
//! Total storage: 124 bytes.
//!

///  
/// The `init` function assumes the following `func_buf`
/// +-----------------------------------------------------------------+
/// | pub_key1 (32 bytes) | pub_key2 (32 bytes) | pub_key3 (32 bytes) |
/// +-----------------------------------------------------------------+
///
#[no_mangle]
pub extern "C" fn init(daily_limit: i64, start_vesting: i64, vesting_months: i32) {
    // 1) read `func-buf` into app-storage
    // 2) store `init` params into app-storage

    todo!()
}

#[no_mangle]
pub extern "C" fn get_vested() -> i32 {
    auth();

    update_vesting();
    read_vested()
}

#[no_mangle]
pub extern "C" fn get_unvested() -> i32 {
    auth();

    // 1  update_vesting();
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
pub extern "C" fn get_balance() -> i64 {
    auth();

    // 1) update_vesting();
    // 2) read_balance();

    todo!()
}

/// The function expects the following func buf:
/// +--------------------------------+
/// | destination address (20 bytes) |
/// +--------------------------------+
///
#[no_mangle]
pub extern "C" fn transfer(amount: i64) {
    auth();

    // 1) update_vesting();
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
extern "C" fn auth() {
    // 1) fetch `public_key` field from `HostCtx` and load it into a new buffer.
    // 2) compare `pub_key1` with `HostCtx pub_key`
    // if are equal return, else 3)
    // 3) compare `pub_key2` with `HostCtx pub_key`
    // if are equal return, else 4)
    // 4) compare `pub_key3` with `HostCtx pub_key`
    // if are equal return, else `panic`

    // const BUF_ID: i32 = 0;
    // if buf_eq(BUF_ID, 0, 0, 32, 32) {
    //     return
    // }
    // else {
    //}

    todo!()
}

#[no_mangle]
extern "C" fn update_vesting() -> i32 {
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
extern "C" fn read_balance() -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn read_vested() -> i32 {
    todo!()
}
