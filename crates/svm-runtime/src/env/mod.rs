use std::io::Cursor;

use svm_codec::ParseError;
use svm_codec::{app, template, transaction};

use svm_types::{AppAddr, SpawnApp, Template, TemplateAddr, Transaction};

/// Default implementations
mod default;
pub use default::{DefaultAppAddressCompute, DefaultTemplateAddressCompute};

/// Extensions
mod ext;

pub use ext::{ExtApp, ExtSpawnApp, ExtTemplate};

/// In-memory types
#[cfg(feature = "default-memory")]
mod memory;

#[cfg(feature = "default-memory")]
pub use memory::{MemAppStore, MemTemplateStore};

#[cfg(feature = "default-memory")]
pub use default::{DefaultMemAppStore, DefaultMemEnvTypes, DefaultMemTemplateStore};

/// Rocksdb related types
#[cfg(feature = "default-rocksdb")]
mod rocksdb;

#[cfg(feature = "default-rocksdb")]
pub use rocksdb::{RocksAppStore, RocksTemplateStore};

#[cfg(feature = "default-rocksdb")]
pub use default::{DefaultRocksAppStore, DefaultRocksEnvTypes, DefaultRocksTemplateStore};

/// Runtime traits
mod traits;

pub use traits::{
    AppAddressCompute, AppStore, EnvTypes, TemplateAddressCompute, TemplateHasher, TemplateStore,
};

/// Runtime types
pub mod hash;
use hash::TemplateHash;

pub struct Env<T>
where
    T: EnvTypes,
{
    app_store: <T as EnvTypes>::AppStore,

    template_store: <T as EnvTypes>::TemplateStore,
}

impl<T> Env<T>
where
    T: EnvTypes,
{
    /// `Env` environment is dictated by its `Types`

    /// Creates a new `Env`. Injects externally the `TemplateStore` and `AppStore`.
    pub fn new(
        app_store: <T as EnvTypes>::AppStore,
        template_store: <T as EnvTypes>::TemplateStore,
    ) -> Self {
        Self {
            app_store,
            template_store,
        }
    }

    /// Borrows environment's `TemplateStore`
    pub fn get_template_store(&self) -> &<T as EnvTypes>::TemplateStore {
        &self.template_store
    }

    /// Borrows mutably a `TemplateStore`
    pub fn get_template_store_mut(&mut self) -> &mut <T as EnvTypes>::TemplateStore {
        &mut self.template_store
    }

    /// Borrows environment's `AppStore`
    pub fn get_app_store(&self) -> &<T as EnvTypes>::AppStore {
        &self.app_store
    }

    /// Borrows mutably environment's `App`(s) store
    pub fn get_app_store_mut(&mut self) -> &mut <T as EnvTypes>::AppStore {
        &mut self.app_store
    }

    /// Computes `TemplateHash`
    pub fn compute_template_hash(&self, template: &ExtTemplate) -> TemplateHash {
        <T as EnvTypes>::TemplateHasher::hash(template.base())
    }

    /// Computes `Template` account address
    pub fn derive_template_address(&self, template: &ExtTemplate) -> TemplateAddr {
        <T as EnvTypes>::TemplateAddressCompute::compute(template)
    }

    /// Computes `App` account `Address`
    pub fn derive_app_address(&self, spawn: &ExtSpawnApp) -> AppAddr {
        <T as EnvTypes>::AppAddressCompute::compute(spawn)
    }

    /// Wire

    /// Parses raw a `Template`
    ///
    /// On success returns `Template`,
    /// On failure returns `ParseError`.
    pub fn parse_deploy_template(&self, bytes: &[u8]) -> Result<Template, ParseError> {
        let mut cursor = Cursor::new(bytes);

        let template = template::decode_deploy_template(&mut cursor)?;

        Ok(template)
    }

    /// Parses raw a `SpawnApp`
    ///
    /// On success returns `SpawnApp`,
    /// On failure returns `ParseError`.
    pub fn parse_spawn_app(&self, bytes: &[u8]) -> Result<SpawnApp, ParseError> {
        let mut cursor = Cursor::new(bytes);

        let spawn = app::decode_spawn_app(&mut cursor)?;

        Ok(spawn)
    }

    /// Parses raw a `Transaction`
    ///
    /// On success returns `AppTransaction`,
    /// On failure returns `ParseError`.
    pub fn parse_exec_app(&self, bytes: &[u8]) -> Result<Transaction, ParseError> {
        let mut cursor = Cursor::new(bytes);

        let tx = transaction::decode_exec_app(&mut cursor)?;

        Ok(tx)
    }

    pub fn store_template(&mut self, template: &ExtTemplate, addr: &TemplateAddr) {
        let hash = self.compute_template_hash(template);

        let store = self.get_template_store_mut();

        store.store(template, &addr, &hash);
    }

    /// Stores `app address` -> `app-template address` relation.
    pub fn store_app(&mut self, app: &ExtApp, addr: &AppAddr) {
        let template = app.template_addr();

        if self.template_exists(template) {
            let store = self.get_app_store_mut();

            store.store(app, &addr);
        } else {
            unreachable!("Should have validated template transaction first.");
        }
    }

    pub fn find_template_addr(&self, addr: &AppAddr) -> Option<TemplateAddr> {
        let store = self.get_app_store();

        store.find_template_addr(&addr)
    }

    /// Given an `App` Address, loads the `Template` the app is associated with.
    pub fn load_template_by_app(&self, addr: &AppAddr) -> Option<ExtTemplate> {
        self.load_app(addr).and_then(|app| {
            let addr = app.template_addr();

            self.load_template(addr)
        })
    }

    /// Loads an `Template` given its `Address`
    #[must_use]
    pub fn load_template(&self, addr: &TemplateAddr) -> Option<ExtTemplate> {
        let store = self.get_template_store();

        store.load(&addr)
    }

    /// Loads an `App` given its `Address`
    #[must_use]
    pub fn load_app(&self, addr: &AppAddr) -> Option<ExtApp> {
        let store = self.get_app_store();

        store.load(&addr)
    }

    /// Returns whether a `Template` with given the `Address` exists.
    #[inline]
    pub fn template_exists(&self, addr: &TemplateAddr) -> bool {
        self.load_template(addr).is_some()
    }

    /// Returns whether an `App` with given the `Address` exists.
    #[inline]
    pub fn app_exists(&self, addr: &AppAddr) -> bool {
        self.load_app(addr).is_some()
    }
}
