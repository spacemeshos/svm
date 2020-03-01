include!("imports.rs");

use crate::{hostctx, offset, sizeof};

#[allow(non_upper_case_globals)]
static page_idx: u32 = 0;

#[no_mangle]
pub(crate) fn read_is_multisig() -> u32 {
    unsafe { storage_read_i32_be(page_idx, offset!(is_multisig), sizeof!(is_multisig)) }
}

#[no_mangle]
pub(crate) fn read_pending_pub_key(reg_bits: u32, reg_idx: u32) {
    read_pub_key(offset!(pending_pub_key), reg_bits, reg_idx);
}

#[no_mangle]
pub(crate) fn read_current_layer() -> u64 {
    unsafe { host_ctx_read_i64_be(hostctx!(layer)) }
}

/// Reads pub-key #{key_idx} into register `reg_bits:reg_idx`
#[no_mangle]
pub(crate) fn read_pub_key(offset: u32, reg_bits: u32, reg_idx: u32) {
    unsafe { storage_read_to_reg(page_idx, offset, reg_bits, reg_idx, sizeof!(pub_key)) }
}

#[no_mangle]
pub(crate) fn read_first_layer() -> u64 {
    unsafe { storage_read_i64_be(page_idx, offset!(first_layer), sizeof!(layer)) }
}

#[no_mangle]
pub(crate) fn read_last_run_layer() -> u64 {
    unsafe { storage_read_i64_be(page_idx, offset!(last_run_layer), sizeof!(layer)) }
}

#[no_mangle]
pub(crate) fn read_liquidated() -> u32 {
    unsafe { storage_read_i32_be(page_idx, offset!(liquidated), sizeof!(liquidated)) }
}

#[no_mangle]
pub(crate) fn read_unliquidated() -> u32 {
    unsafe { storage_read_i32_be(page_idx, offset!(unliquidated), sizeof!(unliquidated)) }
}

#[no_mangle]
pub(crate) fn read_transferred() -> u32 {
    unsafe { storage_read_i32_be(page_idx, offset!(transferred), sizeof!(transferred)) }
}

#[no_mangle]
pub(crate) fn read_layer_liquidation() -> u32 {
    unsafe {
        storage_read_i32_be(
            page_idx,
            offset!(layer_liquidation),
            sizeof!(layer_liquidation),
        )
    }
}

#[no_mangle]
pub(crate) fn read_period_sec() -> u32 {
    unsafe { storage_read_i32_be(page_idx, offset!(period_sec), sizeof!(period_sec)) }
}

#[no_mangle]
pub(crate) fn read_lockup_time_sec() -> u32 {
    unsafe { storage_read_i32_be(page_idx, offset!(lockup_sec), sizeof!(lockup_sec)) }
}
