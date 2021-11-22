//! This crate implements SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust standard-library) annotation in order to reduce the compiled WASM size.

#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(maybe_uninit_uninit_array)]
#![feature(once_cell)]

use svm_abi_encoder::{ByteSize, Encoder};
use svm_sdk_host::Host;
use svm_sdk_std::Vec;
use svm_sdk_types::{Address, Amount, LayerId};

extern crate alloc;
extern crate core;
extern crate std;

use alloc::string::{String, ToString};
use core::cell::RefCell;

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::Once;

/// Since SVM Transactions run one-by-one there is no need for any concurrency primitives usage.
/// We implement the `Host`'s singleton initialization using `unsafe` tools.
///
/// This pattern is similarly used in other cases throughout this crate.

static INIT: Once = Once::new();

static mut HOST: MaybeUninit<InnerHost> = MaybeUninit::uninit();

pub struct MockHost;

impl MockHost {
    fn instance() -> &'static mut InnerHost {
        unsafe {
            INIT.call_once(|| {
                HOST = MaybeUninit::new(InnerHost::new());
            });

            std::mem::transmute(HOST.as_mut_ptr())
        }
    }

    pub fn set_calldata<T>(calldata: T)
    where
        T: Encoder + ByteSize,
    {
        let host = Self::instance();
        host.set_calldata(calldata)
    }

    pub fn set_raw_calldata(bytes: &[u8]) {
        let host = Self::instance();
        host.set_raw_calldata(bytes)
    }

    pub fn returndata() -> Option<alloc::vec::Vec<u8>> {
        let host = Self::instance();
        host.returndata()
    }

    pub fn set_balance(addr: &Address, amount: Amount) {
        let host = Self::instance();
        host.set_balance(addr, amount);
    }

    pub fn set_returndata(bytes: &[u8]) {
        let host = Self::instance();
        host.set_returndata(bytes);
    }

    pub fn set_value(value: Amount) {
        let host = Self::instance();
        host.set_value(value);
    }

    pub fn set_principal(target: Address) {
        let host = Self::instance();
        host.set_principal(target);
    }

    pub fn set_target(target: Address) {
        let host = Self::instance();
        host.set_target(target);
    }

    pub fn set_layer_id(layer_id: LayerId) {
        let host = Self::instance();
        host.set_layer_id(layer_id);
    }

    pub fn value() -> Amount {
        let host = Self::instance();
        host.value()
    }

    pub fn target() -> Address {
        let host = Self::instance();
        host.target()
    }

    pub fn layer_id() -> LayerId {
        let host = Self::instance();
        host.layer_id()
    }

    pub fn balance() -> Amount {
        let host = Self::instance();
        host.balance()
    }

    pub fn balance_of(addr: &Address) -> Amount {
        let host = Self::instance();
        host.balance_of(addr)
    }

    pub fn transfer(dst: &Address, amount: Amount) {
        let host = Self::instance();
        host.transfer(dst, amount);
    }

    pub fn log(msg: &str, code: u8) {
        let host = Self::instance();
        host.log(msg, code);
    }

    pub fn logs() -> alloc::vec::Vec<(String, u8)> {
        let host = Self::instance();
        host.logs()
    }

    pub fn reset() {
        let host = Self::instance();
        host.reset();
    }
}

impl Host for MockHost {
    fn calldata(&self) -> &'static [u8] {
        let host = Self::instance();
        host.calldata()
    }

    fn set_returndata(&mut self, bytes: &[u8]) {
        let host = Self::instance();
        host.set_returndata(bytes);
    }

    fn value(&self) -> Amount {
        let host = Self::instance();
        host.value()
    }

    fn principal(&self) -> Address {
        let host = Self::instance();
        host.principal()
    }

    fn target(&self) -> Address {
        let host = Self::instance();
        host.target()
    }

    fn layer_id(&self) -> LayerId {
        let host = Self::instance();
        host.layer_id()
    }

    fn balance(&self) -> Amount {
        let host = Self::instance();
        host.balance()
    }

    fn transfer(&mut self, dst: &Address, amount: Amount) {
        let host = Self::instance();
        host.transfer(dst, amount);
    }

    fn log(&mut self, msg: &str, code: u8) {
        let host = Self::instance();
        host.log(msg, code);
    }
}

pub struct InnerHost {
    pub calldata: Option<&'static [u8]>,

    pub returndata: Option<alloc::vec::Vec<u8>>,

    pub accounts: HashMap<Address, Amount>,

    pub value: Option<Amount>,

    pub principal: Option<Address>,

    pub target: Option<Address>,

    pub layer_id: Option<LayerId>,

    pub logs: alloc::vec::Vec<(String, u8)>,
}

impl InnerHost {
    fn new() -> Self {
        Self {
            calldata: None,
            returndata: None,
            value: None,
            principal: None,
            target: None,
            accounts: HashMap::new(),
            layer_id: None,
            logs: alloc::vec::Vec::new(),
        }
    }

    pub fn set_calldata<T>(&mut self, calldata: T)
    where
        T: Encoder + ByteSize,
    {
        let cap = calldata.byte_size();

        let mut bytes = Vec::with_capacity(cap);
        calldata.encode(&mut bytes);

        let bytes: &'static [u8] = bytes.leak();

        self.set_raw_calldata(bytes);
    }

    pub fn set_raw_calldata(&mut self, bytes: &[u8]) {
        let bytes = bytes.to_vec();
        let bytes: &'static [u8] = bytes.leak();

        self.calldata = Some(bytes);
    }

    pub fn returndata(&self) -> Option<alloc::vec::Vec<u8>> {
        self.returndata.clone()
    }

    pub fn set_balance(&mut self, addr: &Address, amount: Amount) {
        self.accounts.insert(addr.clone(), amount);
    }

    pub fn set_value(&mut self, value: Amount) {
        self.value = Some(value);
    }

    pub fn set_principal(&mut self, principal: Address) {
        self.principal = Some(principal);
    }

    pub fn set_target(&mut self, target: Address) {
        self.target = Some(target);
    }

    pub fn set_layer_id(&mut self, layer_id: LayerId) {
        self.layer_id = Some(layer_id);
    }

    pub fn logs(&self) -> alloc::vec::Vec<(String, u8)> {
        self.logs.clone()
    }

    pub fn balance_of(&self, addr: &Address) -> Amount {
        *self.accounts.get(&addr).unwrap_or(&Amount(0))
    }

    pub fn reset(&mut self) {
        self.calldata = None;
        self.returndata = None;
        self.value = None;
        self.principal = None;
        self.target = None;
        self.layer_id = None;
        self.logs.clear();
    }
}

impl Host for InnerHost {
    fn calldata(&self) -> &'static [u8] {
        self.calldata.unwrap()
    }

    fn set_returndata(&mut self, bytes: &[u8]) {
        self.returndata = Some(bytes.to_vec());
    }

    fn value(&self) -> Amount {
        self.value.unwrap().clone()
    }

    fn principal(&self) -> Address {
        self.principal.unwrap().clone()
    }

    fn target(&self) -> Address {
        self.target.unwrap().clone()
    }

    fn layer_id(&self) -> LayerId {
        self.layer_id.unwrap()
    }

    fn balance(&self) -> Amount {
        let target = self.target();
        self.balance_of(&target)
    }

    fn transfer(&mut self, dst: &Address, amount: Amount) {
        let target_balance = self.balance();

        assert!(target_balance >= amount);

        let dst_balance = self.balance_of(dst);

        let src_balance = target_balance - amount;
        let dst_balance = dst_balance + amount;

        let src = self.target();

        self.accounts.insert(src, src_balance);
        self.accounts.insert(dst.clone(), dst_balance);
    }

    fn log(&mut self, msg: &str, code: u8) {
        let log = (msg.to_string(), code);

        self.logs.push(log);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate alloc;

    use alloc::vec;

    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_LOCK: Mutex<()> = Mutex::new(());
    }

    fn test(f: fn() -> ()) {
        // we use a `Mutex` to enforce serial execution of `MockHost`'s tests.
        let guard = TEST_LOCK.lock().unwrap();

        MockHost::instance().reset();

        f();

        drop(guard)
    }

    #[test]
    fn host_calldata() {
        test(|| {
            let host = MockHost::instance();

            let calldata = b"Hello World!";
            host.set_raw_calldata(calldata);

            let calldata = host.calldata();
            assert_eq!(calldata, b"Hello World!");
        });
    }

    #[test]
    fn host_returndata() {
        test(|| {
            let host = MockHost::instance();

            let returndata = host.returndata();
            assert_eq!(returndata, None);

            let returndata = b"Done.";
            host.set_returndata(returndata);

            let returndata = host.returndata().unwrap();
            assert_eq!(returndata, b"Done.");
        });
    }

    #[test]
    fn host_accounts() {
        test(|| {
            let addr1: Address = Address::repeat(0x10).into();
            let addr2: Address = Address::repeat(0x20).into();

            MockHost::set_balance(&addr1, Amount(10));
            MockHost::set_balance(&addr2, Amount(20));

            let amount1 = MockHost::balance_of(&addr1);
            let amount2 = MockHost::balance_of(&addr2);

            assert_eq!(amount1, Amount(10));
            assert_eq!(amount2, Amount(20));
        });
    }

    #[test]
    fn host_transfer() {
        test(|| {
            let target: Address = [0x10; 20].into();
            let dst: Address = [0x20; 20].into();

            MockHost::set_target(target);

            MockHost::set_balance(&target, Amount(10));
            MockHost::set_balance(&dst, Amount(20));

            let amount1 = MockHost::balance();
            let amount2 = MockHost::balance_of(&dst);

            assert_eq!(amount1, Amount(10));
            assert_eq!(amount2, Amount(20));

            MockHost::transfer(&dst, Amount(5));

            let amount1 = MockHost::balance();
            let amount2 = MockHost::balance_of(&dst);

            assert_eq!(amount1, Amount(10 - 5));
            assert_eq!(amount2, Amount(20 + 5));
        });
    }

    #[test]
    fn host_layer() {
        test(|| {
            MockHost::set_layer_id(LayerId(10));

            let layer = MockHost::layer_id();
            assert_eq!(layer, LayerId(10));
        });
    }

    #[test]
    fn host_logs() {
        test(|| {
            let logs = MockHost::logs();
            assert!(logs.is_empty());

            MockHost::log("Log #1", 100);
            MockHost::log("Log #2", 200);

            let logs = MockHost::logs();

            assert_eq!(
                logs,
                vec![("Log #1".to_string(), 100), ("Log #2".to_string(), 200)]
            )
        });
    }
}
