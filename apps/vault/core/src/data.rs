use crate::VaultType;

use svm_sdk::{memory, value::Address, Amount, LayerId, Storage};

pub struct VaultData;

impl VaultData {
    // Vault type
    /////////////////////////////////////////////////////////////
    pub fn store_type(ty: VaultType) {
        let var_id = VarId::VAULT_TYPE;
        let ty: u32 = ty.into();

        Storage::set32(var_id, ty)
    }

    pub fn load_type() -> VaultType {
        let var_id = VarId::VAULT_TYPE;
        let ty = Storage::get32(var_id);

        ty.into()
    }

    // Master Account
    /////////////////////////////////////////////////////////////
    pub fn store_master_account(account: &Address, index: usize) {
        let var_id = master_account_var(index);

        store_account(account, var_id)
    }

    pub fn load_master_account(index: usize) -> Address<'static> {
        let var_id = master_account_var(index);

        load_account(var_id)
    }

    // Pending Withdraw Master
    /////////////////////////////////////////////////////////////
    pub fn store_pending_withdraw_master(account: &Address) {
        let var_id = VarId::PENDING_WITHDRAW_MASTER;

        store_account(account, var_id)
    }

    pub fn load_pending_withdraw_master() -> Address<'static> {
        let var_id = VarId::PENDING_WITHDRAW_MASTER;

        load_account(var_id)
    }

    // Pending Withdraw Receiver
    /////////////////////////////////////////////////////////////
    pub fn store_pending_withdraw_receiver(account: &Address) {
        let var_id = VarId::PENDING_WITHDRAW_RECEIVER;

        store_account(account, var_id)
    }

    pub fn load_pending_withdraw_receiver() -> Address<'static> {
        let var_id = VarId::PENDING_WITHDRAW_RECEIVER;

        load_account(var_id)
    }

    // Pending Withdraw Amount
    /////////////////////////////////////////////////////////////
    pub fn store_pending_withdraw_amount(amount: Amount) {
        let var_id = VarId::PENDING_WITHDRAW_AMOUNT;

        Storage::set64(var_id, amount.0)
    }

    pub fn load_pending_withdraw_amount() -> Amount {
        let var_id = VarId::PENDING_WITHDRAW_AMOUNT;
        let amount = Storage::get64(var_id);

        Amount(amount)
    }

    // Pending Withdraw Layer
    /////////////////////////////////////////////////////////////
    pub fn store_pending_withdraw_layer(layer: LayerId) {
        let var_id = VarId::PENDING_WITHDRAW_LAYER;

        Storage::set64(var_id, layer.0)
    }

    pub fn load_pending_withdraw_layer() -> LayerId {
        let var_id = VarId::PENDING_WITHDRAW_LAYER;
        let layer = Storage::get64(var_id);

        LayerId(layer as _)
    }

    // Pending Spending Account
    /////////////////////////////////////////////////////////////
    pub fn store_spending_account(account: &Address) {
        let var_id = VarId::SPENDING_ACCOUNT;

        store_account(account, var_id)
    }

    pub fn load_spending_account() -> Address<'static> {
        let var_id = VarId::SPENDING_ACCOUNT;

        load_account(var_id)
    }

    // Pending Spending Limit
    /////////////////////////////////////////////////////////////
    pub fn store_spending_limit(limit: Amount) {
        let var_id = VarId::SPENDING_LIMIT;

        Storage::set64(var_id, limit.0)
    }

    pub fn load_spending_limit() -> Amount {
        let var_id = VarId::SPENDING_LIMIT;
        let amount = Storage::get64(var_id);

        Amount(amount as _)
    }

    // Spent Today
    /////////////////////////////////////////////////////////////
    pub fn load_spent_today() -> Amount {
        let var_id = VarId::SPENT_TODAY;
        let amount = Storage::get64(var_id);

        Amount(amount as _)
    }

    pub fn store_spent_today(amount: Amount) {
        let var_id = VarId::SPENT_TODAY;

        Storage::set64(var_id, amount.0);
    }

    // Last Spent
    /////////////////////////////////////////////////////////////
    pub fn load_last_spent() -> LayerId {
        let var_id = VarId::LAST_SPENT;
        let layer_id = Storage::get64(var_id);

        LayerId(layer_id as _)
    }

    pub fn store_last_spent(layer_id: LayerId) {
        let var_id = VarId::LAST_SPENT;

        Storage::set64(var_id, layer_id.0);
    }
}

fn store_account(account: &Address, var_id: u32) {
    let ptr: usize = account.as_ptr() as _;

    Storage::store160(var_id, ptr)
}

fn load_account(var_id: u32) -> Address<'static> {
    let ptr = memory::alloc(20);

    Storage::load160(var_id, ptr);

    todo!()
}

struct VarId;

impl VarId {
    const VAULT_TYPE: u32 = 0;

    const MASTER_ADDR1: u32 = 1;
    const MASTER_ADDR2: u32 = 2;
    const MASTER_ADDR3: u32 = 3;

    const SPENDING_LIMIT: u32 = 4;
    const SPENDING_ACCOUNT: u32 = 5;
    const SPENT_TODAY: u32 = 6;
    const LAST_SPENT: u32 = 7;

    const PENDING_WITHDRAW_MASTER: u32 = 8;
    const PENDING_WITHDRAW_RECEIVER: u32 = 9;
    const PENDING_WITHDRAW_AMOUNT: u32 = 10;
    const PENDING_WITHDRAW_LAYER: u32 = 11;
}

fn master_account_var(index: usize) -> u32 {
    match index {
        1 => VarId::MASTER_ADDR1,
        2 => VarId::MASTER_ADDR2,
        3 => VarId::MASTER_ADDR3,
        _ => unreachable!(),
    }
}
