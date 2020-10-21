use crate::host::Host;
use crate::{Address, Amount, LayerId};

extern crate core;

extern crate alloc;
extern crate std;

use alloc::string::{String, ToString};
use core::cell::RefCell;

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::vec::Vec;

use lazy_static::lazy_static;

lazy_static! {
    static ref HOST: Mutex<InnerHost> = {
        let host = InnerHost::new();

        Mutex::new(host)
    };
}

#[inline]
fn host() -> MutexGuard<'static, InnerHost> {
    HOST.lock().unwrap()
}

pub struct MockHost;

impl MockHost {
    fn instance() -> MutexGuard<'static, InnerHost> {
        host()
    }
}

impl Host for MockHost {
    fn get_calldata(&self) -> &'static [u8] {
        let host = host();

        host.get_calldata()
    }

    fn sender(&self) -> Address {
        let host = host();

        host.sender()
    }

    fn app_addr(&self) -> Address {
        let host = host();

        host.app_addr()
    }

    fn layer_id(&self) -> LayerId {
        let host = host();

        host.layer_id()
    }

    fn balance_of(&self, addr: Address) -> Amount {
        let host = host();

        host.balance_of(addr)
    }

    fn transfer(&self, dst: Address, amount: Amount) {
        let host = host();

        host.transfer(dst, amount);
    }

    fn log(&self, msg: &str, code: u8) {
        let host = host();

        host.log(msg, code);
    }

    fn get_logs(&self) -> Vec<(String, u8)> {
        let host = host();

        host.get_logs()
    }
}

struct InnerHost {
    pub calldata: RefCell<Option<&'static [u8]>>,

    pub accounts: RefCell<HashMap<Address, Amount>>,

    pub sender: RefCell<Option<Address>>,

    pub app_addr: RefCell<Option<Address>>,

    pub layer_id: RefCell<Option<LayerId>>,

    pub logs: RefCell<Vec<(String, u8)>>,
}

unsafe impl Send for InnerHost {}

impl InnerHost {
    fn new() -> Self {
        Self {
            calldata: RefCell::new(None),
            sender: RefCell::new(None),
            app_addr: RefCell::new(None),
            accounts: RefCell::new(HashMap::default()),
            layer_id: RefCell::new(None),
            logs: RefCell::new(Vec::new()),
        }
    }

    fn set_calldata(&self, bytes: &'static [u8]) {
        *self.calldata.borrow_mut() = Some(bytes);
    }

    fn set_sender(&self, sender: Address) {
        *self.sender.borrow_mut() = Some(sender);
    }

    fn set_app_addr(&self, app_addr: Address) {
        *self.app_addr.borrow_mut() = Some(app_addr);
    }

    fn set_layer_id(&self, layer_id: LayerId) {
        *self.layer_id.borrow_mut() = Some(layer_id);
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Host for InnerHost {
    fn get_calldata(&self) -> &'static [u8] {
        self.calldata.borrow().unwrap()
    }

    fn sender(&self) -> Address {
        self.sender.borrow().unwrap().clone()
    }

    fn app_addr(&self) -> Address {
        self.app_addr.borrow().unwrap().clone()
    }

    fn layer_id(&self) -> LayerId {
        self.layer_id.borrow().unwrap()
    }

    fn balance_of(&self, addr: Address) -> Amount {
        let sender = self.sender();

        *self.accounts.borrow().get(&sender).unwrap_or(&Amount(0))
    }

    fn transfer(&self, dst: Address, amount: Amount) {
        let app_balance = self.app_balance();

        assert!(app_balance >= amount);

        let dst_balance = self.balance_of(dst.clone());

        let src_balance = app_balance - amount;
        let dst_balance = dst_balance + amount;

        let src = self.app_addr();

        {
            self.accounts.borrow_mut().insert(src, src_balance);
        }

        {
            self.accounts.borrow_mut().insert(dst, dst_balance);
        }
    }

    fn log(&self, msg: &str, code: u8) {
        let log = (msg.to_string(), code);

        self.logs.borrow_mut().push(log);
    }

    fn get_logs(&self) -> Vec<(String, u8)> {
        self.logs.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate alloc;

    use alloc::vec;

    lazy_static! {
        static ref TEST_LOCK: Mutex<()> = Mutex::new(());
    }

    fn test(f: fn() -> ()) {
        let guard = TEST_LOCK.lock().unwrap();

        MockHost::instance().reset();

        f();

        drop(guard);
    }

    #[test]
    fn host_mock_calldata() {
        test(|| {
            let buf = b"Hello World!";
            MockHost::instance().set_calldata(buf);

            let buf = MockHost::instance().get_calldata();
            assert_eq!(buf, b"Hello World!");
        });
    }

    //     #[test]
    //     fn host_accounts_balance() {
    //         test(|| {
    //             let addr1: Address = [0x10; 20].into();
    //             let addr2: Address = [0x20; 20].into();

    //             MockHost.set_balance(&addr1, Amount(10));
    //             MockHost.set_balance(&addr2, Amount(20));

    //             let amount1 = MockHost.get_balance(&addr1);
    //             let amount2 = MockHost.get_balance(&addr2);

    //             assert_eq!(amount1, Amount(10));
    //             assert_eq!(amount2, Amount(20));
    //         });
    //     }

    //     #[test]
    //     fn host_transfer() {
    //         test(|| {
    //             let src: Address = [0x10; 20].into();
    //             let dst: Address = [0x20; 20].into();

    //             MockHost.set_sender(src);

    //             MockHost.set_balance(&src, Amount(10));
    //             MockHost.set_balance(&dst, Amount(20));

    //             let amount1 = MockHost.get_balance(&src);
    //             let amount2 = MockHost.get_balance(&dst);
    //             assert_eq!(amount1, Amount(10));
    //             assert_eq!(amount2, Amount(20));

    //             MockHost.transfer(&dst, Amount(5));

    //             let amount1 = MockHost.get_balance(&src);
    //             let amount2 = MockHost.get_balance(&dst);
    //             assert_eq!(amount1, Amount(10 - 5));
    //             assert_eq!(amount2, Amount(20 + 5));
    //         });
    //     }

    //     #[test]
    //     fn host_layer() {
    //         test(|| {
    //             MockHost.set_layer(LayerId(10));

    //             let layer = MockHost.now();
    //             assert_eq!(layer, LayerId(10));
    //         });
    //     }

    //     #[test]
    //     fn host_logs() {
    //         test(|| {
    //             let logs = MockHost.get_logs();
    //             assert!(logs.is_empty());

    //             MockHost.log("Log #1", 100);
    //             MockHost.log("Log #2", 200);

    //             let logs = MockHost.get_logs();
    //             assert_eq!(
    //                 logs,
    //                 vec![("Log #1".to_string(), 100), ("Log #2".to_string(), 200)]
    //             )
    //         });
    //     }
}
