use crate::{VaultData, VaultType};

use svm_abi_decoder::CallData;

use svm_sdk::ensure;
use svm_sdk::value::Address;

pub(crate) fn auth_simple(calldata: &mut CallData) {
    assert_simple_vault();

    let input_master: Address = calldata.next_1();
    let storage_master = VaultData::load_master_account(1);

    ensure!(
        input_master == storage_master,
        "Invalid input master account"
    );
}

pub(crate) fn auth_multisig_begin(calldata: &mut CallData) -> Address {
    assert_multisig_vault();

    let mut good_input = false;
    let input_master = calldata.next_1();

    for i in 1..=3 {
        let storage_master = VaultData::load_master_account(i);

        if input_master == storage_master {
            good_input = true;
            break;
        }
    }

    ensure!(good_input, "Invalid Master given");

    input_master
}

pub(crate) fn auth_multisig_complete(calldata: &mut CallData, begin_addr: &Address) {
    todo!();

    // assert_multisig_vault();

    // let mut good_input = false;
    // let input_master: Address = calldata.next_1();

    // for i in 1..=3 {
    //     let storage_master = VaultData::load_master_account(i);

    //     if input_master == storage_master {
    //         good_input = true;
    //         break;
    //     }
    // }

    // ensure!(good_input, "Invalid Master given");
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
