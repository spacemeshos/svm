include!("imports.rs");
include!("constants.rs");

use super::{read::*, write::*};

#[no_mangle]
pub fn is_multisig() -> bool {
    false
}

#[no_mangle]
pub fn multisig_prepare() {
    unsafe {
        reg_push(256, 0);
        reg_push(256, 1);
    }

    copy_host_pub_key_to_reg(256, 0);

    for idx in 0..3 {
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
fn pub_key_cmp(reg_idx1: u32, reg_idx2: u32) -> i32 {
    unsafe { reg_cmp(256, reg_idx1, reg_idx2) }
}

#[no_mangle]
pub(crate) fn copy_host_pub_key_to_reg(reg_bits: u32, reg_idx: u32) {
    unsafe {
        host_ctx_read_into_reg(PUBLIC_KEY_FIELD, reg_bits, reg_idx);
    }
}
