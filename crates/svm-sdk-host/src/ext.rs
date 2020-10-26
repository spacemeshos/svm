use crate::traits::Host;

use svm_sdk_types::{Address, Amount, LayerId};

extern crate alloc;
extern crate std;

use alloc::string::String;

use std::sync::{Mutex, MutexGuard};
use std::vec::Vec;

/// ### `offset` meaning in this file:
///
/// The parameter `offset` here denotes a memory address integer serving as a pointer to a cell
/// within the current running WASM instance. Counting is zero-based.

/// ## SVM Imports  
///
/// WASM Imports under namespace `svm` for SVM programs.
/// Each running SVM app can assume their existence regardless of
/// the additional imports given by the Spacemesh node.
#[link(wasm_import_module = "svm")]
extern "C" {
    /// Returns the memory offset where the transaction's input `calldata` starts.
    fn svm_calldata_offset() -> u32;

    /// Returns the transaction's input `calldata` byte-length.
    fn svm_calldata_len() -> u32;

    /// Signals to SVM that the current running transaction output (a.k.a `returndata`)
    /// lays out in memory starting from offset `offset` and its byte-length is `length`.
    ///
    /// * Not calling that method during app execution will result in an empty `returndata`.
    ///
    /// * Calling this method multiple times - the last call wins.
    fn svm_set_returndata(offset: u32, length: u32);

    /// Sends to SVM the logging message that starts
    /// at memory offset `offset` (of byte-length `length`)
    /// and it's associated message code (for signaling errors severity such as `trace/info/error` etc.)
    fn svm_log(offset: u32, length: u32, code: u32);
}

/// ## Spacemesh Imports
///
/// WASM imports under namespace `sm` for SVM programs targeting Spacemesh Full-Node (i.e `go-spacemesh`).
/// Each running SVM programs under Spacemesh platform can assume their existence.
///
/// If other blockchain projects will want to take SVM and use it for their purposes then they
/// should bring their own imports.
#[link(wasm_import_module = "sm")]
extern "C" {
    /// Receives an account address.
    /// (The `Address::len()` bytes starting at memory offset `offset`)
    ///
    /// Returns the account balance.
    fn sm_balance(offset: u32) -> u64;

    /// Receives an offset to allocated `Address` (`Address::len()` of bytes).
    /// The node will copy the address of the current executed transaction `sender`
    /// starting at offset `offset`.
    fn sm_sender(offset: u32);

    /// Receives an offset to allocated `Address` (`Address::len()` of bytes).
    /// The node will copy the address of the current executed transaction `app`
    /// starting at offset `offset`.
    fn sm_app(offset: u32);

    /// Returns the Spacemesh layer the current executed transaction is running at.
    fn sm_layer() -> u64;

    /// Transfers `amount` coins from the current running `app` ("the source")
    /// to the account ("the destination") which is address is starts offset `dst_offset` (`Address::len()` of bytes).
    fn sm_transfer(dst_offset: u32, amount: u64);
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

/// Implements the `Host` trait.
/// Its methods delegate work to the singleton `InnerHost` instance
/// which also implements the `Host` trait and contains the actual implementation of the `Host` trait.
///
/// In order to get access to this singleton instance the API user should use `ExtHost::instance()`
/// when running in non-test environment. Otherwise, a run-time linking error will be raised since the linker
/// won't know how to link these `extern "C"` functions above. (see `MockHost` for running when at test environment).
pub struct ExtHost;

impl ExtHost {
    pub fn instance() -> MutexGuard<'static, InnerHost> {
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

pub struct InnerHost;

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

            svm_set_returndata(offset, length);
        }
    }

    #[inline]
    fn sender(&self) -> Address {
        unsafe {
            let offset = self.alloc_addr();

            sm_sender(offset);

            offset.into()
        }
    }

    #[inline]
    fn app(&self) -> Address {
        unsafe {
            let offset = self.alloc_addr();

            sm_app(offset);

            offset.into()
        }
    }

    #[inline]
    fn layer_id(&self) -> LayerId {
        unsafe {
            let id = sm_layer();

            LayerId(id)
        }
    }

    #[inline]
    fn balance_of(&self, addr: &Address) -> Amount {
        unsafe {
            let offset = addr.offset() as u32;

            let amount = sm_balance(offset);

            Amount(amount)
        }
    }

    #[inline]
    fn transfer(&self, dst: &Address, amount: Amount) {
        unsafe {
            let dst = dst.offset() as u32;

            sm_transfer(dst, amount.0);
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
        svm_sdk_alloc::alloc(Address::len()) as u32
    }
}
