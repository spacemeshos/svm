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
//! period_sec:        The period of time (in seconds) for full-liquidation.
//! lockup_sec:        The wallet's lockup time (in seconds).
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

pub(crate) use {constants::*, read::*, write::*};

#[allow(non_upper_case_globals)]
static buf_idx: u32 = 0;

/// Public API

/// called as part of app's spawning.
#[no_mangle]
pub extern "C" fn init(is_multisig: u32, coins: u32, period_sec: u32, lockup_sec: u32) {
    /// We need to make sure that `coins` have been locked into tha app.
    /// Otherwise, spawning the wallet should fail.
    assert_eq!(coins as u64, unsafe { host_get_my_balance() });

    write_pub_keys(is_multisig);
    write_first_layer();
    write_period_sec(period_sec);

    write_liquidated(0);
    write_unliquidated(coins);

    write_layer_liquidation(coins, period_sec);
    write_lockup_time(lockup_sec);
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

#[no_mangle]
pub extern "C" fn get_transfered() -> u32 {
    read_transferred()
}

#[no_mangle]
pub extern "C" fn get_available() -> u32 {
    refresh_liquidation();

    let liquidated = read_liquidated();
    let transferred = read_transferred();
    assert!(liquidated >= transferred);

    let available = liquidated - transferred;

    available
}

/// The function expects the following func buf:
/// +------------------------------------------------------+
/// | pub-key (32 bytes) | destination address (20 bytes)  |
/// +------------------------------------------------------+
///
#[no_mangle]
pub extern "C" fn transfer(amount: u32) {
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
pub extern "C" fn transfer_apporove(amount: u32) {
    auth::multisig_complete();
    do_transfer(amount);
}

/// Private

#[no_mangle]
fn do_transfer(amount: u32) {
    unsafe {
        refresh_liquidation();

        let liquidated = read_liquidated();
        let transferred = read_transferred();
        assert!(liquidated >= transferred);

        let available = liquidated - transferred;
        assert!(available >= amount);

        reg_push(160, 0);

        // loading `dest-address` given in func-buf into register `160:0`
        buffer_copy_to_reg(buf_idx, 0, 160, 0, sizeof!(addr));

        host_transfer(amount as u64, 160, 0);
        reg_pop(160, 0);

        write_transferred(transferred + amount);
    }
}

#[no_mangle]
fn refresh_liquidation() {
    let layer_liq = read_layer_liquidation();
    let last_run_layer = read_last_run_layer();
    let current_layer = read_current_layer();

    let delta = computations::liquidation_delta(layer_liq, last_run_layer, current_layer);

    let liquidated = read_liquidated();
    let unliquidated = read_unliquidated();

    assert!(unliquidated >= delta);

    write_last_run_layer(current_layer);
    write_liquidated(liquidated + delta);
    write_unliquidated(unliquidated - delta);
}
