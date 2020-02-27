include!("imports.rs");
include!("constants.rs");

use super::{computations, read};

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
pub fn write_pub_keys(is_multisig: u32) {
    unsafe {
        if is_multisig == 0 {
            // store `pub_key1`
            buffer_copy_to_storage(FUNC_BUF_ID, 0, PAGE_IDX, 0, PUB_KEY_SIZE);

            // store: `is_multisig=0`
            storage_write_i32_be(PAGE_IDX, IS_MULTISIG_OFFSET, 0, 1);
        } else {
            // storing `pub_key1, pub_key2, pub_key3`
            // we copy the keys at one shot,
            // since they are laid contagiously at both input func-buffer and app-storage
            buffer_copy_to_storage(FUNC_BUF_ID, 0, PAGE_IDX, 0, PUB_KEY_SIZE * 3);

            // store: `is_multisig=1`
            storage_write_i32_be(PAGE_IDX, IS_MULTISIG_OFFSET, 1, 1);
        }
    }
}

#[no_mangle]
pub fn write_first_layer() {
    unsafe {
        let layer = read::read_current_layer();

        storage_write_i64_be(PAGE_IDX, FIRST_LAYER_OFFSET, layer, 8);

        // set init `last_run_layer` with `first_layer`
        write_last_run_layer(layer);
    }
}

#[no_mangle]
pub fn write_layer_liquidation(unliquidated: u32, period_sec: u32) {
    unsafe {
        let layer = host_ctx_read_i64_be(LAYER_ID_FIELD);
        let layer_time_sec = host_ctx_read_i32_be(LAYER_TIME_FIELD);

        let layer_count = computations::layer_count(period_sec, layer_time_sec);
        let layer_liq = computations::layer_liquidation(unliquidated, layer_count);

        assert!(layer_liq <= 0xFFFF);

        storage_write_i32_be(PAGE_IDX, LAYER_LIQ_OFFSET, layer_liq, 2);
    }
}

// persist `HostCtx pub_key` into app-storage `last_pub_key`
#[no_mangle]
pub fn write_pending_pub_key() {
    unsafe {
        reg_push(256, 0);
        host_ctx_read_into_reg(PUBLIC_KEY_FIELD, 256, 0);
        storage_write_from_reg(256, 0, PAGE_IDX, PENDING_PUB_KEY_OFFSET, PUB_KEY_SIZE);
        reg_pop(256, 0);
    }
}

#[no_mangle]
pub fn write_liquidated(liquidated: u32) {
    unsafe {
        storage_write_i32_be(PAGE_IDX, LIQUIDATED_OFFSET, liquidated, 4);
    }
}

#[no_mangle]
pub fn write_unliquidated(unliquidated: u32) {
    unsafe {
        storage_write_i32_be(PAGE_IDX, UNLIQUIDATED_OFFSET, unliquidated, 4);
    }
}

#[no_mangle]
pub fn write_last_run_layer(layer: u64) {
    unsafe {
        storage_write_i64_be(PAGE_IDX, LAST_RUN_LAYER_OFFSET, layer, 8);
    }
}
