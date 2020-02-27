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
//!  |  first_layer         (8 bytes) |
//!  +--------------------------------+
//!  |  last_run_layer     (8 bytes)  |
//!  +--------------------------------+
//!  |  liquidated         (4 bytes)  |
//!  +--------------------------------+
//!  |  unliquidated       (4 bytes)  |
//!  +--------------------------------+
//!  |  layer_liquidation  (2 bytes)  |
//!  +--------------------------------+
//!  |  daily_pull_limit   (2 bytes)  |
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

mod auth;
mod computations;
mod read;
mod write;

use auth::*;
use read::*;
use write::*;

include!("constants.rs");
include!("imports.rs");

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
    write_pub_keys(is_multisig);
    write_first_layer();

    write_liquidated(liquidated);
    write_unliquidated(unliquidated);
    write_layer_liquidation(unliquidated, period_sec);

    // TODO: write `daily_pull_limit`
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
    let status = auth();

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
fn do_transfer(amount: u32) {
    unsafe {
        let liquidated = read_liquidated();

        assert!(liquidated >= amount);

        reg_push(160, 0);

        // loading `dest-address` given in func-buf into register `160:0`
        buffer_copy_to_reg(FUNC_BUF_ID, 0, 160, 0, ADDRESS_SIZE);

        add_balance_i32(amount, 160, 0);

        reg_pop(160, 0);
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
