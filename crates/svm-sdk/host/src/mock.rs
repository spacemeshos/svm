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
    pub fn instance() -> &'static InnerHost {
        unsafe {
            INIT.call_once(|| {
                HOST = MaybeUninit::new(InnerHost::new());
            });

            std::mem::transmute(HOST.as_ptr())
        }
    }
}

impl Host for MockHost {
    fn get_calldata(&self) -> &'static [u8] {
        let host = Self::instance();

        host.get_calldata()
    }

    fn set_returndata(&self, bytes: &[u8]) {
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

    fn transfer(&self, dst: &Address, amount: Amount) {
        let host = Self::instance();

        host.transfer(dst, amount);
    }

    fn log(&self, msg: &str, code: u8) {
        let host = Self::instance();

        host.log(msg, code);
    }
}

pub struct InnerHost {
    pub calldata: RefCell<Option<&'static [u8]>>,

    pub returndata: RefCell<Option<Vec<u8>>>,

    pub accounts: RefCell<HashMap<Address, Amount>>,

    pub sender: RefCell<Option<Address>>,

    pub app: RefCell<Option<Address>>,

    pub layer_id: RefCell<Option<LayerId>>,

    pub logs: RefCell<Vec<(String, u8)>>,
}

impl InnerHost {
    fn new() -> Self {
        Self {
            calldata: RefCell::new(None),
            returndata: RefCell::new(None),
            sender: RefCell::new(None),
            app: RefCell::new(None),
            accounts: RefCell::new(HashMap::default()),
            layer_id: RefCell::new(None),
            logs: RefCell::new(Vec::new()),
        }
    }

    pub fn set_calldata<T>(&self, calldata: T)
    where
        T: svm_abi_encoder::Encoder,
    {
        let mut bytes = Vec::new();
        calldata.encode(&mut bytes);

        let bytes: &'static [u8] = bytes.leak();

        self.set_raw_calldata(bytes);
    }

    pub fn set_raw_calldata(&self, bytes: &'static [u8]) {
        *self.calldata.borrow_mut() = Some(bytes);
    }

    pub fn get_returndata(&self) -> Option<Vec<u8>> {
        self.returndata.borrow().clone()
    }

    pub fn set_balance(&self, addr: &Address, amount: Amount) {
        self.accounts.borrow_mut().insert(addr.clone(), amount);
    }

    pub fn set_sender(&self, sender: Address) {
        *self.sender.borrow_mut() = Some(sender);
    }

    pub fn set_app(&self, app: Address) {
        *self.app.borrow_mut() = Some(app);
    }

    pub fn set_layer_id(&self, layer_id: LayerId) {
        *self.layer_id.borrow_mut() = Some(layer_id);
    }

    pub fn get_logs(&self) -> Vec<(String, u8)> {
        self.logs.borrow().clone()
    }

    pub fn reset(&self) {
        *self.calldata.borrow_mut() = None;
        *self.returndata.borrow_mut() = None;
        *self.sender.borrow_mut() = None;
        *self.app.borrow_mut() = None;
        *self.layer_id.borrow_mut() = None;
        self.logs.borrow_mut().clear();
    }
}

impl Host for InnerHost {
    fn get_calldata(&self) -> &'static [u8] {
        self.calldata.borrow().unwrap()
    }

    fn set_returndata(&self, bytes: &[u8]) {
        *self.returndata.borrow_mut() = Some(bytes.to_vec());
    }

    fn sender(&self) -> Address {
        self.sender.borrow().unwrap().clone()
    }

    fn app(&self) -> Address {
        self.app.borrow().unwrap().clone()
    }

    fn layer_id(&self) -> LayerId {
        self.layer_id.borrow().unwrap()
    }

    fn balance_of(&self, addr: &Address) -> Amount {
        *self.accounts.borrow().get(addr).unwrap_or(&Amount(0))
    }

    fn transfer(&self, dst: &Address, amount: Amount) {
        let app_balance = self.app_balance();

        assert!(app_balance >= amount);

        let dst_balance = self.balance_of(dst);

        let src_balance = app_balance - amount;
        let dst_balance = dst_balance + amount;

        let src = self.app();

        {
            self.accounts.borrow_mut().insert(src, src_balance);
        }

        {
            self.accounts.borrow_mut().insert(dst.clone(), dst_balance);
        }
    }

    fn log(&self, msg: &str, code: u8) {
        let log = (msg.to_string(), code);

        self.logs.borrow_mut().push(log);
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
