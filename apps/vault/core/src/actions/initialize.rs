use svm_abi_decoder::{Cursor, Decoder};

use svm_sdk::ensure;
use svm_sdk::{
    value::{self, Address, Array},
    Amount, Host,
};

use crate::{VaultData, VaultType};

extern crate alloc;
use alloc::vec::Vec;

struct Initialize<'a> {
    masters: Vec<Address<'a>>,

    spending_limit: Amount,

    spending_account: Address<'a>,
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

macro_rules! decode_masters {
    ($decoder:ident, $cursor:ident) => {{
        match $decoder.decode_value(&mut $cursor) {
            Ok(v) => {
                let addrs = v.into();
                addrs
            }
            Err(..) => panic!("invalid `calldata`"),
        }
    }};
}

macro_rules! decode_action {
    ($action:ident, $spending_limit:expr, $decoder:ident, $cursor:ident) => {
        let bytes = Host::get_calldata();
        let mut $cursor = Cursor::new(bytes);
        let $decoder = Decoder::new();

        let masters = decode_masters!($decoder, $cursor);
        let spending_account: Address<'static> = decode_addr!($decoder, $cursor);
        let when = Host::now();

        let $action = Initialize {
            masters,
            spending_account,
            spending_limit: $spending_limit,
        };
    };
}

pub fn initialize(vault_ty: VaultType, spending_limit: Amount) {
    let vault_ty: VaultType = vault_ty.into();
    let spending_limit: Amount = spending_limit.into();

    match vault_ty {
        VaultType::Simple => simple_initialize(spending_limit),
        VaultType::MultiSig => multisig_initialize(spending_limit),
    }
}

fn simple_initialize(spending_limit: Amount) {
    decode_action!(action, spending_limit, decoder, cursor);

    let masters = &action.masters;

    ensure!(masters.len() == 1, "Wrong number of master accounts given.");

    VaultData::store_type(VaultType::Simple);
    VaultData::store_master_account(&masters[0], 1);
    VaultData::store_spending_limit(spending_limit);
    VaultData::store_spending_account(&action.spending_account);
}

fn multisig_initialize(spending_limit: Amount) {
    decode_action!(action, spending_limit, decoder, cursor);

    let masters = &action.masters;

    ensure!(masters.len() == 3, "Wrong number of master accounts given.");

    let a = &masters[0];
    let b = &masters[1];
    let c = &masters[2];

    assert_not_same(a, b);
    assert_not_same(a, c);
    assert_not_same(b, c);

    VaultData::store_type(VaultType::MultiSig);
    VaultData::store_master_account(a, 1);
    VaultData::store_master_account(b, 2);
    VaultData::store_master_account(c, 3);
    VaultData::store_spending_limit(spending_limit);
    VaultData::store_spending_account(&action.spending_account);
}

fn assert_not_same(addr1: &Address, addr2: &Address) {
    ensure!(
        addr1 != addr2,
        "Master Accounts must be different from one another."
    );
}
