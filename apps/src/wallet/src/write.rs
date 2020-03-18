include!("imports.rs");

use super::{auth, computations, constants, read};

use crate::{hostctx, offset, sizeof};

#[allow(non_upper_case_globals)]
static page_idx: u32 = 0;

#[allow(non_upper_case_globals)]
static buf_idx: u32 = 0;

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
        let is_multisig = if auth::is_multisig() {
            // storing `pub_key1, pub_key2, pub_key3`
            // we copy the keys at one shot,
            // since they are laid contagiously at both input func-buffer and app-storage
            buffer_copy_to_storage(
                buf_idx,
                0, // func-buf `pub_key` offset
                page_idx,
                offset!(pub_key, 0),
                sizeof!(pub_key) * 3,
            );

            1
        } else {
            // store a single `pub_key`

            buffer_copy_to_storage(
                buf_idx,
                0, // func-buf `pub_key` offset
                page_idx,
                offset!(pub_key, 0),
                sizeof!(pub_key),
            );

            0
        };

        storage_write_i32_be(
            page_idx,
            offset!(is_multisig),
            is_multisig,
            sizeof!(is_multisig),
        );
    }
}

#[no_mangle]
pub(crate) fn write_first_layer() {
    unsafe {
        let first_layer = read::read_current_layer();

        storage_write_i64_be(page_idx, offset!(first_layer), first_layer, sizeof!(layer));

        // init `last_run_layer` with `first_layer`
        write_last_run_layer(first_layer);
    }
}

#[no_mangle]
pub(crate) fn write_layer_liquidation(unliquidated: u32, period_sec: u32) {
    unsafe {
        let layer = host_ctx_read_i64_be(hostctx!(layer));
        let layer_time_sec = host_ctx_read_i32_be(hostctx!(layer_time_sec));

        let layer_count = computations::layer_count(period_sec, layer_time_sec);
        let layer_liq = computations::layer_liquidation(unliquidated, layer_count);

        // `layer_liq` should fit into 2 bytes
        assert!(layer_liq <= 0xFFFF);

        storage_write_i32_be(
            page_idx,
            offset!(layer_liquidation),
            layer_liq,
            sizeof!(layer_liquidation),
        );
    }
}

#[no_mangle]
pub(crate) fn write_pending_pub_key() {
    let reg_bits = sizeof!(pub_key) * 8;
    let reg_idx = 0;

    unsafe {
        reg_push(reg_bits, reg_idx);

        host_ctx_read_into_reg(hostctx!(pub_key), reg_bits, reg_idx);

        storage_write_from_reg(
            reg_bits,
            reg_idx,
            page_idx,
            offset!(pending_pub_key),
            sizeof!(pub_key),
        );

        reg_pop(reg_bits, reg_idx);
    }
}

#[no_mangle]
pub(crate) fn reset_pending_pub_key() {
    let reg_bits = sizeof!(pub_key) * 8;
    let reg_idx = 0;

    unsafe {
        reg_push(reg_bits, reg_idx);

        // the side-effect of the following is zero-ing
        // the `{reg_bits}:{reg_idx}` register.
        let value = 0;
        reg_set_i32_be(reg_bits, reg_idx, value);

        storage_write_from_reg(
            reg_bits,
            reg_idx,
            page_idx,
            offset!(pending_pub_key),
            sizeof!(pub_key),
        );

        reg_pop(reg_bits, reg_idx);
    }
}

#[no_mangle]
pub(crate) fn write_liquidated(liquidated: u32) {
    unsafe {
        storage_write_i32_be(
            page_idx,
            offset!(liquidated),
            liquidated,
            sizeof!(liquidated),
        );
    }
}

#[no_mangle]
pub(crate) fn write_unliquidated(unliquidated: u32) {
    unsafe {
        storage_write_i32_be(
            page_idx,
            offset!(unliquidated),
            unliquidated,
            sizeof!(unliquidated),
        );
    }
}

#[no_mangle]
pub(crate) fn write_transferred(transferred: u32) {
    unsafe {
        storage_write_i32_be(
            page_idx,
            offset!(transferred),
            transferred,
            sizeof!(transferred),
        );
    }
}

#[no_mangle]
pub(crate) fn write_last_run_layer(last_run_layer: u64) {
    unsafe {
        storage_write_i64_be(
            page_idx,
            offset!(last_run_layer),
            last_run_layer,
            sizeof!(layer),
        );
    }
}

#[no_mangle]
pub(crate) fn write_period_sec(period_sec: u32) {
    unsafe {
        storage_write_i32_be(
            page_idx,
            offset!(period_sec),
            period_sec,
            sizeof!(period_sec),
        );
    }
}

#[no_mangle]
pub(crate) fn write_lockup_time(lockup_sec: u32) {
    unsafe {
        storage_write_i32_be(
            page_idx,
            offset!(lockup_sec),
            lockup_sec,
            sizeof!(lockup_sec),
        );
    }
}
