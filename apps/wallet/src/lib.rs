#![no_std]
#![allow(unused)]

//! The `SVM` wallet app is intended to serve as a 2-3 MultiSig Wallet
//! For managing Spacemesh investors coins.
//!
//! Link to the product specification:
//! https://github.com/spacemeshos/product/blob/master/svm-wallet.md
//!
//! App Storage variables explanation:
//!
//! pub_key1:          Wallet public-key. Used also when `is_multisig = 0`
//! pub_key2:          The 2nd public-key for 2-3 MultiSig. Relevant only when `is_multisig = 1`
//! pub_key3:          The 3rd public-key for 2-3 MultiSig. Relevant only when `is_multisig = 1`
//! pending_pub_key:   Relevant only when `is_multisig = 1`
//! first_layer:       The layer when the app ran for the first-time.
//! last_run_layer:    The layer when the app ran last time.
//! period_time_sec:   The period of time (in seconds) for full-liquidation.
//! lockup_time_sec:   The wallet's lockup time (in seconds).
//! liquidated:        The amount of liquidated coins of the wallet.
//! unliquidated:      The amount of not-liquidated (yet) coins of the wallet.
//! balance:           The wallet's balance (i.e: `liquidated` minus the amount of pulled).
//! layer_liquidation: The amount of newly liquidated coins per-layer. (calculated in the app's `init`).
//! is_multisig:       Whether the wallet is a 2-3 MultiSig or not.
//!

include!("imports.rs");

mod auth;
mod computations;
mod constants;
mod read;
mod write;

pub use constants::*;

/// Public API

#[no_mangle]
pub extern "C" fn init(is_multisig: u32, coins: u32, period_sec: u32, lockup_time_sec: u32) {
    write::write_pub_keys(is_multisig);
    write::write_first_layer();
    write::write_period_sec(period_sec);

    write::write_liquidated(0);
    write::write_unliquidated(coins);

    write::write_layer_liquidation(coins, period_sec);
    write::write_lockup_time(lockup_time_sec);
}

#[no_mangle]
pub extern "C" fn get_liquidated() -> u32 {
    refresh_liquidation();
    read::read_liquidated()
}

#[no_mangle]
pub extern "C" fn get_unliquidated() -> u32 {
    refresh_liquidation();
    read::read_unliquidated()
}

/// The function expects the following func buf:
/// +------------------------------------------------------+
/// | pub-key (32 bytes) | destination address (20 bytes)  |
/// +------------------------------------------------------+
///
#[no_mangle]
pub extern "C" fn transfer(amount: u64) {
    auth::pub_key_auth();
    do_transfer(amount);
}

/// The function expects the following func buf:
/// +---------------------+
/// | pub-key (32 bytes)  |
/// +---------------------+
///
#[no_mangle]
pub extern "C" fn transfer_prepare() {
    auth::multisig_start()
}

///
/// The function expects the following func buf:
/// +------------------------------------------------------+
/// | pub-key (32 bytes) | destination address (20 bytes)  |
/// +------------------------------------------------------+
///
#[no_mangle]
pub extern "C" fn transfer_apporove(amount: u64) {
    auth::multisig_complete();
    do_transfer(amount);
}

/// Private

#[no_mangle]
fn do_transfer(amount: u64) {
    unsafe {
        refresh_liquidation();

        let balance = host_current_balance();

        assert!(balance >= amount);

        reg_push(160, 0);

        // loading `dest-address` given in func-buf into register `160:0`
        buffer_copy_to_reg(0, 0, 160, 0, 20);

        host_transfer(amount, 160, 0);

        reg_pop(160, 0);
    }
}

#[no_mangle]
fn refresh_liquidation() {
    let layer_liq = read::read_layer_liquidation();
    let last_run_layer = read::read_last_run_layer();
    let current_layer = read::read_current_layer();

    let delta = computations::liquidation_delta(layer_liq, last_run_layer, current_layer);

    let liquidated = read::read_liquidated();
    let unliquidated = read::read_unliquidated();

    assert!(unliquidated >= delta);

    write::write_last_run_layer(current_layer);
    write::write_liquidated(liquidated + delta);
    write::write_unliquidated(unliquidated - delta);
}
