use crate::{Address, Amount, LayerId};

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub trait Host {
    fn get_calldata(&self) -> &'static [u8];

    fn sender(&self) -> Address;

    fn balance(&self) -> Amount;

    fn layer(&self) -> LayerId;

    fn get_balance(&self, addr: &Address) -> Amount;

    fn transfer(&self, dst: &Address, amount: Amount);

    fn get_logs(&self) -> Vec<(String, u8)>;

    fn log(&self, msg: &str, code: u8);
}
