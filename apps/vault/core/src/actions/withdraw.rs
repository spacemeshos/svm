use crate::{Action, ActionKind, VaultData, VaultType};

use svm_abi_decoder::{Cursor, Decoder};
use svm_sdk::{ensure, value::Address, Amount, Host, LayerId};

#[derive(PartialEq)]
struct Withdraw<'a> {
    pub amount: Amount,

    pub master_account: Address<'a>,

    pub receiver: Address<'a>,

    pub when: LayerId,
}

impl Action for Withdraw<'_> {
    fn master_account(&self) -> &Address {
        &self.master_account
    }
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

macro_rules! decode_action {
    ($action:ident, $amount:expr, $decoder:ident, $cursor:ident) => {
        let bytes = Host::get_calldata();
        let mut $cursor = Cursor::new(bytes);
        let $decoder = Decoder::new();

        let master_account: Address<'static> = decode_addr!($decoder, $cursor);
        let receiver: Address<'static> = decode_addr!($decoder, $cursor);
        let when = Host::now();

        let $action = Withdraw {
            amount: $amount,
            master_account,
            receiver,
            when,
        };
    };
}

pub fn daily_withdraw(amount: Amount) {
    decode_action!(action, amount, decoder, cursor);

    let spending_account = VaultData::load_spending_account();
    let balance = Host::balance();

    assert_eq!(action.master_account, spending_account);
    assert!(amount.0 <= balance.0);

    let daily_limit = VaultData::load_spending_limit();
    let spent_today = VaultData::load_spent_today();
    let last_spent = VaultData::load_last_spent();

    let spent_today = refresh_spent_today(spent_today, last_spent, action.when);
    let new_spent_today = Amount(spent_today.0 + amount.0);

    ensure!(
        new_spent_today.0 <= daily_limit.0,
        "Exceeded the daily limit"
    );

    Host::transfer(&action.receiver, amount);

    VaultData::store_last_spent(action.when);
    VaultData::store_spent_today(new_spent_today);
}

pub fn withdraw(ty: VaultType, amount: Amount) {
    match ty {
        VaultType::Simple => simple_withdraw(amount),
        VaultType::MultiSig => multisig_withdraw(amount),
    }
}

fn simple_withdraw(amount: Amount) {
    decode_action!(action, amount, decoder, cursor);

    let vault_master = VaultData::load_master_account(1);
    let balance = Host::balance();

    assert_eq!(action.master_account, vault_master);
    assert!(amount.0 <= balance.0);

    Host::transfer(&action.receiver, amount);
}

fn multisig_withdraw(amount: Amount) {
    decode_action!(action, amount, decoder, cursor);

    match multisig_load_pending_action() {
        Some(pending) => try_multisig_withdraw(pending, action),
        None => multisig_queue_withdraw(action),
    }
}

fn try_multisig_withdraw(pending: Withdraw, action: Withdraw) {
    let same_master = action.master_account != pending.master_account;
    let same_amount = pending.amount == action.amount;

    if same_master {
        multisig_queue_withdraw(action);
        return;
    }
    ensure!(same_amount, "Different withdraw `amount` given.");

    let same_receiver = action.receiver == pending.receiver;
    ensure!(same_receiver, "Different withdraw `receiver` given.");

    reset_pending();

    Host::transfer(&action.receiver, action.amount);
}

pub fn reset_pending() {
    // TODO: have `Address::zeros`
    let zero_addr = Address(&[0; 20]);

    VaultData::store_pending_withdraw_layer(LayerId(0));
    VaultData::store_pending_withdraw_amount(Amount(0));
    VaultData::store_pending_withdraw_master(&zero_addr);
    VaultData::store_pending_withdraw_receiver(&zero_addr);
}

fn multisig_queue_withdraw(action: Withdraw) {
    VaultData::store_pending_withdraw_layer(action.when);
    VaultData::store_pending_withdraw_amount(action.amount);
    VaultData::store_pending_withdraw_master(&action.master_account);
    VaultData::store_pending_withdraw_receiver(&action.receiver);
}

fn multisig_load_pending_action() -> Option<Withdraw<'static>> {
    let now = Host::now();
    let layer = VaultData::load_pending_withdraw_layer();
    let expires_at = LayerId(0); // TODO

    if has_expired(expires_at, now) {
        reset_pending();
        return None;
    }

    let amount = VaultData::load_pending_withdraw_amount();
    let master_account = VaultData::load_pending_withdraw_master();
    let receiver = VaultData::load_pending_withdraw_receiver();

    let action = Withdraw {
        amount,
        master_account,
        receiver,
        when: now,
    };

    Some(action)
}

#[inline]
fn has_expired(expires_at: LayerId, now: LayerId) -> bool {
    now.0 > expires_at.0
}

fn refresh_spent_today(spent_today: Amount, last_spent: LayerId, now: LayerId) -> Amount {
    todo!()
    // if Host::layer_day(now) > Host::layer_day(last_spent) {
    //     Amount(0)
    // } else {
    //     spent_today
    // }
}
