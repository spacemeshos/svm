use crate::{VaultData, VaultType};

use svm_abi_decoder::CallData;

use svm_sdk::ensure;
use svm_sdk::value::{Address, AddressOwned};

pub(crate) fn auth_simple(calldata: &mut CallData) {
    assert_simple_vault();

    let master_input: Address = calldata.next_1();
    let master_db = VaultData::load_master_account(1);

    ensure!(master_input == master_db, "Invalid input Master-Key");
}

pub(crate) fn auth_multisig_begin(calldata: &mut CallData) -> AddressOwned {
    assert_multisig_vault();

    let mut good_input = false;
    let master_begin: Address = calldata.next_1();

    for i in 1..=3 {
        let master_db = VaultData::load_master_account(i);

        if master_begin == master_db {
            good_input = true;
            break;
        }
    }

    ensure!(good_input, "Invalid Master-Key given");

    master_begin.to_owned()
}

pub(crate) fn auth_multisig_complete(calldata: &mut CallData, master_begin: Address) {
    assert_multisig_vault();

    let mut good_input = false;
    let master_complete: Address = calldata.next_1();

    ensure!(
        master_complete != master_begin,
        "Master-Key is already in-use."
    );

    for i in 1..=3 {
        let master_db = VaultData::load_master_account(i);

        if master_db == master_complete {
            good_input = true;
            break;
        }
    }

    ensure!(good_input, "Invalid Master given");
}

pub(crate) fn auth_multisig_reset(ctx: &str) {
    //
}

#[inline]
pub(crate) fn assert_simple_vault() {
    let ty = VaultData::load_type();

    ensure!(
        ty == VaultType::Simple,
        "tx is relevant only for Simple Vaults"
    );
}

#[inline]
pub(crate) fn assert_multisig_vault() {
    let ty = VaultData::load_type();

    ensure!(
        ty == VaultType::MultiSig,
        "tx is relevant only for MultiSig Vaults"
    );
}
