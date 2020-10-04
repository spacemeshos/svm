use svm_abi_decoder::CallData;

use svm_sdk::ensure;
use svm_sdk::{
    value::{self, Address, AddressOwned, Value},
    Amount, Host,
};

use crate::{VaultData, VaultType};

extern crate alloc;
use alloc::vec::Vec;

pub fn initialize() {
    let bytes = Host::get_calldata();
    let mut calldata = CallData::new(bytes);

    let (ty, spending_limit, spending_account): (bool, _, _) = calldata.next_3();

    VaultData::store_spending_limit(spending_limit);
    VaultData::store_spending_account(&spending_account);

    match ty.into() {
        VaultType::Simple => simple_initialize(calldata),
        VaultType::MultiSig => multisig_initialize(calldata),
    }
}

fn simple_initialize(mut calldata: CallData) {
    let masters: [AddressOwned; 1] = calldata.next_1();

    VaultData::store_type(VaultType::Simple);
    // VaultData::store_master_account(&masters[0], 1);
}

fn multisig_initialize(mut calldata: CallData) {
    let masters: [AddressOwned; 3] = calldata.next_1();

    let (a, b, c) = (&masters[0], &masters[1], &masters[2]);

    assert_not_same(a, b);
    assert_not_same(a, c);
    assert_not_same(b, c);

    VaultData::store_type(VaultType::MultiSig);
    // VaultData::store_master_account(a, 1);
    // VaultData::store_master_account(b, 2);
    // VaultData::store_master_account(c, 3);
}

fn assert_not_same(addr1: &AddressOwned, addr2: &AddressOwned) {
    ensure!(
        addr1 != addr2,
        "Master Keys must be different from one another."
    );
}
