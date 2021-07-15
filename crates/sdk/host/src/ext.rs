use crate::traits::Host;

use svm_sdk_alloc::Ptr;
use svm_sdk_types::{Address, Amount, LayerId};

use core::mem::MaybeUninit;

/// ### `offset` meaning in this file:
///
/// The parameter `offset` here denotes a memory address integer serving as a pointer to a cell
/// within the current running WASM instance. Counting is zero-based.

/// ## SVM Imports  
///
/// WASM Imports under the `svm` namespace `svm` (Node-agnostic)
/// Each running SVM transaction (a.k.a `Call Account) can assume these imports exist regardless of
/// the additional imports which are `Node aware`.

#[link(wasm_import_module = "svm")]
extern "C" {
    /// Returns the memory offset where the transaction's input `calldata` starts.
    fn svm_calldata_offset() -> u32;

    /// Returns the transaction's input `calldata` byte-length.
    fn svm_calldata_len() -> u32;

    /// Signals to SVM that the current running transaction output (a.k.a `returndata`)
    /// lays out in memory starting from offset `offset` and its byte-length is `length`.
    ///
    /// * Not calling that method during a transaction execution will result in an empty `returndata`.
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
    /// Returns the `value` field of the current executed transaction.
    fn sm_value() -> u64;

    /// Returns the currently executed `Account`'s (a.k.a the `target`) balance.
    fn sm_balance() -> u64;

    /// Receives an offset to allocated `Address` (`Address::len()` of bytes).
    /// The `Node` will copy the `Address` of the transaction's `Principal`
    /// starting at offset `offset`.
    fn sm_principal(offset: u32);

    /// Receives an offset to allocated `Address` (`Address::len()` of bytes).
    /// The `Node` will copy the `Address` of the transaction's `Target` (the currently executing `Account`)
    /// starting at offset `offset`.
    fn sm_target(offset: u32);

    /// Returns the Spacemesh layer the current executed transaction is running at.
    fn sm_layer() -> u64;

    /// Transfers `amount` coins from the currently executed `Account` (the `target`)
    /// to the account ("the destination") which is address is starts offset `dst_offset` (`Address::len()` of bytes).
    fn sm_transfer(dst_offset: u32, amount: u64);
}

/// Regarding why we don't use any concurrency primitives for initializing `HOST` see the explanation of `MockHost`.
static mut INITIALIZED: bool = false;

static mut HOST: MaybeUninit<InnerHost> = MaybeUninit::uninit();

/// Implements the `Host` trait.
/// Its methods delegate work to the singleton `InnerHost` instance
/// which also implements the `Host` trait and contains the actual implementation of the `Host` trait.
///
/// In order to get access to this singleton instance the API user should use `ExtHost::instance()`
/// when running in non-test environment. Otherwise, a run-time linking error will be raised since the linker
/// won't know how to link these `extern "C"` functions above. (see `MockHost` for running when at test environment).
pub struct ExtHost;

impl ExtHost {
    pub fn instance() -> &'static mut InnerHost {
        unsafe {
            if !INITIALIZED {
                HOST = MaybeUninit::new(InnerHost::new());

                INITIALIZED = true;
            };

            core::mem::transmute(HOST.as_mut_ptr())
        }
    }

    #[inline]
    pub fn value() -> Amount {
        let host = Self::instance();
        host.value()
    }
}

impl Host for ExtHost {
    #[inline]
    fn calldata(&self) -> &'static [u8] {
        let host = Self::instance();
        host.calldata()
    }

    #[inline]
    fn set_returndata(&mut self, bytes: &[u8]) {
        let host = Self::instance();
        host.set_returndata(bytes);
    }

    #[inline]
    fn value(&self) -> Amount {
        let host = Self::instance();
        host.value()
    }

    #[inline]
    fn principal(&self) -> Address {
        let host = Self::instance();
        host.principal()
    }

    #[inline]
    fn target(&self) -> Address {
        let host = Self::instance();
        host.target()
    }

    #[inline]
    fn layer_id(&self) -> LayerId {
        let host = Self::instance();
        host.layer_id()
    }

    #[inline]
    fn balance(&self) -> Amount {
        let host = Self::instance();

        host.balance()
    }

    #[inline]
    fn transfer(&mut self, dst: &Address, amount: Amount) {
        let host = Self::instance();
        host.transfer(&dst, amount);
    }

    #[inline]
    fn log(&mut self, msg: &str, code: u8) {
        let host = Self::instance();
        host.log(msg, code);
    }
}

pub struct InnerHost;

impl Host for InnerHost {
    #[inline]
    fn calldata(&self) -> &'static [u8] {
        unsafe {
            let offset = svm_calldata_offset();
            let len = svm_calldata_len() as _;
            let bytes: &[u8] = core::slice::from_raw_parts(offset as *const u8, len);

            core::mem::transmute(bytes)
        }
    }

    #[inline]
    fn set_returndata(&mut self, bytes: &[u8]) {
        unsafe {
            let offset = bytes.as_ptr() as u32;
            let length = bytes.len() as u32;

            svm_set_returndata(offset, length);
        }
    }

    #[inline]
    fn value(&self) -> Amount {
        unsafe {
            let value = sm_value();
            Amount(value)
        }
    }

    #[inline]
    fn principal(&self) -> Address {
        unsafe {
            let offset = self.alloc_addr();

            sm_principal(offset);

            offset.into()
        }
    }

    #[inline]
    fn target(&self) -> Address {
        unsafe {
            let offset = self.alloc_addr();

            sm_target(offset);

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
    fn balance(&self) -> Amount {
        unsafe {
            let amount = sm_balance();
            Amount(amount)
        }
    }

    #[inline]
    fn transfer(&mut self, dst: &Address, amount: Amount) {
        unsafe {
            let dst = dst.offset() as u32;
            sm_transfer(dst, amount.0);
        }
    }

    #[inline]
    fn log(&mut self, msg: &str, code: u8) {
        unsafe {
            let offset = msg.as_ptr() as u32;
            let len = msg.len() as u32;

            svm_log(offset, len, code as u32)
        }
    }
}

impl InnerHost {
    fn new() -> Self {
        Self {}
    }

    #[inline]
    fn alloc_addr(&self) -> u32 {
        let ptr = svm_sdk_alloc::alloc(Address::len());
        ptr.offset() as u32
    }
}
