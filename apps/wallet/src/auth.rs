include!("imports.rs");
include!("constants.rs");

use super::{read::*, write::*};

#[no_mangle]
pub fn is_multisig() -> bool {
    false
}

#[no_mangle]
pub fn pub_key_auth() {
    unsafe {
        reg_push(256, 0);
        reg_push(256, 1);
    }

    // load `HostCtx.public_key` into register `256:0`
    unsafe {
        host_ctx_read_into_reg(PUBLIC_KEY_FIELD, 256, 0);
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
pub fn auth() -> i32 {
    pub_key_auth();

    // store regs
    unsafe {
        reg_push(256, 0);
        reg_push(256, 1);
    }

    //  load `pending_pub_key` into register `256:0`
    read_pending_pub_key(256, 0);

    // 2) if its all zeros:
    //   2.1) `write_pending_pub_key();`
    //   2.2)  return `1` signifying `multisig process isn't complete`
    //         else, goto 3)

    // load HostCtx `pub-key` into register `256:1`
    unsafe {
        host_ctx_read_into_reg(PUBLIC_KEY_FIELD, 256, 1);
    }

    let status = pub_key_cmp(0, 1);

    if status == 0 {
        unsafe {
            // registers are equals
            reg_push(256, 0);

            // this will zero register `256:0`
            reg_set_i32_be(256, 0, 4);

            // zeroing the `last_pub_key` since we copy the contents of register `256:0`
            storage_write_from_reg(256, 0, PAGE_IDX, PENDING_PUB_KEY_OFFSET, PUB_KEY_SIZE);

            reg_pop(256, 0);
        }
    } else {
        write_pending_pub_key();
    }

    // store regs
    unsafe {
        reg_pop(256, 0);
        reg_pop(256, 1);
    }

    status
}

#[no_mangle]
fn pub_key_cmp(reg_idx1: u32, reg_idx2: u32) -> i32 {
    unsafe { reg_cmp(256, reg_idx1, reg_idx2) }
}
