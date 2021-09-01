use std::convert::TryInto;

use svm_layout::{FixedLayout, Id};
use svm_types::{Account, Address, BytesPrimitive};

use crate::GlobalState;

pub struct AccountStorage<'a> {
    gs: GlobalState,
    account: Account,
    address: Address,
    layout: &'a FixedLayout,
}

impl<'a> AccountStorage<'a> {
    pub fn new(
        gs: GlobalState,
        account: &Account,
        address: &Address,
        layout: &'a FixedLayout,
    ) -> Self {
        Self {
            gs,
            account: account.clone(),
            address: address.clone(),
            layout,
        }
    }

    async fn var(&self, var_id: u32) -> [u8; 32] {
        let key = keys::account_var(&self.address, var_id, &self.layout);
        self.gs
            .storage
            .get(key.as_bytes(), None)
            .await
            .unwrap()
            .unwrap_or(vec![0; 32])
            .try_into()
            .expect("Unexpected length of value, expected 32 bytes.")
    }

    pub async fn var_i64(&self, var_id: u32) -> i64 {
        let offset = self.layout.get(Id(var_id)).offset() as usize % 32;
        let byte_size = self.layout.get(Id(var_id)).byte_size() as usize;

        let slice = &self.var(var_id).await[offset..offset + byte_size];
        i64::from_be_bytes(slice.try_into().unwrap())
    }

    pub async fn var_160(&self, var_id: u32) -> [u8; 20] {
        let offset = self.layout.get(Id(var_id)).offset() as usize % 32;
        let byte_size = self.layout.get(Id(var_id)).byte_size() as usize;

        self.var(var_id).await[offset..offset + byte_size]
            .try_into()
            .unwrap()
    }

    pub async fn set_var(&mut self, var_id: u32, new_value: &[u8]) {
        let mut value = self.var(var_id).await;
        let key = keys::account_var(&self.address, var_id, &self.layout);
        let offset = self.layout.get(Id(var_id)).offset() as usize % 32;

        let slice = &mut value[offset..offset + new_value.len()];
        slice.copy_from_slice(new_value);

        self.gs.storage.upsert(key.as_bytes(), value).await;
    }

    pub async fn set_var_i64(&mut self, var_id: u32, value: i64) {
        let byte_size = self.layout.get(Id(var_id)).byte_size() as usize;
        self.set_var(var_id, &value.to_be_bytes()[..byte_size])
            .await;
    }

    pub async fn set_var_160(&mut self, var_id: u32, value: [u8; 20]) {
        debug_assert_eq!(self.layout.get(Id(var_id)).byte_size(), 20);

        self.set_var(var_id, &value).await;
    }
}

mod keys {
    use super::*;

    pub fn account_var(account_addr: &Address, var_id: u32, layout: &FixedLayout) -> String {
        let offset = layout.get(Id(var_id)).offset();
        let key_index = offset % 32;

        format!("accounts:{}:vars:{}", account_addr.to_string(), key_index)
    }
}
