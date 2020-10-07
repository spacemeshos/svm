use super::auth;
use crate::{VaultData, VaultType};

use svm_abi_decoder::CallData;
use svm_sdk::{ensure, value::Address, Amount, Host, LayerId};

pub fn simple_set_spending_account() {
    let bytes = Host::get_calldata();
    let mut calldata = CallData::new(bytes);

    auth::auth_simple(&mut calldata);

    let account: Address = calldata.next_1();
    VaultData::store_spending_account(&account);
}

pub fn multisig_set_spending_account_begin() {
    let bytes = Host::get_calldata();
    let mut calldata = CallData::new(bytes);

    let input_master = auth::auth_multisig_begin(&mut calldata);

    // TODO: save pending master in the `set_spending_account` context.
}

pub fn multisig_set_spending_account_complete() {
    let bytes = Host::get_calldata();
    let mut calldata = CallData::new(bytes);

    // TODO: load master_begin from storage
    let master_begin = Address(&[0; 20]);
    auth::auth_multisig_complete(&mut calldata, master_begin);
}
