use tokio::runtime::Runtime;

use std::sync::{Arc, Mutex, MutexGuard};

use svm_codec::Codec;
use svm_hash::{Blake3Hasher, Hasher};
use svm_layout::{FixedLayout, Id};
use svm_types::{Address, BytesPrimitive, Layer, Sections, TemplateAddr};

use crate::account_data::{AccountData, AccountMut};
use crate::storage::{Fingerprint, Storage};
use crate::{StorageError, StorageResult as Result};

/// A key-value store with a non-falsifiable state signature, historical data
/// querying and other features which make it suitable for storing Spacemesh'
/// global state.
///
/// This data structure is backed by SQLite.
#[derive(Debug, Clone)]
pub struct GlobalState {
    pub(crate) storage: Arc<Mutex<Storage>>,
    runtime: Arc<Mutex<Runtime>>,
}

impl GlobalState {
    /// Recovers a [`GlobalState`] from a SQLite instance at `sqlite_uri`.
    ///
    /// # Warning
    ///
    /// This method assumes that the given SQLite instance is in a "good" state;
    /// "good" means that only SVM has ever accessed and modified its contents.
    pub fn new(sqlite_uri: &str) -> Self {
        let runtime = Runtime::new().unwrap();
        let storage = runtime.block_on(Storage::new(sqlite_uri)).unwrap();
        Self {
            storage: Arc::new(Mutex::new(storage)),
            runtime: Arc::new(Mutex::new(runtime)),
        }
    }

    /// Creates a pristine [`GlobalState`] backed by an in-memory SQLite
    /// instance. No disk operations at all will be done.
    pub fn in_memory() -> Self {
        Self::new(":memory:")
    }

    pub(crate) fn storage(&self) -> MutexGuard<Storage> {
        self.storage
            .lock()
            .expect("Poisoned lock on global state storage")
    }

    pub(crate) fn block_on<F>(&self, future: F) -> F::Output
    where
        F: std::future::Future,
    {
        self.runtime
            .lock()
            .expect("Poisoned lock on global state runtime")
            .block_on(future)
    }

    fn read_and_decode<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: Codec,
    {
        let key_hash = Blake3Hasher::hash(key.as_bytes());
        let opt_value = self.block_on(self.storage().get(&key_hash, None))?;

        if let Some(bytes) = opt_value {
            T::decode_bytes(bytes)
                .map(|data| Some(data))
                .map_err(|_| StorageError::IllegalData { key_hash })
        } else {
            Ok(None)
        }
    }

    fn encode_and_write<T>(&mut self, item: &T, key: &str) -> ()
    where
        T: Codec,
    {
        self.block_on(self.storage().upsert(key.as_bytes(), item.encode_to_vec()));
    }

    fn replace<T, F>(&mut self, key: &str, f: F) -> Result<()>
    where
        T: Codec,
        F: Fn(T) -> T,
    {
        let key_hash = Blake3Hasher::hash(key.as_bytes());

        let item = self
            .read_and_decode::<T>(key)
            .and_then(|opt| opt.ok_or(StorageError::NotFound { key_hash }))?;

        self.encode_and_write(&f(item), key);
        Ok(())
    }

    // ACCOUNT
    // -------

    pub fn set_account(
        &mut self,
        account_addr: &Address,
        name: String,
        template_addr: TemplateAddr,
        balance: u64,
        counter: u64,
    ) {
        self.encode_and_write(
            &AccountData {
                name,
                template_addr,
            },
            &AccountData::key(account_addr),
        );

        self.encode_and_write(
            &AccountMut { balance, counter },
            &AccountMut::key(account_addr),
        );
    }

    pub fn account_name(&self, account_addr: &Address) -> Result<Option<String>> {
        self.read_and_decode::<AccountData>(&AccountData::key(account_addr))
            .map(|res| res.map(|data| data.name))
    }

    pub fn account_template_addr(&self, account_addr: &Address) -> Result<Option<TemplateAddr>> {
        self.read_and_decode::<AccountData>(&AccountData::key(account_addr))
            .map(|res| res.map(|data| data.template_addr))
    }

    /// Reads and returns the balance of `account_addr`.
    pub fn account_balance(&self, account_addr: &Address) -> Result<Option<u64>> {
        self.read_and_decode::<AccountMut>(&AccountMut::key(account_addr))
            .map(|res| res.map(|data| data.balance))
    }

    /// Reads and returns the nonce counter of `account_addr`.
    pub fn account_counter(&self, account_addr: &Address) -> Result<Option<u64>> {
        self.read_and_decode::<AccountMut>(&AccountMut::key(account_addr))
            .map(|res| res.map(|data| data.counter))
    }

    pub fn set_account_balance(&mut self, account_addr: &Address, balance: u64) -> Result<()> {
        self.replace(&AccountMut::key(account_addr), |mut data: AccountMut| {
            data.balance = balance;
            data
        })
    }

    pub fn set_account_counter(&mut self, account_addr: &Address, counter: u64) -> Result<()> {
        self.replace(&AccountMut::key(account_addr), |mut data: AccountMut| {
            data.counter = counter;
            data
        })
    }

    // TEMPLATE SECTIONS
    // -----------------

    pub fn template_sections(&self, template_addr: &TemplateAddr) -> Result<Option<Sections>> {
        let core_sections_opt: Option<Sections> =
            self.read_and_decode(&keys::template_core(template_addr))?;
        let noncore_sections_opt: Option<Sections> =
            self.read_and_decode(&keys::template_noncore(template_addr))?;

        match (core_sections_opt, noncore_sections_opt) {
            (Some(mut sections), Some(noncore)) => {
                for s in noncore.iter().cloned() {
                    sections.insert(s);
                }
                Ok(Some(sections))
            }
            _ => return Ok(None),
        }
    }

    pub fn set_template_core_sections(
        &mut self,
        template_addr: &TemplateAddr,
        sections: &Sections,
    ) -> Result<()> {
        self.encode_and_write(sections, &keys::template_core(template_addr));
        Ok(())
    }

    pub fn set_template_noncore_sections(
        &mut self,
        template_addr: &TemplateAddr,
        sections: &Sections,
    ) -> Result<()> {
        self.encode_and_write(sections, &keys::template_noncore(template_addr));
        Ok(())
    }

    // VERSIONING
    // ----------

    pub fn checkpoint(&mut self) -> Result<()> {
        self.block_on(self.storage().checkpoint())?;
        Ok(())
    }

    pub fn commit(&mut self) -> Result<(Layer, Fingerprint)> {
        Ok(self.block_on(self.storage().commit())?)
    }

    pub fn current_layer(&mut self) -> Result<(Layer, Fingerprint)> {
        Ok(self.block_on(self.storage().last_layer())?)
    }

    pub fn rollback(&mut self) -> Result<()> {
        self.block_on(self.storage().rollback())?;
        Ok(())
    }

    pub fn rewind(&mut self, layer_id: Layer) -> Result<()> {
        self.block_on(self.storage().rewind(layer_id))?;
        Ok(())
    }
}

mod keys {
    use super::*;

    pub fn account_var(account_addr: &Address, var_id: u32, layout: &FixedLayout) -> String {
        let offset = layout.get(Id(var_id)).offset();
        let key_index = offset % 32;

        format!("accounts:{}:vars:{}", account_addr.to_string(), key_index)
    }

    pub fn template_core(template_addr: &TemplateAddr) -> String {
        format!("templates:{}:core", template_addr.to_string())
    }

    pub fn template_noncore(template_addr: &TemplateAddr) -> String {
        format!("templates:{}:noncore", template_addr.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_account_then_get() {
        let mut gs = GlobalState::in_memory();
        let account_addr = Address::zeros();

        gs.set_account(
            &account_addr,
            "@foobar".to_string(),
            TemplateAddr::zeros(),
            42,
            1337,
        );

        assert_eq!(gs.account_name(&account_addr).unwrap().unwrap(), "@foobar");
        assert_eq!(
            gs.account_template_addr(&account_addr).unwrap().unwrap(),
            TemplateAddr::zeros()
        );
        assert_eq!(gs.account_balance(&account_addr).unwrap().unwrap(), 42);
        assert_eq!(gs.account_counter(&account_addr).unwrap().unwrap(), 1337);
    }
}
