use crate::traits::Host;
use svm_sdk_types::{Address, Amount, LayerId};

extern crate core;

extern crate alloc;
extern crate std;

use alloc::string::{String, ToString};
use core::cell::RefCell;

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::Once;
use std::vec::Vec;

static INIT: Once = Once::new();

static mut HOST: MaybeUninit<InnerHost> = MaybeUninit::uninit();

pub struct MockHost;

impl MockHost {
    pub fn instance() -> &'static mut InnerHost {
        unsafe {
            INIT.call_once(|| {
                HOST = MaybeUninit::new(InnerHost::new());
            });

            std::mem::transmute(HOST.as_mut_ptr())
        }
    }
}

impl Host for MockHost {
    fn get_calldata(&self) -> &'static [u8] {
        let host = Self::instance();

        host.get_calldata()
    }

    fn set_returndata(&mut self, bytes: &[u8]) {
        let host = Self::instance();

        host.set_returndata(bytes);
    }

    fn sender(&self) -> Address {
        let host = Self::instance();

        host.sender()
    }

    fn app(&self) -> Address {
        let host = Self::instance();

        host.app()
    }

    fn layer_id(&self) -> LayerId {
        let host = Self::instance();

        host.layer_id()
    }

    fn balance_of(&self, addr: &Address) -> Amount {
        let host = Self::instance();

        host.balance_of(addr)
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

    pub returndata: Option<Vec<u8>>,

    pub accounts: HashMap<Address, Amount>,

    pub sender: Option<Address>,

    pub app: Option<Address>,

    pub layer_id: Option<LayerId>,

    pub logs: Vec<(String, u8)>,
}

impl InnerHost {
    fn new() -> Self {
        Self {
            calldata: None,
            returndata: None,
            sender: None,
            app: None,
            accounts: HashMap::new(),
            layer_id: None,
            logs: Vec::new(),
        }
    }

    pub fn set_calldata<T>(&mut self, calldata: T)
    where
        T: svm_abi_encoder::Encoder,
    {
        let mut bytes = Vec::new();
        calldata.encode(&mut bytes);

        let bytes: &'static [u8] = bytes.leak();

        self.set_raw_calldata(bytes);
    }

    pub fn set_raw_calldata(&mut self, bytes: &'static [u8]) {
        self.calldata = Some(bytes);
    }

    pub fn get_returndata(&self) -> Option<Vec<u8>> {
        self.returndata.clone()
    }

    pub fn set_balance(&mut self, addr: &Address, amount: Amount) {
        self.accounts.insert(addr.clone(), amount);
    }

    pub fn set_sender(&mut self, sender: Address) {
        self.sender = Some(sender);
    }

    pub fn set_app(&mut self, app: Address) {
        self.app = Some(app);
    }

    pub fn set_layer_id(&mut self, layer_id: LayerId) {
        self.layer_id = Some(layer_id);
    }

    pub fn get_logs(&self) -> Vec<(String, u8)> {
        self.logs.clone()
    }

    pub fn reset(&mut self) {
        self.calldata = None;
        self.returndata = None;
        self.sender = None;
        self.app = None;
        self.layer_id = None;
        self.logs.clear();
    }
}

impl Host for InnerHost {
    fn get_calldata(&self) -> &'static [u8] {
        self.calldata.unwrap()
    }

    fn set_returndata(&mut self, bytes: &[u8]) {
        self.returndata = Some(bytes.to_vec());
    }

    fn sender(&self) -> Address {
        self.sender.unwrap().clone()
    }

    fn app(&self) -> Address {
        self.app.unwrap().clone()
    }

    fn layer_id(&self) -> LayerId {
        self.layer_id.unwrap()
    }

    fn balance_of(&self, addr: &Address) -> Amount {
        *self.accounts.get(addr).unwrap_or(&Amount(0))
    }

    fn transfer(&mut self, dst: &Address, amount: Amount) {
        let app_balance = self.app_balance();

        assert!(app_balance >= amount);

        let dst_balance = self.balance_of(dst);

        let src_balance = app_balance - amount;
        let dst_balance = dst_balance + amount;

        let src = self.app();

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

            let calldata = host.get_calldata();
            assert_eq!(calldata, b"Hello World!");
        });
    }

    #[test]
    fn host_returndata() {
        test(|| {
            let host = MockHost::instance();

            let returndata = host.get_returndata();
            assert_eq!(returndata, None);

            let returndata = b"Done.";
            host.set_returndata(returndata);

            let returndata = host.get_returndata().unwrap();
            assert_eq!(returndata, b"Done.");
        });
    }

    #[test]
    fn host_accounts() {
        test(|| {
            let host = MockHost::instance();

            let addr1: Address = [0x10; 20].into();
            let addr2: Address = [0x20; 20].into();

            host.set_balance(&addr1, Amount(10));
            host.set_balance(&addr2, Amount(20));

            let amount1 = host.balance_of(&addr1);
            let amount2 = host.balance_of(&addr2);

            assert_eq!(amount1, Amount(10));
            assert_eq!(amount2, Amount(20));
        });
    }

    #[test]
    fn host_transfer() {
        test(|| {
            let host = MockHost::instance();

            let src: Address = [0x10; 20].into();
            let dst: Address = [0x20; 20].into();

            host.set_app(src);

            host.set_balance(&src, Amount(10));
            host.set_balance(&dst, Amount(20));

            let amount1 = host.balance_of(&src);
            let amount2 = host.balance_of(&dst);

            assert_eq!(amount1, Amount(10));
            assert_eq!(amount2, Amount(20));

            host.transfer(&dst, Amount(5));

            let amount1 = host.balance_of(&src);
            let amount2 = host.balance_of(&dst);

            assert_eq!(amount1, Amount(10 - 5));
            assert_eq!(amount2, Amount(20 + 5));
        });
    }

    #[test]
    fn host_layer() {
        test(|| {
            let host = MockHost::instance();

            host.set_layer_id(LayerId(10));

            let layer = host.layer_id();
            assert_eq!(layer, LayerId(10));
        });
    }

    #[test]
    fn host_logs() {
        test(|| {
            let host = MockHost::instance();

            let logs = host.get_logs();
            assert!(logs.is_empty());

            host.log("Log #1", 100);
            host.log("Log #2", 200);

            let logs = host.get_logs();
            assert_eq!(
                logs,
                vec![("Log #1".to_string(), 100), ("Log #2".to_string(), 200)]
            )
        });
    }
}
