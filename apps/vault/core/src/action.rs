use svm_sdk::{Amount, LayerId};

use svm_sdk::value::{Address, AddressOwned};

pub trait Action {
    fn master_account(&self) -> &Address;
}

pub enum ActionKind {
    Withdraw,

    SetDailyLimit,

    SetSpendingAccount,
}

impl From<i32> for ActionKind {
    fn from(v: i32) -> Self {
        match v {
            0 => ActionKind::Withdraw,
            1 => ActionKind::SetDailyLimit,
            2 => ActionKind::SetSpendingAccount,
            _ => unreachable!(),
        }
    }
}

impl From<ActionKind> for i32 {
    fn from(kind: ActionKind) -> i32 {
        match kind {
            ActionKind::Withdraw => 0,
            ActionKind::SetDailyLimit => 1,
            ActionKind::SetSpendingAccount => 2,
        }
    }
}
