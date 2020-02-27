include!("imports.rs");

#[no_mangle]
pub(crate) fn read_pending_pub_key(reg_bits: u32, reg_idx: u32) {
    read_pub_key(3, reg_bits, reg_idx);
}

#[no_mangle]
pub(crate) fn read_current_layer() -> u64 {
    unsafe { host_ctx_read_i64_be(0) }
}

/// Reads pub-key #{key_idx} into register `reg_bits:reg_idx`
#[no_mangle]
pub(crate) fn read_pub_key(key_idx: u32, reg_bits: u32, reg_idx: u32) {
    assert!(key_idx <= 3);

    let offset = 32 * key_idx;

    unsafe { storage_read_to_reg(0, offset, reg_bits, reg_idx, 0) }
}

#[no_mangle]
pub(crate) fn read_first_layer() -> u64 {
    unsafe { storage_read_i64_be(0, 0, 8) }
}

#[no_mangle]
pub(crate) fn read_last_run_layer() -> u64 {
    unsafe { storage_read_i64_be(0, 0, 8) }
}

#[no_mangle]
pub(crate) fn read_liquidated() -> u32 {
    unsafe { storage_read_i32_be(0, 0, 4) }
}

#[no_mangle]
pub(crate) fn read_unliquidated() -> u32 {
    unsafe { storage_read_i32_be(0, 0, 4) }
}

#[no_mangle]
pub(crate) fn read_layer_liquidation() -> u32 {
    unsafe { storage_read_i32_be(0, 0, 2) }
}
