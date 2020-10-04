use crate::{Amount, LayerId};

extern crate alloc;
extern crate std;

use alloc::string::{String, ToString};

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::vec::Vec;

use lazy_static::lazy_static;

use crate::value::{Address, AddressOwned};

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

    pub accounts: HashMap<AddressOwned, Amount>,

    pub sender: Option<AddressOwned>,

    pub layer_id: LayerId,

    pub logs: Vec<(String, u8)>,
}

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
        let sender: Address = self.sender().deref();

        self.get_balance(&sender)
    }

    fn get_balance(&self, addr: &Address) -> Amount {
        let addr: AddressOwned = addr.to_owned();

        *self.accounts.get(&addr).unwrap_or(&Amount(0))
    }

    fn set_balance(&mut self, addr: AddressOwned, amount: Amount) {
        let entry = self.accounts.entry(addr).or_insert(Amount(0));
        *entry = amount;
    }

    fn reset(&mut self) {
        self.calldata = None;
        self.sender = None;
        self.accounts = HashMap::new();
        self.layer_id = LayerId(0);
        self.logs = Vec::new()
    }

    #[inline]
    fn sender(&self) -> &AddressOwned {
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

pub struct Host;

impl Host {
    #[inline]
    pub fn get_calldata(&self) -> &'static [u8] {
        let host = host();

        host.calldata.as_ref().unwrap()
    }

    pub fn now(&self) -> LayerId {
        let host = host();

        host.layer_id
    }

    pub fn set_func_buf(&self, calldata: &'static [u8]) {
        let mut host = host();

        host.calldata = Some(calldata);
    }

    pub fn set_sender(&self, sender: AddressOwned) {
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

    pub fn set_balance(&self, addr: AddressOwned, amount: Amount) {
        let mut host = host();

        host.set_balance(addr, amount);
    }

    pub fn transfer(&self, dst: &Address, amount: Amount) {
        let mut host = host();
        let balance = host.balance();

        assert!(balance >= amount);

        let src_balance = balance - amount;
        let dst_balance = host.get_balance(dst) + amount;

        let src: AddressOwned = host.sender().clone();
        let dst: AddressOwned = dst.to_owned();

        host.set_balance(src, src_balance);
        host.set_balance(dst, dst_balance);
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
        Host.reset();

        f();

        drop(guard);
    }

    #[test]
    fn host_mock_calldata() {
        test(|| {
            let buf = b"Hello World!";
            Host.set_func_buf(buf);

            let buf = Host.get_calldata();
            assert_eq!(buf, b"Hello World!");
        });
    }

    #[test]
    fn host_accounts_balance() {
        test(|| {
            let addr1 = AddressOwned([0x10; 20]);
            let addr2 = AddressOwned([0x20; 20]);

            Host.set_balance(addr1, Amount(10));
            Host.set_balance(addr2, Amount(20));

            let addr1 = Address(&[0x10; 20]);
            let addr2 = Address(&[0x20; 20]);

            let amount1 = Host.get_balance(&addr1);
            let amount2 = Host.get_balance(&addr2);
            assert_eq!(amount1, Amount(10));
            assert_eq!(amount2, Amount(20));
        });
    }

    #[test]
    fn host_transfer() {
        test(|| {
            let src = AddressOwned([0x10; 20]);
            let dst = AddressOwned([0x20; 20]);

            Host.set_sender(src.clone());

            Host.set_balance(src, Amount(10));
            Host.set_balance(dst, Amount(20));

            let src = Address(&[0x10; 20]);
            let dst = Address(&[0x20; 20]);

            let amount1 = Host.get_balance(&src);
            let amount2 = Host.get_balance(&dst);
            assert_eq!(amount1, Amount(10));
            assert_eq!(amount2, Amount(20));

            Host.transfer(&dst, Amount(5));

            let amount1 = Host.get_balance(&src);
            let amount2 = Host.get_balance(&dst);
            assert_eq!(amount1, Amount(10 - 5));
            assert_eq!(amount2, Amount(20 + 5));
        });
    }

    #[test]
    fn host_layer() {
        test(|| {
            Host.set_layer(LayerId(10));

            let layer = Host.now();
            assert_eq!(layer, LayerId(10));
        });
    }

    #[test]
    fn host_logs() {
        test(|| {
            let logs = Host.get_logs();
            assert!(logs.is_empty());

            Host.log("Log #1", 100);
            Host.log("Log #2", 200);

            let logs = Host.get_logs();
            assert_eq!(
                logs,
                vec![("Log #1".to_string(), 100), ("Log #2".to_string(), 200)]
            )
        });
    }
}
