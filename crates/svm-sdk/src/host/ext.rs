include!("../externs.rs");

use crate::{Amount, LayerId};

use crate::value::Address;

const HOST_LAYER_ID: u32 = 0;
const HOST_BALANCE: u32 = 1;

pub struct Host;

impl Host {
    #[inline]
    pub fn get_calldata() -> &'static [u8] {
        unsafe {
            let ptr = svm_calldata_offset();
            let len = svm_calldata_len() as _;

            core::slice::from_raw_parts(ptr as *const u8, len)
        }
    }

    pub fn now() -> LayerId {
        unsafe {
            let layer = svm_host_get64(HOST_LAYER_ID);

            LayerId(layer)
        }
    }

    pub fn balance() -> Amount {
        unsafe {
            let amount = svm_host_get64(HOST_BALANCE);

            Amount(amount)
        }
    }

    pub fn balance_of(addr: &Address) -> Amount {
        todo!()
    }

    pub fn transfer(dst: &Address, amount: Amount) {
        unsafe {
            let dst_ptr = dst.as_ptr() as u32;

            svm_transfer(dst_ptr, amount.0);
        }
    }

    pub fn log(msg: &str, code: u8) {
        unsafe {
            let ptr = msg.as_ptr() as u32;
            let len = msg.len() as u32;

            svm_log(ptr, len, code as u32)
        }
    }
}
