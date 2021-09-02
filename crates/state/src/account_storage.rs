use std::convert::TryInto;

use svm_layout::{FixedLayout, Id};
use svm_types::{Address, BytesPrimitive, TemplateAddr};

use crate::account_data::{AccountData, AccountMut};
use crate::template_storage::TemplateStorage;
use crate::{GlobalState, StorageResult};

pub struct AccountStorage {
    gs: GlobalState,
    address: Address,
    template_addr: TemplateAddr,
    layout: FixedLayout,
}

impl AccountStorage {
    pub fn new(
        gs: GlobalState,
        address: &Address,
        template_addr: &TemplateAddr,
        layout: &FixedLayout,
    ) -> Self {
        Self {
            gs,
            address: address.clone(),
            template_addr: template_addr.clone(),
            layout: layout.clone(),
        }
    }

    pub fn create(
        &mut self,
        name: String,
        template_addr: TemplateAddr,
        balance: u64,
        counter: u64,
    ) {
        self.gs.encode_and_write(
            &AccountData {
                name,
                template_addr,
            },
            &AccountData::key(&self.address),
        );

        self.gs.encode_and_write(
            &AccountMut { balance, counter },
            &AccountMut::key(&self.address),
        );
    }

    pub fn gs(&self) -> &GlobalState {
        &self.gs
    }

    pub fn gs_mut(&mut self) -> &mut GlobalState {
        &mut self.gs
    }

    fn var(&self, var_id: u32) -> [u8; 32] {
        let key = keys::account_var(&self.address, var_id, &self.layout);
        self.gs
            .block_on(self.gs.storage().get(key.as_bytes(), None))
            .unwrap()
            .unwrap_or(vec![0; 32])
            .try_into()
            .expect("Unexpected length of value, expected 32 bytes.")
    }

    pub fn var_i64(&self, var_id: u32) -> i64 {
        let offset = self.layout.get(Id(var_id)).offset() as usize % 32;
        let byte_size = self.layout.get(Id(var_id)).byte_size() as usize;

        let slice = &self.var(var_id)[offset..offset + byte_size];
        i64::from_be_bytes(slice.try_into().unwrap())
    }

    pub fn var_160(&self, var_id: u32) -> [u8; 20] {
        let offset = self.layout.get(Id(var_id)).offset() as usize % 32;
        let byte_size = self.layout.get(Id(var_id)).byte_size() as usize;

        self.var(var_id)[offset..offset + byte_size]
            .try_into()
            .unwrap()
    }

    pub fn set_var(&mut self, var_id: u32, new_value: &[u8]) {
        let mut value = self.var(var_id);
        let key = keys::account_var(&self.address, var_id, &self.layout);
        let offset = self.layout.get(Id(var_id)).offset() as usize % 32;

        let slice = &mut value[offset..offset + new_value.len()];
        slice.copy_from_slice(new_value);

        self.gs.storage().upsert(key.as_bytes(), value);
    }

    pub fn get_var(&mut self, var_id: u32) -> Vec<u8> {
        let offset = self.layout.get(Id(var_id)).offset() as usize % 32;
        let byte_size = self.layout.get(Id(var_id)).byte_size() as usize;

        self.var(var_id)[offset..offset + byte_size]
            .iter()
            .copied()
            .collect()
    }

    pub fn template_storage(&self) -> TemplateStorage {
        TemplateStorage::new(&self.template_addr, self.gs.clone())
    }

    pub fn set_var_i64(&mut self, var_id: u32, value: i64) {
        let byte_size = self.layout.get(Id(var_id)).byte_size() as usize;
        self.set_var(var_id, &value.to_be_bytes()[..byte_size]);
    }

    pub fn set_var_160(&mut self, var_id: u32, value: [u8; 20]) {
        debug_assert_eq!(self.layout.get(Id(var_id)).byte_size(), 20);

        self.set_var(var_id, &value);
    }

    pub fn name(&self, account_addr: &Address) -> StorageResult<Option<String>> {
        self.gs
            .read_and_decode::<AccountData>(&AccountData::key(account_addr))
            .map(|res| res.map(|data| data.name))
    }

    pub fn template_addr(&self, account_addr: &Address) -> StorageResult<Option<TemplateAddr>> {
        self.gs
            .read_and_decode::<AccountData>(&AccountData::key(account_addr))
            .map(|res| res.map(|data| data.template_addr))
    }

    /// Reads and returns the balance of `account_addr`.
    pub fn balance(&self, account_addr: &Address) -> StorageResult<Option<u64>> {
        self.gs
            .read_and_decode::<AccountMut>(&AccountMut::key(account_addr))
            .map(|res| res.map(|data| data.balance))
    }

    /// Reads and returns the nonce counter of `account_addr`.
    pub fn counter(&self, account_addr: &Address) -> StorageResult<Option<u64>> {
        self.gs
            .read_and_decode::<AccountMut>(&AccountMut::key(account_addr))
            .map(|res| res.map(|data| data.counter))
    }

    pub fn set_balance(&mut self, account_addr: &Address, balance: u64) -> StorageResult<()> {
        self.gs
            .replace(&AccountMut::key(account_addr), |mut data: AccountMut| {
                data.balance = balance;
                data
            })
    }

    pub fn set_counter(&mut self, account_addr: &Address, counter: u64) -> StorageResult<()> {
        self.gs
            .replace(&AccountMut::key(account_addr), |mut data: AccountMut| {
                data.counter = counter;
                data
            })
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
