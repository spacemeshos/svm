#![allow(unused)]

use svm_sdk::{value::Address, Amount};
use svm_sdk_macros::AppStorage;

#[derive(AppStorage, Debug)]
struct Vault {
    multisig: bool,
}

fn main() {
    let v = VaultStorage {};

    let _ = v;
}

// type Addr = Address<'static>;

// #[derive(AppStorage)]
// struct VaultStorage {
//     multisig: bool,

//     master: [Addr; 3],

//     daily_spending_limit: Amount,

//     daily_spending_account: Addr,

//     withdraw_queue: [Addr; 3],

//     spending_account_queue: [Addr; 3],

//     spending_limit_queue: [Amount; 3],
// }

// impl VaultStorage {
//     pub fn set_multisig(v: bool) {
//         //
//     }

//     pub fn get_multisig() -> bool {
//         false
//     }

//     pub fn set_master(addr: &Address, index: usize) {
//         //
//     }

//     pub fn get_master(index: usize) -> Addr {
//         todo!()
//     }

//     pub fn set_daily_spending_limit(amount: Amount) {
//         //
//     }

//     pub fn get_daily_spending_limit() -> Amount {
//         todo!()
//     }

//     pub fn set_daily_spending_account(addr: &Addr) {
//         //
//     }

//     pub fn get_daily_spending_account() -> Addr {
//         todo!()
//     }

//     pub fn set_withdraw_queue(addr: &Addr, index: usize) {
//         //
//     }

//     pub fn get_withdraw_queue(index: usize) -> Addr {
//         todo!()
//     }

//     pub fn set_spending_account_queue(addr: &Addr, index: usize) {
//         //
//     }

//     pub fn get_spending_account_queue(index: usize) -> Addr {
//         todo!()
//     }

//     pub fn set_spending_limit_queue(amount: Amount, index: usize) {
//         //
//     }

//     pub fn get_spending_limit_queue(index: usize) -> Amount {
//         todo!()
//     }
// }
