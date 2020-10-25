use crate::{Address, Amount, LayerId};

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub trait Host {
    fn get_calldata(&self) -> &'static [u8];

    fn set_returndata(&self, bytes: &[u8]);

    fn sender(&self) -> Address;

    fn app(&self) -> Address;

    fn layer_id(&self) -> LayerId;

    fn balance_of(&self, addr: &Address) -> Amount;

    fn transfer(&self, dst: &Address, amount: Amount);

    fn log(&self, msg: &str, code: u8);

    #[inline]
    fn sender_balance(&self) -> Amount {
        let sender = self.sender();

        self.balance_of(&sender)
    }

    #[inline]
    fn app_balance(&self) -> Amount {
        let addr = self.app();

        self.balance_of(&addr)
    }
}
