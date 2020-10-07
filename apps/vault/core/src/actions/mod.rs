use svm_abi_decoder::CallData;
use svm_sdk::{value::AddressOwned, Host};

mod auth;
mod daily;
mod initialize;

// mod getters;
// mod withdraw;

pub use initialize::initialize;

// pub use getters::*;
// pub use withdraw::{daily_withdraw, withdraw};

enum ReqKind {
    Withdraw,

    SetSpendingLimit,

    SetSpendingAccount,
}

fn action_begin<F>(action: F)
where
    F: Fn(&mut CallData, AddressOwned),
{
    let bytes = Host::get_calldata();
    let mut calldata = CallData::new(bytes);

    let master = auth::auth_multisig_begin(&mut calldata);

    action(&mut calldata, master);
}

fn action_complete<F>(action: F, req_kind: ReqKind)
where
    F: Fn(&mut CallData),
{
    let bytes = Host::get_calldata();
    let mut calldata = CallData::new(bytes);

    let master = auth::auth_multisig_begin(&mut calldata);
    //
}
