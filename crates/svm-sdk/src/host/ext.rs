use crate::host::Host;
use crate::memory;
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

    fn svm_returndata(offset: u32, length: u32);

    fn svm_log(offset: u32, length: u32, code: u32);
}

#[link(wasm_import_module = "host")]
extern "C" {
    fn host_balance(offset: u32) -> u64;

    fn host_sender(offset: u32);

    fn host_app(offset: u32);

    fn host_layer() -> u64;

    fn host_transfer(dst_offset: u32, amount: u64);
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
    fn set_returndata(&self, bytes: &[u8]) {
        let host = host();

        host.set_returndata(bytes);
    }

    #[inline]
    fn sender(&self) -> Address {
        let host = host();

        host.sender()
    }

    #[inline]
    fn app(&self) -> Address {
        let host = host();

        host.app()
    }

    #[inline]
    fn layer_id(&self) -> LayerId {
        let host = host();

        host.layer_id()
    }

    #[inline]
    fn balance_of(&self, addr: &Address) -> Amount {
        let host = host();

        host.balance_of(&addr)
    }

    #[inline]
    fn transfer(&self, dst: &Address, amount: Amount) {
        let host = host();

        host.transfer(&dst, amount);
    }

    #[inline]
    fn log(&self, msg: &str, code: u8) {
        let host = host();

        host.log(msg, code);
    }
}

struct InnerHost;

impl Host for InnerHost {
    #[inline]
    fn get_calldata(&self) -> &'static [u8] {
        unsafe {
            let offset = svm_calldata_offset();
            let len = svm_calldata_len() as _;

            core::slice::from_raw_parts(offset as *const u8, len)
        }
    }

    #[inline]
    fn set_returndata(&self, bytes: &[u8]) {
        unsafe {
            let offset = bytes.as_ptr() as u32;
            let length = bytes.len() as u32;

            svm_returndata(offset, length);
        }
    }

    #[inline]
    fn sender(&self) -> Address {
        unsafe {
            let offset = self.alloc_addr();

            host_sender(offset);

            offset.into()
        }
    }

    #[inline]
    fn app(&self) -> Address {
        unsafe {
            let offset = self.alloc_addr();

            host_app(offset);

            offset.into()
        }
    }

    #[inline]
    fn layer_id(&self) -> LayerId {
        unsafe {
            let id = host_layer();

            LayerId(id)
        }
    }

    #[inline]
    fn balance_of(&self, addr: &Address) -> Amount {
        unsafe {
            let offset = addr.offset() as u32;

            let amount = host_balance(offset);

            Amount(amount)
        }
    }

    #[inline]
    fn transfer(&self, dst: &Address, amount: Amount) {
        unsafe {
            let dst = dst.offset() as u32;

            host_transfer(dst, amount.0);
        }
    }

    #[inline]
    fn log(&self, msg: &str, code: u8) {
        unsafe {
            let offset = msg.as_ptr() as u32;
            let len = msg.len() as u32;

            svm_log(offset, len, code as u32)
        }
    }
}

impl InnerHost {
    #[inline]
    fn alloc_addr(&self) -> u32 {
        memory::alloc(Address::len()) as u32
    }
}
