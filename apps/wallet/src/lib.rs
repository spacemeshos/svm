#![no_std]
#![allow(unused)]

//! The `SVM` wallet app is intended to serve as a 2-3 MultiSig Wallet
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
//!  +--------------------------------+
//!  |  pub_key1           (32 bytes) |
//!  |--------------------------------+
//!  |  pub_key2           (32 bytes) |
//!  |--------------------------------+
//!  |  pub_key3           (32 bytes) |
//!  |--------------------------------+
//!  |  last_pub_key       (32 bytes) |
//!  |--------------------------------+
//!  |  is_multisig         (1 byte)  |
//!  |--------------------------------+
//!  |  first_layer        (8 bytes)  |
//!  +--------------------------------+
//!  |  last_run_layer     (8 bytes)  |
//!  +--------------------------------+
//!  |  liquidated         (4 bytes)  |
//!  +--------------------------------+
//!  |  unliquidated       (4 bytes)  |
//!  +--------------------------------+
//!  |  daily_pull_limit   (2 bytes)  |
//!  +--------------------------------+
//!  |  layer_liquidation  (2 bytes)  |
//!  +--------------------------------+
//!
//! App Storage variables explanation:
//!
//! pub_key1:         Wallet public-key. Used also when `is_multisig = 0`
//! pub_key2:         The 2nd public-key for 2-3 MultiSig. Relevant only when `is_multisig = 1`
//! pub_key3:         The 3rd public-key for 2-3 MultiSig. Relevant only when `is_multisig = 1`
//! last_pub_key:     Relevant only when `is_multisig = 1`
//! is_multisig:      Whether the wallet is a 2-3 MultiSig or not.
//! first_layer:      The layer when the app first ran.
//! last_run_layer:   The layer when the app ran last time.
//! liquidated:       The amount of liquidated coins of the wallet.
//! unliquidated:     The amount of not liquidated yet coins of the wallet.
//! daily_pull_limit: The maximum liquidated coins that can be pulled from the wallet.
//! layer_liquidation: The amount of liquidated coins per-layer.
//!

mod computations;

include!("constants.rs");

// SVM Internal Imports
svm_extern::include_storage_vmcalls!();
svm_extern::include_node_vmcalls!();
svm_extern::include_buffer_vmcalls!();
svm_extern::include_host_ctx_vmcalls!();
svm_extern::include_register_vmcalls!();

// Host Imports
extern "C" {
    fn add_balance_i32(amount: u32, reg_bits: u32, reg_idx: u32);
}

///  
/// The `init` function assumes the following `func_buf`
///
/// When `is_multi_sig = 0`
/// +---------------------+
/// | pub_key1 (32 bytes) |
/// +---------------------+
///
/// When `is_multi_sig != 0`
/// +-----------------------------------------------------------------+
/// | pub_key1 (32 bytes) | pub_key2 (32 bytes) | pub_key3 (32 bytes) |
/// +-----------------------------------------------------------------+
///
#[no_mangle]
pub extern "C" fn init(
    is_multi_sig: u32,
    liquidated: u32,
    unliquidated: u32,
    daily_pull_limit: u32,
    period_sec: u32,
) {
    // 1) grab layer_id from the `HostCtx`
    // 2) store `liquidated`
    // 3) store `unliquidated`
    // 4) store `daily_pull_limit`
    // 5)liquidation_per_layer(unliquidated, layer)

    // storing `pub_key1, pub_key2, pub_key3`
    unsafe {
        // we copy the keys at one operation
        // since they are laid in contiguous at both input buffer and storage
        buffer_copy_to_storage(IN_FUNC_BUF_ID, 0, PAGE_IDX, 0, PUB_KEY_SIZE * 3);
    }

    // store `vesting_start`
    unsafe {
        storage_write_i64_be(
            PAGE_IDX,
            VESTING_START_OFFSET,
            vesting_start,
            VESTING_START_SIZE,
        )
    }

    // store `max_vesting`
    unsafe {
        storage_write_i32_be(PAGE_IDX, MAX_VESTING_OFFSET, max_vesting, BALANCE_SIZE);
    }

    // store `daily_limit`
    unsafe { storage_write_i64_be(PAGE_IDX, DAILY_LIMIT_OFFSET, daily_limit, DAILY_LIMIT_SIZE) }

    // store `vesting_months`
    unsafe {
        storage_write_i32_be(
            PAGE_IDX,
            VESTING_MONTHS_OFFSET,
            vesting_months,
            VESTING_MONTHS_SIZE,
        );
    }
}

#[no_mangle]
pub extern "C" fn get_vested() -> u32 {
    refresh_vesting();
    read_vested()
}

#[no_mangle]
pub extern "C" fn get_unvested() -> u32 {
    refresh_vesting();

    let vested = read_vested();
    let max_vesting = read_max_vesting();

    assert!(max_vesting >= vested);

    max_vesting - vested
}

/// Returns the Wallet's balance.
/// The Wallet's balance might be less than the vested amount
/// since vested coins can be transfered to other account.
//
//  See `transfer` method.
#[no_mangle]
pub extern "C" fn get_app_balance() -> u32 {
    refresh_vesting();

    read_balance()
}

/// The function expects the following func buf:
/// +----------------------+
/// | destination address  |
/// +----------------------+
///
#[no_mangle]
pub extern "C" fn transfer(amount: u32) {
    let status = multisig_auth();

    if status != 0 {
        // we've got only one pub-key for the transfer.
        // TODO: should we ignore `amount` here?
        return;
    }

    refresh_vesting();

    let balance = read_balance();

    if balance >= amount {
        unsafe {
            reg_push(160, 0);

            // loading `dest-address` given in func-buf into register `160:0`
            buffer_copy_to_reg(IN_FUNC_BUF_ID, 0, 160, 0, ADDRESS_SIZE);

            add_balance_i32(amount, 160, 0);

            reg_pop(160, 0);
            return;
        }
    }

    panic!("not enough balance")
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
            // we've found a match

            // restore regs
            unsafe {
                reg_pop(256, 1);
                reg_pop(256, 0);
            };

            return;
        }
    }

    panic!("auth failed")
}

#[no_mangle]
fn multisig_auth() -> i32 {
    auth();

    // store regs
    unsafe {
        reg_push(256, 0);
        reg_push(256, 1);
    }

    //  load `last_pub_key` into register `256:0`
    read_last_pub_key(256, 0);

    // 2) if its all zeros:
    //   2.1) `write_last_pub_key();`
    //   2.2)  return `1` signifying `multisig process isn't complete`
    //         else, goto 3)

    // load HostCtx `pub-key` into register `256:1`
    unsafe {
        host_ctx_read_into_reg(PUBLIC_KEY_FIELD_IDX, 256, 1);
    }

    let status = pub_key_cmp(0, 1);

    if status == 0 {
        unsafe {
            // registers are equals
            reg_push(256, 0);

            // this will zero register `256:0`
            reg_set_i32_be(256, 0, 4);

            // zeroing the `last_pub_key` since we copy the contents of register `256:0`
            storage_write_from_reg(256, 0, PAGE_IDX, LAST_PUB_KEY_OFFSET, PUB_KEY_SIZE);

            reg_pop(256, 0);
        }
    } else {
        // overriding the `last_pub_key`
        write_last_pub_key();
    }

    // store regs
    unsafe {
        reg_pop(256, 0);
        reg_pop(256, 1);
    }

    status
}

#[no_mangle]
extern "C" fn refresh_vesting() {
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

// persist `HostCtx pub_key` into app-storage `last_pub_key`
#[no_mangle]
fn write_last_pub_key() {
    unsafe {
        reg_push(256, 0);
        host_ctx_read_into_reg(PUBLIC_KEY_FIELD_IDX, 256, 0);
        storage_write_from_reg(256, 0, PAGE_IDX, LAST_PUB_KEY_OFFSET, PUB_KEY_SIZE);
        reg_pop(256, 0);
    }
}

#[no_mangle]
fn read_last_pub_key(reg_bits: u32, reg_idx: u32) {
    read_pub_key(3, reg_bits, reg_idx);
}

#[no_mangle]
fn read_pub_key(key_idx: u32, reg_bits: u32, reg_idx: u32) {
    assert!(key_idx <= 3);

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
    unsafe { storage_read_i32_be(PAGE_IDX, VESTED_OFFSET, BALANCE_SIZE) }
}

#[no_mangle]
fn read_max_vesting() -> u32 {
    unsafe { storage_read_i32_be(PAGE_IDX, MAX_VESTING_OFFSET, BALANCE_SIZE) }
}

#[no_mangle]
fn pub_key_cmp(reg_idx1: u32, reg_idx2: u32) -> i32 {
    unsafe { reg_cmp(256, reg_idx1, reg_idx2) }
}
