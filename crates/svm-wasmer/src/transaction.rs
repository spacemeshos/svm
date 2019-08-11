use svm_common::{Address, Balance, ContractState};

/// `Transaction` aggregates all the data necessary for running a constract transaction.
/// Its data will be consumed by the running contracts and the hosting vm itself (a.ka. `svm`).
pub struct Transaction {
    addr: Address,
    state: ContractState,
    balance: Balance,
    sender_addr: Address,
    sender_balance: Balance,
    gas_left: u64,
    payload: Vec<u8>,
    func_name: String,
}

impl Transaction {
    /// Creates a new instance of `Transaction` from raw inputs.
    pub fn from_raw(
        addr: *const u8,
        state: *const u8,
        balance: *const u8,
        sender_addr: *const u8,
        sender_balance: *const u8,
        gas_left: u64,
        payload: *const u8,
        payload_len: u32,
        func_name: *const u8,
        func_name_len: u32,
    ) -> Self {
        let addr = Address::from(addr);
        let state = ContractState::from(state);
        let balance = Balance::from(balance);
        let sender_addr = Address::from(sender_addr);
        let sender_balance = Balance::from(sender_balance);

        let payload = unsafe {
            let bytes = std::slice::from_raw_parts(payload, payload_len as usize);
            bytes.to_vec()
        };

        let func_name = unsafe {
            let bytes = std::slice::from_raw_parts(func_name, func_name_len as usize);
            String::from_utf8(bytes.to_vec()).unwrap()
        };

        Self {
            addr,
            state,
            balance,
            sender_addr,
            sender_balance,
            gas_left,
            payload,
            func_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_raw() {
        let addr = Address::from(0x10_20_30_40);
        let state = ContractState::from(0xAB_CD_EF);
        let balance = Balance(100);
        let sender_addr = Address::from(0x50_60_70_80);
        let sender_balance = Balance(200);
        let gas_left = 100;
        let payload = vec![10, 20, 30, 40, 50];
        let func_name = String::from("execute");

        let tx = Transaction::from_raw(
            addr.as_ptr(),
            state.as_ptr(),
            balance.0.to_le_bytes().as_ptr(),
            sender_addr.as_ptr(),
            sender_balance.0.to_le_bytes().as_ptr(),
            gas_left,
            payload.as_ptr(),
            payload.len() as u32,
            func_name.as_ptr(),
            func_name.len() as u32,
        );

        assert_eq!(Address::from(0x10_20_30_40), tx.addr);
        assert_eq!(ContractState::from(0xAB_CD_EF), tx.state);
        assert_eq!(Balance(100), tx.balance);
        assert_eq!(Address::from(0x50_60_70_80), tx.sender_addr);
        assert_eq!(Balance(200), tx.sender_balance);
        assert_eq!(100, tx.gas_left);
        assert_eq!(vec![10, 20, 30, 40, 50], tx.payload);
        assert_eq!(String::from("execute"), tx.func_name);
    }
}
