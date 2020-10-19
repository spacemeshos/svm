use crate::{Address, Amount, LayerId};

extern crate alloc;
extern crate std;

use alloc::string::{String, ToString};

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

struct InnerHost {
    pub calldata: Option<&'static [u8]>,

    pub accounts: HashMap<Address, Amount>,

    pub sender: Option<Address>,

    pub layer_id: LayerId,

    pub logs: Vec<(String, u8)>,
}

unsafe impl Send for InnerHost {}

impl InnerHost {
    fn new() -> Self {
        Self {
            calldata: None,
            sender: None,
            accounts: HashMap::default(),
            layer_id: LayerId(0),
            logs: Vec::new(),
        }
    }

    fn balance(&self) -> Amount {
        let sender = self.sender();

        self.get_balance(&sender)
    }

    fn get_balance(&self, addr: &Address) -> Amount {
        *self.accounts.get(&addr).unwrap_or(&Amount(0))
    }

    fn set_balance(&mut self, addr: &Address, balance: Amount) {
        todo!()
    }

    fn reset(&mut self) {
        self.calldata = None;
        self.sender = None;
        self.accounts = HashMap::new();
        self.layer_id = LayerId(0);
        self.logs = Vec::new()
    }

    #[inline]
    fn sender(&self) -> &Address {
        self.sender.as_ref().unwrap()
    }

    pub fn get_logs(&self) -> Vec<(String, u8)> {
        self.logs.clone()
    }

    pub fn log(&mut self, msg: &str, code: u8) {
        let log = (msg.to_string(), code);
        self.logs.push(log);
    }
}

pub struct MockHost;

impl MockHost {
    #[inline]
    pub fn get_calldata(&self) -> &'static [u8] {
        let host = host();

        host.calldata.as_ref().unwrap()
    }

    pub fn now(&self) -> LayerId {
        let host = host();

        host.layer_id
    }

    pub fn set_calldata(&self, calldata: &'static [u8]) {
        let mut host = host();

        host.calldata = Some(calldata);
    }

    pub fn set_sender(&self, sender: Address) {
        let mut host = host();

        host.sender = Some(sender);
    }

    pub fn set_layer(&self, layer_id: LayerId) {
        let mut host = host();

        host.layer_id = layer_id;
    }

    pub fn balance(&self) -> Amount {
        let host = host();

        host.balance()
    }

    pub fn get_balance(&self, addr: &Address) -> Amount {
        let host = host();

        host.get_balance(addr)
    }

    pub fn set_balance(&self, addr: &Address, amount: Amount) {
        let mut host = host();

        host.set_balance(addr, amount)
    }

    pub fn transfer(&self, dst: &Address, amount: Amount) {
        let mut host = host();
        let balance = host.balance();

        assert!(balance >= amount);

        let src_balance = balance - amount;
        let dst_balance = host.get_balance(dst) + amount;

        host.set_balance(dst, dst_balance);

        // let src: &Address = host.sender();
        // host.set_balance(src, src_balance);
    }

    pub fn get_logs(&self) -> Vec<(String, u8)> {
        let mut host = host();

        host.get_logs()
    }

    pub fn log(&self, msg: &str, code: u8) {
        let mut host = host();

        host.log(msg, code);
    }

    pub fn reset(&self) {
        let mut host = host();

        host.reset();
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
        MockHost.reset();

        f();

        drop(guard);
    }

    #[test]
    fn host_mock_calldata() {
        test(|| {
            let buf = b"Hello World!";
            MockHost.set_calldata(buf);

            let buf = MockHost.get_calldata();
            assert_eq!(buf, b"Hello World!");
        });
    }

    #[test]
    fn host_accounts_balance() {
        test(|| {
            let addr1: Address = [0x10; 20].into();
            let addr2: Address = [0x20; 20].into();

            MockHost.set_balance(&addr1, Amount(10));
            MockHost.set_balance(&addr2, Amount(20));

            let amount1 = MockHost.get_balance(&addr1);
            let amount2 = MockHost.get_balance(&addr2);

            assert_eq!(amount1, Amount(10));
            assert_eq!(amount2, Amount(20));
        });
    }

    #[test]
    fn host_transfer() {
        test(|| {
            let src: Address = [0x10; 20].into();
            let dst: Address = [0x20; 20].into();

            MockHost.set_sender(src);

            MockHost.set_balance(&src, Amount(10));
            MockHost.set_balance(&dst, Amount(20));

            let amount1 = MockHost.get_balance(&src);
            let amount2 = MockHost.get_balance(&dst);
            assert_eq!(amount1, Amount(10));
            assert_eq!(amount2, Amount(20));

            MockHost.transfer(&dst, Amount(5));

            let amount1 = MockHost.get_balance(&src);
            let amount2 = MockHost.get_balance(&dst);
            assert_eq!(amount1, Amount(10 - 5));
            assert_eq!(amount2, Amount(20 + 5));
        });
    }

    #[test]
    fn host_layer() {
        test(|| {
            MockHost.set_layer(LayerId(10));

            let layer = MockHost.now();
            assert_eq!(layer, LayerId(10));
        });
    }

    #[test]
    fn host_logs() {
        test(|| {
            let logs = MockHost.get_logs();
            assert!(logs.is_empty());

            MockHost.log("Log #1", 100);
            MockHost.log("Log #2", 200);

            let logs = MockHost.get_logs();
            assert_eq!(
                logs,
                vec![("Log #1".to_string(), 100), ("Log #2".to_string(), 200)]
            )
        });
    }
}
