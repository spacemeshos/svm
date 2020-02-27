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
//!  |  pending_pub_key    (32 bytes) |
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
//! pub_key1:          Wallet public-key. Used also when `is_multisig = 0`
//! pub_key2:          The 2nd public-key for 2-3 MultiSig. Relevant only when `is_multisig = 1`
//! pub_key3:          The 3rd public-key for 2-3 MultiSig. Relevant only when `is_multisig = 1`
//! pending_pub_key:   Relevant only when `is_multisig = 1`
//! is_multisig:       Whether the wallet is a 2-3 MultiSig or not.
//! first_layer:       The layer when the app ran for the first-time.
//! last_run_layer:    The layer when the app ran last time.
//! liquidated:        The amount of liquidated coins of the wallet.
//! unliquidated:      The amount of not-liquidated (yet) coins of the wallet.
//! daily_pull_limit:  The maximum liquidated coins that can be pulled from the wallet on a single-day.
//! layer_liquidation: The amount of newly liquidated coins per-layer.
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

/// Public API

#[no_mangle]
pub extern "C" fn init(
    is_multisig: u32,
    liquidated: u32,
    unliquidated: u32,
    daily_pull_limit: u32,
    period_sec: u32,
) {
    init_pub_keys(is_multisig);
    init_first_layer();

    store_liquidated(liquidated);
    store_unliquidated(unliquidated);
    init_layer_liquidation(unliquidated, period_sec);

    // TODO: store `daily_pull_limit`
}

#[no_mangle]
pub extern "C" fn get_liquidated() -> u32 {
    refresh_liquidation();
    read_liquidated()
}

#[no_mangle]
pub extern "C" fn get_unliquidated() -> u32 {
    refresh_liquidation();

    read_unliquidated()
}

/// The function expects the following func buf:
/// +---------------------------------+
/// | destination address (20 bytes)  |
/// +---------------------------------+
///
#[no_mangle]
pub extern "C" fn transfer(amount: u32) {
    let status = multisig_auth();

    if status != 0 {
        // we've got only one pub-key for the transfer.
        // TODO: should we ignore `amount` here?
        return;
    }

    refresh_liquidation();

    do_transfer(amount);
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

    //  load `pending_pub_key` into register `256:0`
    read_pending_pub_key(256, 0);

    // 2) if its all zeros:
    //   2.1) `store_pending_pub_key();`
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
        store_pending_pub_key();
    }

    // store regs
    unsafe {
        reg_pop(256, 0);
        reg_pop(256, 1);
    }

    status
}

#[no_mangle]
fn do_transfer(amount: u32) {
    unsafe {
        let liquidated = read_liquidated();
        assert!(liquidated >= amount);

        reg_push(160, 0);

        // loading `dest-address` given in func-buf into register `160:0`
        buffer_copy_to_reg(IN_FUNC_BUF_ID, 0, 160, 0, ADDRESS_SIZE);

        add_balance_i32(amount, 160, 0);

        reg_pop(160, 0);
    }
}

#[no_mangle]
fn refresh_liquidation() {
    auth();

    let layer_liq = read_layer_liquidation();
    let last_run_layer = read_last_run_layer();
    let current_layer = read_current_layer();

    let delta = computations::liquidation_delta(layer_liq, last_run_layer, current_layer);

    let liquidated = read_liquidated();
    let unliquidated = read_unliquidated();

    assert!(unliquidated >= delta);

    store_liquidated(liquidated + delta);
    store_unliquidated(unliquidated - delta);
}

#[no_mangle]
fn pub_key_cmp(reg_idx1: u32, reg_idx2: u32) -> i32 {
    unsafe { reg_cmp(256, reg_idx1, reg_idx2) }
}

/// Reading from storage
//===========================================================================================
#[no_mangle]
fn read_pending_pub_key(reg_bits: u32, reg_idx: u32) {
    read_pub_key(3, reg_bits, reg_idx);
}

#[no_mangle]
fn read_current_layer() -> u64 {
    host_ctx_read_i64_be(LAYER_INDEX)
}

#[no_mangle]
fn read_pub_key(key_idx: u32, reg_bits: u32, reg_idx: u32) {
    assert!(key_idx <= 3);

    let offset = 32 * key_idx;

    unsafe { storage_read_to_reg(PAGE_IDX, offset, reg_bits, reg_idx, PUB_KEY_SIZE) }
}

#[no_mangle]
fn read_first_layer() -> u64 {
    unsafe { storage_read_i64_be(PAGE_IDX, FIRST_LAYER_OFFSET, 8) }
}

#[no_mangle]
fn read_last_run_layer() -> u64 {
    unsafe { storage_read_i64_be(PAGE_IDX, LAST_RUN_LAYER, 8) }
}

#[no_mangle]
fn read_liquidated() -> u32 {
    unsafe { storage_read_i32_be(PAGE_IDX, LIQUIDATED_OFFSET, 4) }
}

#[no_mangle]
fn read_unliquidated() -> u32 {
    unsafe { storage_read_i32_be(PAGE_IDX, UNLIQUIDATED_OFFSET, 4) }
}

#[no_mangle]
fn read_layer_liquidation() -> u32 {
    unsafe { storage_read_i32_be(PAGE_IDX, LAYER_LIQ_OFFSET, 2) }
}

/// Storing data into app-storage
//===========================================================================================
///
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
fn init_pub_keys(is_multisig: u32) {
    unsafe {
        if is_multisig == 0 {
            // store `pub_key1`
            buffer_copy_to_storage(IN_FUNC_BUF_ID, 0, PAGE_IDX, 0, PUB_KEY_SIZE);

            // store: `is_multisig=0`
            storage_write_i32_be(PAGE_IDX, IS_MULTISIG_OFFSET, 0, 1);
        } else {
            // storing `pub_key1, pub_key2, pub_key3`
            // we copy the keys at one shot,
            // since they are laid contagiously at both input func-buffer and app-storage
            buffer_copy_to_storage(IN_FUNC_BUF_ID, 0, PAGE_IDX, 0, PUB_KEY_SIZE * 3);

            // store: `is_multisig=1`
            storage_write_i32_be(PAGE_IDX, IS_MULTISIG_OFFSET, 1, 1);
        }
    }
}

#[no_mangle]
fn init_first_layer() {
    unsafe {
        let layer = read_current_layer();

        storage_write_i64_be(PAGE_IDX, FIRST_LAYER_OFFSET, layer, 8);
        storage_write_i64_be(PAGE_IDX, LAST_RUN_LAYER_OFFSET, layer, 8);
    }
}

#[no_mangle]
fn init_layer_liquidation(unliquidated: u32, period_sec: u32) {
    unsafe {
        let layer = host_ctx_read_i64_be(LAYER_INDEX);
        let layer_time_sec = host_ctx_read_i32_be(LAYER_TIME_INDEX);

        let layer_count = computations::layer_count(period_sec, layer_time_sec);
        let layer_liq = computations::layer_liquidation(unliquidated, layer_count);

        assert!(layer_liq <= 0xFFFF);

        storage_write_i32_be(PAGE_IDX, LAYER_LIQ_OFFSET, layer_liq, 2);
    }
}

// persist `HostCtx pub_key` into app-storage `last_pub_key`
#[no_mangle]
fn store_pending_pub_key() {
    unsafe {
        reg_push(256, 0);
        host_ctx_read_into_reg(PUBLIC_KEY_FIELD_IDX, 256, 0);
        storage_write_from_reg(256, 0, PAGE_IDX, LAST_PUB_KEY_OFFSET, PUB_KEY_SIZE);
        reg_pop(256, 0);
    }
}

#[no_mangle]
fn store_liquidated(liquidated: u32) {
    unsafe {
        storage_write_i32_be(PAGE_IDX, LIQUIDATED_OFFSET, liquidated, 4);
    }
}

#[no_mangle]
fn store_unliquidated(unliquidated: u32) {
    unsafe {
        storage_write_i32_be(PAGE_IDX, UNLIQUIDATED_OFFSET, unliquidated, 4);
    }
}
