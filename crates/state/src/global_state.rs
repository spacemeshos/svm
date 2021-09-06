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
    historical_query_context: Option<Layer>,
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
            historical_query_context: None,
        }
    }

    /// Creates a pristine [`GlobalState`] backed by an in-memory SQLite
    /// instance. No disk operations at all will be done.
    pub fn in_memory() -> Self {
        Self::new(":memory:")
    }

    /// Returns a mutable reference to the [`Option<Layer>`] that controls the
    /// historical query parameter used in *all* read operations. When set to
    /// [`None`], the most recent value is always read; when set to
    /// [`Some<some_layer>`], only the value present at the time of `some_layer`
    /// is read.
    pub fn historical_query(&mut self) -> &mut Option<Layer> {
        &mut self.historical_query_context
    }

    // VERSIONING
    // ----------

    /// Saves dirty changes in preparation of [`GlobalState::commit`]. After
    /// saving, changes are frozen and can't be removed from the current layer.
    ///
    /// This might return a [`StorageError::KeyCollision`] depending on the
    /// content of the dirty changes, so beware.
    pub fn checkpoint(&mut self) -> Result<()> {
        self.block_on(self.storage().checkpoint())?;
        Ok(())
    }

    /// Persists all changes to disk and returns the root [`State`] of the new
    /// layer. It returns a [`StorageError::DirtyChanges`] in case there's any
    /// dirty changes that haven't been saved via [`GlobalState::checkpoint`]
    /// before this call.
    pub fn commit(&mut self) -> Result<(Layer, State)> {
        Ok(self.block_on(self.storage().commit())?)
    }

    /// Returns the [`Layer`] and [`State`] of the last ever committed
    /// layer; i.e. persisted changes without dirty and saved changes.
    pub fn current_layer(&mut self) -> Result<(Layer, State)> {
        Ok(self.block_on(self.storage().last_layer())?)
    }

    /// Erases all dirty changes from memory. Persisted and saved data are left
    /// untouched.
    pub fn rollback(&mut self) -> Result<()> {
        self.block_on(self.storage().rollback())?;
        Ok(())
    }

    /// Erases all saved data from memory and completely deletes all layers
    /// after and excluding `layer_id` from the SQLite store. Persisted data is
    /// left untouched. It returns a [`StorageError::DirtyChanges`] in case
    /// there's any dirty changes, i.e. you must call [`GlobalState::rollback`]
    /// beforehand.
    ///
    /// # Panics
    ///
    /// Panics if `layer_id` is invalid.
    pub fn rewind(&mut self, layer_id: Layer) -> Result<()> {
        self.block_on(self.storage().rewind(layer_id))?;
        Ok(())
    }

    // GETTER/SETTER UTILITIES
    // -----------------------

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
        let opt_value = self.block_on(
            self.storage()
                .get(key.as_bytes(), self.historical_query_context),
        )?;

        if let Some(bytes) = opt_value {
            T::decode_bytes(bytes)
                .map(|data| Some(data))
                .map_err(|_| StorageError::IllegalData {
                    key_hash: Blake3Hasher::hash(key.as_bytes()),
                })
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
        let old_item = self
            .read_and_decode::<T>(key)?
            .ok_or(StorageError::NotFound {
                key_hash: Blake3Hasher::hash(key.as_bytes()),
            })?;

        self.encode_and_write(&f(old_item), key);
        Ok(())
    }
}
