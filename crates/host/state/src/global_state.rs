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
    layer_query_parameter: Option<Layer>,
    genesis_state: State,
}

impl GlobalState {
    /// Creates a new [`GlobalState`] from the database instance sitting at
    /// `sqlite_uri` and with the given [`GenesisConfig`].
    pub async fn new(sqlite_uri: &str, genesis: GenesisConfig) -> Self {
        tracing::info!(
            sqlite_uri = sqlite_uri,
            "Intitializing a new global state database."
        );

        let storage = Storage::new(sqlite_uri).await.unwrap();
        let mut gs = Self {
            storage: Arc::new(Mutex::new(storage)),
            layer_query_parameter: None,
            genesis_state: State::zeros(),
        };
        gs.init_genesis(genesis)
            .await
            .expect("Genesis initialization failed.");
        gs
    }

    /// Creates a pristine [`GlobalState`] backed by an in-memory SQLite
    /// instance. No disk operations at all will be done.
    pub async fn in_memory(genesis: GenesisConfig) -> Self {
        Self::new(":memory:", genesis).await
    }

    async fn init_genesis(&mut self, genesis: GenesisConfig) -> Result<()> {
        tracing::debug!("Initializing genesis configuration.");

        let genesis_fingerprint = self.storage().genesis_fingerprint().await?;
        if genesis_fingerprint.is_some() {
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
            TemplateStorage::create(
                self.clone(),
                &template_addr,
                core_sections,
                noncore_sections,
            )
            .await?;
        }

        self.checkpoint().await?;
        let (layer_id, state) = self.storage().commit().await?;

        debug_assert_eq!(layer_id, -1);
        debug_assert_eq!(self.storage().last_layer_id().await?, Some(-1));

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
    pub async fn checkpoint(&mut self) -> Result<()> {
        self.storage().checkpoint().await?;
        Ok(())
    }

    /// Persists all changes to disk and returns the root [`State`] of the new
    /// layer. It returns a [`StorageError::DirtyChanges`] in case there's any
    /// dirty changes that haven't been saved via [`GlobalState::checkpoint`]
    /// before this call.
    pub async fn commit(&mut self) -> Result<(Layer, State)> {
        let (layer_id, state) = self.storage().commit().await?;
        Ok((Layer(layer_id.try_into().unwrap()), state))
    }

    /// Returns the [`Layer`] and [`State`] of the last ever committed
    /// layer; i.e. persisted changes without dirty and saved changes.
    pub async fn current_layer(&mut self) -> Result<(Layer, State)> {
        let (layer_id, state) = self.storage().last_layer().await?;
        Ok((Layer(layer_id.try_into().unwrap()), state))
    }

    /// Erases all dirty changes from memory. Persisted and saved data are left
    /// untouched.
    pub async fn rollback(&mut self) -> Result<()> {
        self.storage().rollback().await?;
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
    pub async fn rewind(&mut self, layer_id: Layer) -> Result<()> {
        self.storage()
            .rewind(layer_id.0.try_into().unwrap())
            .await?;
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

    pub(crate) async fn read_and_decode<T>(&self, key: &str) -> Result<T>
    where
        T: Codec,
    {
        let bytes = self
            .storage()
            .get(
                key.as_bytes(),
                self.layer_query_parameter.map(|layer| layer.0 as i64),
            )
            .await?
            .ok_or(StorageError::NotFound {
                key_hash: State(Blake3Hasher::hash(key.as_bytes())),
            })?;

        T::decode_bytes(bytes).map_err(|_| StorageError::IllegalData {
            key_hash: State(Blake3Hasher::hash(key.as_bytes())),
        })
    }

    pub(crate) async fn encode_and_write<T>(&mut self, item: &T, key: &str) -> ()
    where
        T: Codec,
    {
        self.storage()
            .upsert(key.as_bytes(), item.encode_to_vec())
            .await;
    }

    pub(crate) async fn replace<T, F>(&mut self, key: &str, f: F) -> Result<()>
    where
        T: Codec,
        F: Fn(T) -> T,
    {
        let old_item = self.read_and_decode::<T>(key).await?;

        self.encode_and_write(&f(old_item), key).await;
        Ok(())
    }
}
