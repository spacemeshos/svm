include!("imports.rs");

use super::{auth, computations, read};

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
pub(crate) fn write_pub_keys(is_multisig: u32) {
    unsafe {
        if auth::is_multisig() {
            // storing `pub_key1, pub_key2, pub_key3`
            // we copy the keys at one shot,
            // since they are laid contagiously at both input func-buffer and app-storage
            buffer_copy_to_storage(0, 0, 0, 0, 32 * 3);

            // store: `is_multisig=1`
            storage_write_i32_be(0, 0, 1, 1);
        } else {
            // store `pub_key1`
            buffer_copy_to_storage(0, 0, 0, 0, 32);

            // store: `is_multisig=0`
            storage_write_i32_be(0, 0, 0, 1);
        }
    }
}

#[no_mangle]
pub(crate) fn write_first_layer() {
    unsafe {
        let layer = read::read_current_layer();

        storage_write_i64_be(0, 0, layer, 8);

        // set init `last_run_layer` with `first_layer`
        write_last_run_layer(layer);
    }
}

#[no_mangle]
pub(crate) fn write_layer_liquidation(unliquidated: u32, period_sec: u32) {
    unsafe {
        let layer = host_ctx_read_i64_be(0);
        let layer_time_sec = host_ctx_read_i32_be(0);

        let layer_count = computations::layer_count(period_sec, layer_time_sec);
        let layer_liq = computations::layer_liquidation(unliquidated, layer_count);

        assert!(layer_liq <= 0xFFFF);

        storage_write_i32_be(0, 0, layer_liq, 2);
    }
}

// persist `HostCtx pub_key` into app-storage `last_pub_key`
#[no_mangle]
pub(crate) fn write_pending_pub_key() {
    unsafe {
        reg_push(256, 0);
        host_ctx_read_into_reg(0, 256, 0);
        storage_write_from_reg(256, 0, 0, 0, 32);
        reg_pop(256, 0);
    }
}

#[no_mangle]
pub(crate) fn reset_pending_pub_key() {
    unsafe {
        reg_push(256, 0);

        // the side-effect of the folllowing is zero-ing
        // the `256:0` register.
        reg_set_i32_be(256, 0, 4);

        storage_write_from_reg(256, 0, 0, 0, 32);

        reg_pop(256, 0);
    }
}

#[no_mangle]
pub(crate) fn write_liquidated(liquidated: u32) {
    unsafe {
        storage_write_i32_be(0, 0, liquidated, 4);
    }
}

#[no_mangle]
pub(crate) fn write_unliquidated(unliquidated: u32) {
    unsafe {
        storage_write_i32_be(0, 0, unliquidated, 4);
    }
}

#[no_mangle]
pub(crate) fn write_last_run_layer(layer: u64) {
    unsafe {
        storage_write_i64_be(0, 0, layer, 8);
    }
}

#[no_mangle]
pub(crate) fn write_period_sec(period_sec: u32) {
    unsafe {
        storage_write_i32_be(0, 0, period_sec, 4);
    }
}

#[no_mangle]
pub(crate) fn write_lockup_time(lockup_time_sec: u32) {
    unsafe {
        storage_write_i32_be(0, 0, lockup_time_sec, 4);
    }
}
