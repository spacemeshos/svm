//! Managing the Runtime's environment

use svm_codec::ParseError;
use svm_codec::{app, template, transaction};
use svm_gas::PriceResolver;
use svm_types::{AccountAddr, SectionKind, SpawnApp, Template, TemplateAddr, Transaction};

use std::collections::HashSet;
use std::io::Cursor;

/// Default implementations
mod default;
pub use default::{DefaultAppAddressCompute, DefaultTemplateAddressCompute};

/// Extensions
mod ext;

pub use ext::{ExtApp, ExtSpawnApp};

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

mod traits;

pub use traits::{AppStore, ComputeAddress, TemplateHasher, TemplateStore};

/// Represents an `Template` Hash.
pub type TemplateHash = [u8; 32];

pub trait EnvTypes {
    /// `Template` store type.
    type TemplateStore: TemplateStore;

    /// `AppStore` store type.
    type AppStore: AppStore;

    /// Compute `Template` address type.
    type TemplateAddressCompute: ComputeAddress<Template, Address = TemplateAddr>;

    /// Compute `App` address type.
    type AppAddressCompute: ComputeAddress<ExtSpawnApp, Address = AccountAddr>;

    /// `Template` content Hasher type.
    type TemplateHasher: TemplateHasher;

    /// A pricing engine for templates.
    type Pricer: PriceResolver;
}

pub struct Env<T>
where
    T: EnvTypes,
{
    app_store: T::AppStore,
    template_store: T::TemplateStore,
}

impl<T> Env<T>
where
    T: EnvTypes,
{
    /// `Env` environment is dictated by its `Types`

    /// Creates a new `Env`. Injects externally the `TemplateStore` and `AppStore`.
    pub fn new(app_store: T::AppStore, template_store: T::TemplateStore) -> Self {
        Self {
            app_store,
            template_store,
        }
    }

    /// Borrows environment's `TemplateStore`
    pub fn template_store(&self) -> &T::TemplateStore {
        &self.template_store
    }

    /// Borrows mutably a `TemplateStore`
    pub fn template_store_mut(&mut self) -> &mut T::TemplateStore {
        &mut self.template_store
    }

    /// Borrows environment's `AppStore`
    pub fn account_store(&self) -> &T::AppStore {
        &self.app_store
    }

    /// Borrows mutably environment's `App`(s) store
    pub fn account_store_mut(&mut self) -> &mut T::AppStore {
        &mut self.app_store
    }

    /// Computes a [`TemplateHash`].
    pub fn compute_template_hash(&self, template: &Template) -> TemplateHash {
        T::TemplateHasher::hash(template)
    }

    /// Computes a `Template`'s `Address`
    pub fn compute_template_addr(&self, template: &Template) -> TemplateAddr {
        T::TemplateAddressCompute::compute(template)
    }

    /// Computes an `Account`'s `Address`
    pub fn compute_account_addr(&self, spawn: &ExtSpawnApp) -> AccountAddr {
        T::AppAddressCompute::compute(spawn)
    }

    /// Parses a raw `Template`
    ///
    /// On success returns `Template`,
    /// On failure returns `ParseError`.
    pub fn parse_deploy_template(
        &self,
        bytes: &[u8],
        interests: Option<HashSet<SectionKind>>,
    ) -> Result<Template, ParseError> {
        let cursor = Cursor::new(bytes);
        let template = template::decode(cursor, interests)?;

        Ok(template)
    }

    /// Parses a raw `SpawnApp`
    ///
    /// On success returns `SpawnApp`,
    /// On failure returns `ParseError`.
    pub fn parse_spawn_app(&self, bytes: &[u8]) -> Result<SpawnApp, ParseError> {
        let mut cursor = Cursor::new(bytes);

        let spawn = app::decode(&mut cursor)?;

        Ok(spawn)
    }

    /// Parses a raw `Transaction`
    ///
    /// On success returns `AppTransaction`,
    /// On failure returns `ParseError`.
    pub fn parse_exec_app(&self, bytes: &[u8]) -> Result<Transaction, ParseError> {
        let mut cursor = Cursor::new(bytes);

        let tx = transaction::decode_exec_app(&mut cursor)?;

        Ok(tx)
    }

    pub fn store_template(&mut self, template: &Template, addr: &TemplateAddr) {
        let hash = self.compute_template_hash(template);

        let store = self.template_store_mut();

        store.store(template, &addr, &hash);
    }

    /// Stores `app address` -> `app-template address` relation.
    pub fn store_app(&mut self, app: &ExtApp, addr: &AccountAddr) {
        let template = app.template_addr();

        if self.contains_template(template) {
            let store = self.account_store_mut();

            store.store(app, &addr);
        } else {
            unreachable!("Should have validated template transaction first.");
        }
    }

    pub fn resolve_template_addr(&self, addr: &AccountAddr) -> Option<TemplateAddr> {
        let store = self.account_store();

        store.resolve_template_addr(&addr)
    }

    /// Given an `Account` Address, loads the associated `Template`
    pub fn account_template(
        &self,
        addr: &AccountAddr,
        interests: Option<HashSet<SectionKind>>,
    ) -> Option<Template> {
        self.account(addr).and_then(|app| {
            let addr = app.template_addr();
            self.template(addr, interests)
        })
    }

    /// Loads an `Template` given its `Address`
    #[must_use]
    pub fn template(
        &self,
        addr: &TemplateAddr,
        interests: Option<HashSet<SectionKind>>,
    ) -> Option<Template> {
        let store = self.template_store();

        store.load(&addr, interests)
    }

    /// Loads an `Account` given its `Address`
    #[must_use]
    pub fn account(&self, addr: &AccountAddr) -> Option<ExtApp> {
        let store = self.account_store();

        store.load(&addr)
    }

    /// Returns whether a `Template` with given the `Address` exists.
    #[inline]
    pub fn contains_template(&self, addr: &TemplateAddr) -> bool {
        self.template(addr, None).is_some()
    }

    /// Returns whether an `Account` with given the `Address` exists.
    #[inline]
    pub fn contains_account(&self, addr: &AccountAddr) -> bool {
        self.account(addr).is_some()
    }
}
