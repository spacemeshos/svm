use tokio::runtime::Runtime;

use std::sync::{Arc, Mutex, MutexGuard};

use svm_codec::Codec;
use svm_hash::{Blake3Hasher, Hasher};
use svm_types::{Layer, State};

use crate::storage::Storage;
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

    pub(crate) fn read_and_decode<T>(&self, key: &str) -> Result<Option<T>>
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

    pub(crate) fn encode_and_write<T>(&mut self, item: &T, key: &str) -> ()
    where
        T: Codec,
    {
        self.block_on(self.storage().upsert(key.as_bytes(), item.encode_to_vec()));
    }

    pub(crate) fn replace<T, F>(&mut self, key: &str, f: F) -> Result<()>
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

    // VERSIONING
    // ----------

    pub fn checkpoint(&mut self) -> Result<()> {
        self.block_on(self.storage().checkpoint())?;
        Ok(())
    }

    pub fn commit(&mut self) -> Result<(Layer, State)> {
        Ok(self.block_on(self.storage().commit())?)
    }

    pub fn current_layer(&mut self) -> Result<(Layer, State)> {
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

//#[cfg(test)]
//mod test {
//    use super::*;
//
//    #[test]
//    fn set_account_then_get() {
//        let mut gs = GlobalState::in_memory();
//        let account_addr = Address::zeros();
//
//        gs.set_account(
//            &account_addr,
//            "@foobar".to_string(),
//            TemplateAddr::zeros(),
//            42,
//            1337,
//        );
//
//        assert_eq!(gs.account_name(&account_addr).unwrap().unwrap(), "@foobar");
//        assert_eq!(
//            gs.account_template_addr(&account_addr).unwrap().unwrap(),
//            TemplateAddr::zeros()
//        );
//        assert_eq!(gs.account_balance(&account_addr).unwrap().unwrap(), 42);
//        assert_eq!(gs.account_counter(&account_addr).unwrap().unwrap(), 1337);
//    }
//}
