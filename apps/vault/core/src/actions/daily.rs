use crate::{VaultData, VaultType};

use svm_abi_decoder::{Cursor, Decoder};
use svm_sdk::{ensure, value::Address, Amount, Host, LayerId};

struct SpendingAccount<'a> {
    pub master_account: Address<'a>,

    pub spending_account: Address<'a>,

    pub when: LayerId,
}

struct SpendingLimit<'a> {
    pub limit: Amount,

    pub master_account: Address<'a>,

    pub when: LayerId,
}

macro_rules! decode_addr {
    ($decoder:ident, $cursor:ident) => {{
        match $decoder.decode_value(&mut $cursor) {
            Ok(v) => {
                let addr: Address<'static> = v.into();
                addr
            }
            Err(..) => panic!("invalid `calldata`"),
        }
    }};
}

macro_rules! decode_spending_account {
    ($action:ident, $spending_account:expr, $decoder:ident, $cursor:ident) => {
        let bytes = Host::get_calldata();
        let mut $cursor = Cursor::new(bytes);
        let $decoder = Decoder::new();

        let master_account: Address<'static> = decode_addr!($decoder, $cursor);
        let spending_account: Address<'static> = decode_addr!($decoder, $cursor);
        let when = Host::now();

        let $action = SpendingAccount {
            master_account,
            spending_account,
            when,
        };
    };
}

macro_rules! decode_spending_limit {
    ($action:ident, $limit:expr, $decoder:ident, $cursor:ident) => {
        let bytes = Host::get_calldata();
        let mut $cursor = Cursor::new(bytes);
        let $decoder = Decoder::new();

        let master_account = decode_addr!($decoder, $cursor);
        let when = Host::now();

        let $action = SpendingLimit {
            limit: $limit,
            master_account,
            when,
        };
    };
}

pub fn set_spending_account() {
    decode_spending_account!(action, account, decoder, cursor);

    let ty = VaultData::load_type();

    match ty {
        VaultType::Simple => simple_set_spending_account(action),
        VaultType::MultiSig => multisig_set_spending_account(action),
    }
}

pub fn set_spending_limit(amount: Amount) {
    decode_spending_limit!(action, amount, decoder, cursor);

    let ty = VaultData::load_type();

    match ty {
        VaultType::Simple => simple_set_spending_limit(action),
        VaultType::MultiSig => multisig_set_spending_limit(action),
    }
}

fn simple_set_spending_account(action: SpendingAccount) {
    let master_account = VaultData::load_master_account(1);

    ensure!(
        master_account == action.master_account,
        "Invalid input master account"
    );

    VaultData::store_spending_account(&action.spending_account);
}

fn multisig_set_spending_account(action: SpendingAccount) {
    //
}

fn simple_set_spending_limit(action: SpendingLimit) {
    //
}

fn multisig_set_spending_limit(action: SpendingLimit) {
    //
}
