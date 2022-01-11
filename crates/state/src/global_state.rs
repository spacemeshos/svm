use tokio::runtime::Runtime;

use std::sync::{Arc, Mutex, MutexGuard};

use svm_codec::Codec;
use svm_genesis_config::GenesisConfig;
use svm_hash::{Blake3Hasher, Hasher};
use svm_types::{BytesPrimitive, Layer, State};

use crate::storage::Storage;
use crate::{StorageError, StorageResult as Result, TemplateStorage};

/// A key-value store with a non-falsifiable state signature, historical data
/// querying and other features which make it suitable for storing Spacemesh'
/// global state.
///
/// This data structure is backed by SQLite.
#[derive(Debug, Clone)]
pub struct GlobalState {
    pub(crate) storage: Arc<Mutex<Storage>>,
    runtime: Arc<Mutex<Runtime>>,
    layer_query_parameter: Option<Layer>,
    genesis_state: State,
}

impl GlobalState {
    /// Creates a new [`GlobalState`] from the database instance sitting at
    /// `sqlite_uri` and with the given [`GenesisConfig`].
    pub fn new(sqlite_uri: &str, genesis: GenesisConfig) -> Self {
        tracing::info!(
            sqlite_uri = sqlite_uri,
            "Intitializing a new global state database."
        );

        let runtime = Runtime::new().unwrap();
        let storage = runtime.block_on(Storage::new(sqlite_uri)).unwrap();
        let mut gs = Self {
            storage: Arc::new(Mutex::new(storage)),
            runtime: Arc::new(Mutex::new(runtime)),
            layer_query_parameter: None,
            genesis_state: State::zeros(),
        };
        gs.init_genesis(genesis)
            .expect("Genesis initialization failed.");
        gs
    }

    /// Creates a pristine [`GlobalState`] backed by an in-memory SQLite
    /// instance. No disk operations at all will be done.
    pub fn in_memory(genesis: GenesisConfig) -> Self {
        Self::new(":memory:", genesis)
    }

    fn init_genesis(&mut self, genesis: GenesisConfig) -> Result<()> {
        let last_layer_id = self.block_on(self.storage().last_layer_id())?;

        tracing::debug!("Initializing genesis configuration.");

        if last_layer_id.is_some() {
            tracing::debug!("The database is not empty. Genesis is assumed to have been configured already. Skipping.");
            return Ok(());
        }

        tracing::info!(
            num_templates = genesis.templates.len(),
            "The global state is not initialized. Writing templates."
        );

        for (template_addr, template) in genesis.templates {
            let mut core_sections = template.sections().clone();
            let noncore_sections = core_sections.remove_noncore();

            tracing::debug!(
                template_addr = template_addr.to_string().as_str(),
                "Initializing a new template for genesis."
            );
            println!("new template with addr {}", template_addr.to_string());
            TemplateStorage::create(
                self.clone(),
                &template_addr,
                core_sections,
                noncore_sections,
            )?;
        }

        self.checkpoint()?;
        let (layer_id, state) = self.block_on(self.storage().commit())?;

        debug_assert_eq!(layer_id, -1);
        debug_assert_eq!(self.block_on(self.storage().last_layer_id())?, Some(0));

        self.genesis_state = state;

        tracing::info!(
            state_fingerprint = state.to_string().as_str(),
            "Genesis initialization completed."
        );

        Ok(())
    }

    /// Returns a mutable reference to the [`Option<Layer>`] that controls the
    /// historical query parameter used in *all* read operations. When set to
    /// [`None`], the most recent value is always read; when set to
    /// [`Some<some_layer>`], only the value present at the time of `some_layer`
    /// is read.
    pub fn layer_query_parameter_mut(&mut self) -> &mut Option<Layer> {
        &mut self.layer_query_parameter
    }

    // VERSIONING
    // ----------

    /// Saves dirty changes in preparation of [`GlobalState::commit`]. After
    /// saving, changes are frozen and can't be removed from the current layer.
    pub fn checkpoint(&mut self) -> Result<()> {
        self.block_on(self.storage().checkpoint())?;
        Ok(())
    }

    /// Persists all changes to disk and returns the root [`State`] of the new
    /// layer. It returns a [`StorageError::DirtyChanges`] in case there's any
    /// dirty changes that haven't been saved via [`GlobalState::checkpoint`]
    /// before this call.
    pub fn commit(&mut self) -> Result<(Layer, State)> {
        let (layer_id, state) = self.block_on(self.storage().commit())?;
        Ok((Layer(layer_id.try_into().unwrap()), state))
    }

    /// Returns the [`Layer`] and [`State`] of the last ever committed
    /// layer; i.e. persisted changes without dirty and saved changes.
    pub fn current_layer(&mut self) -> Result<(Layer, State)> {
        let (layer_id, state) = self.block_on(self.storage().last_layer())?;
        Ok((Layer(layer_id.try_into().unwrap()), state))
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
        self.block_on(self.storage().rewind(layer_id.0.try_into().unwrap()))?;
        Ok(())
    }

    /// Returns [`true`] iff there are any dirty changes in memory; [`false`]
    /// otherwise.
    pub fn has_uncommitted_changes(&self) -> bool {
        self.storage().has_uncommitted_changes()
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

    pub(crate) fn read_and_decode<T>(&self, key: &str) -> Result<T>
    where
        T: Codec,
    {
        let bytes = self
            .block_on(self.storage().get(
                key.as_bytes(),
                self.layer_query_parameter.map(|layer| layer.0 as i64),
            ))?
            .ok_or(StorageError::NotFound {
                key_hash: State(Blake3Hasher::hash(key.as_bytes())),
            })?;

        T::decode_bytes(bytes).map_err(|_| StorageError::IllegalData {
            key_hash: State(Blake3Hasher::hash(key.as_bytes())),
        })
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
        let old_item = self.read_and_decode::<T>(key)?;

        self.encode_and_write(&f(old_item), key);
        Ok(())
    }
}
