include!("imports.rs");

use super::{read::*, write::*};

use crate::{hostctx, offset, sizeof};

#[no_mangle]
pub(crate) fn is_multisig() -> bool {
    match read_is_multisig() {
        0 => false,
        _ => true,
    }
}

#[no_mangle]
pub(crate) fn pub_key_auth() {
    assert!(is_multisig() == false);

    let reg_bits = sizeof!(pub_key) * 8;

    unsafe {
        reg_push(reg_bits, 0);
        reg_push(reg_bits, 1);

        copy_host_pub_key_to_reg(reg_bits, 0);
        read_pub_key(offset!(pub_key1), reg_bits, 1);

        if pub_key_cmp(0, 1) == 0 {
            panic!("auth failed!")
        }

        reg_pop(reg_bits, 1);
        reg_pop(reg_bits, 0);
    }
}

#[no_mangle]
pub(crate) fn multisig_start() {
    assert!(is_multisig());

    multisig_any_key_auth();
    write_pending_pub_key();
}

#[no_mangle]
pub(crate) fn multisig_complete() {
    assert!(is_multisig());

    multisig_any_key_auth();

    let reg_bits = sizeof!(pub_key) * 8;

    /// we need to assert that the current `pub_key` is different
    /// from the persisted `pending_pub_key`.
    /// If they keys differ - we conclude the multisig auth as a success
    /// and we zero the `pending_pub_key.`
    unsafe {
        reg_push(reg_bits, 0);
        reg_push(reg_bits, 1);

        copy_host_pub_key_to_reg(reg_bits, 0);
        read_pending_pub_key(reg_bits, 1);

        if pub_key_cmp(0, 1) == 0 {
            panic!("auth failed!")
        }

        reset_pending_pub_key();

        reg_pop(reg_bits, 1);
        reg_pop(reg_bits, 0);
    };
}

/// Private

#[no_mangle]
fn multisig_any_key_auth() {
    let reg_bits = sizeof!(pub_key) * 8;

    unsafe {
        reg_push(reg_bits, 0);
        reg_push(reg_bits, 1);
    }

    copy_host_pub_key_to_reg(reg_bits, 0);

    for idx in 0..3 {
        read_pub_key(offset!(pub_key, idx), reg_bits, 1);

        if pub_key_cmp(0, 1) == 0 {
            // we've found a match

            // restore regs
            unsafe {
                reg_pop(reg_bits, 1);
                reg_pop(reg_bits, 0);
            };

            return;
        }
    }

    panic!("auth failed")
}

#[no_mangle]
fn pub_key_cmp(reg_idx1: u32, reg_idx2: u32) -> i32 {
    unsafe { reg_cmp(sizeof!(pub_key) * 8, reg_idx1, reg_idx2) }
}

#[no_mangle]
fn copy_host_pub_key_to_reg(reg_bits: u32, reg_idx: u32) {
    unsafe {
        host_ctx_read_into_reg(hostctx!(pub_key), reg_bits, reg_idx);
    }
}
