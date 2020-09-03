#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use svm_sdk::{ensure, Amount};

mod action;
mod actions;
mod amount;
mod data;
mod vault_type;

use data::VaultData;
use vault_type::VaultType;

pub use action::{Action, ActionKind};

pub fn initialize(vault_ty: VaultType, spending_limit: Amount) {
    actions::initialize(vault_ty, spending_limit);
}

pub fn withdraw(ty: VaultType, amount: Amount) {
    actions::withdraw(ty, amount)
}

pub fn daily_withdraw(amount: Amount) {
    actions::daily_withdraw(amount)
}

pub fn set_spending_account() {
    actions::set_spending_account();
}

pub fn set_spending_limit(amount: Amount) {
    actions::set_spending_limit(amount)
}

pub fn cancel_action(action: ActionKind) {
    // let vault_ty = VaultData::load_type();

    // ensure!(
    //     vault_ty == VaultType::MultiSig,
    //     "`cancel_action` is relevant only for `MultiSig` Vaults."
    // );

    // // TODO: authenticate ??

    // match action {
    //     ActionKind::Withdraw => withdraw::reset_pending(),
    //     ActionKind::SetDailyLimit => todo!(),
    //     ActionKind::SetSpendingAccount => todo!(),
    // }
}

// Getters

pub fn get_pending_withdraw() -> usize {
    let ptr = actions::get_pending_withdraw();
    ptr.0
}

pub fn get_pending_spending_limit() -> usize {
    let ptr = actions::get_pending_spending_limit();
    ptr.0
}

pub fn get_pending_spending_account() -> usize {
    let ptr = actions::get_pending_spending_account();
    ptr.0
}
