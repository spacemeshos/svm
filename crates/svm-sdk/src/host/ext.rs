use crate::host::Host;
use crate::{Address, Amount, LayerId};

extern crate alloc;
extern crate std;

use alloc::string::String;

use std::sync::{Mutex, MutexGuard};
use std::vec::Vec;

#[link(wasm_import_module = "svm")]
extern "C" {
    fn svm_calldata_offset() -> u32;

    fn svm_calldata_len() -> u32;

    fn svm_host_get64(field: u32) -> u64;

    fn svm_log(msg_ptr: u32, msg_len: u32, code: u32);

    fn svm_load160(var_id: u32, ptr: u32);
}

const HOST_LAYER_ID: u32 = 0;
const HOST_BALANCE: u32 = 1;

#[link(wasm_import_module = "env")]
extern "C" {
    //
}

use lazy_static::lazy_static;

lazy_static! {
    static ref HOST: Mutex<InnerHost> = {
        let host = InnerHost {};

        Mutex::new(host)
    };
}

#[inline]
fn host() -> MutexGuard<'static, InnerHost> {
    HOST.lock().unwrap()
}

pub struct ExtHost;

impl ExtHost {
    fn instance() -> MutexGuard<'static, InnerHost> {
        host()
    }
}

impl Host for ExtHost {
    #[inline]
    fn get_calldata(&self) -> &'static [u8] {
        let host = host();

        host.get_calldata()
    }

    #[inline]
    fn sender(&self) -> Address {
        let host = host();

        host.sender()
    }

    #[inline]
    fn app_addr(&self) -> Address {
        let host = host();

        host.app_addr()
    }

    #[inline]
    fn layer_id(&self) -> LayerId {
        let host = host();

        host.layer_id()
    }

    #[inline]
    fn balance_of(&self, addr: Address) -> Amount {
        let host = host();

        host.balance_of(addr)
    }

    #[inline]
    fn transfer(&self, dst: Address, amount: Amount) {
        let host = host();

        host.transfer(dst, amount);
    }

    #[inline]
    fn log(&self, msg: &str, code: u8) {
        let host = host();

        host.log(msg, code);
    }

    #[inline]
    fn get_logs(&self) -> Vec<(String, u8)> {
        let host = host();

        host.get_logs()
    }
}

struct InnerHost;

impl Host for InnerHost {
    #[inline]
    fn get_calldata(&self) -> &'static [u8] {
        unsafe {
            let ptr = svm_calldata_offset();
            let len = svm_calldata_len() as _;

            core::slice::from_raw_parts(ptr as *const u8, len)
        }
    }

    #[inline]
    fn sender(&self) -> Address {
        todo!()
    }

    #[inline]
    fn app_addr(&self) -> Address {
        todo!()
    }

    #[inline]
    fn layer_id(&self) -> LayerId {
        todo!()
    }

    #[inline]
    fn balance_of(&self, addr: Address) -> Amount {
        todo!()
    }

    #[inline]
    fn transfer(&self, dst: Address, amount: Amount) {
        todo!()
    }

    #[inline]
    fn log(&self, msg: &str, code: u8) {
        unsafe {
            let ptr = msg.as_ptr() as u32;
            let len = msg.len() as u32;

            svm_log(ptr, len, code as u32)
        }
    }

    #[inline]
    fn get_logs(&self) -> Vec<(String, u8)> {
        todo!()
    }
}
