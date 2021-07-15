//! Managing a `Runtime`'s environment (see [`Env`]).

use svm_codec::ParseError;
use svm_codec::{call, spawn, template};
use svm_gas::PriceResolver;
use svm_types::{AccountAddr, SectionKind, SpawnAccount, Template, TemplateAddr, Transaction};

use std::collections::HashSet;
use std::io::Cursor;

/// Default implementations
mod default;
pub use default::{DefaultAccountAddressCompute, DefaultTemplateAddressCompute};

/// Extensions
mod ext;

pub use ext::{ExtAccount, ExtSpawn};

/// In-memory types
#[cfg(feature = "default-memory")]
mod memory;

#[cfg(feature = "default-memory")]
pub use memory::{MemAccountStore, MemTemplateStore};

#[cfg(feature = "default-memory")]
pub use default::{DefaultMemAccountStore, DefaultMemEnvTypes, DefaultMemTemplateStore};

/// Rocksdb related types
#[cfg(feature = "default-rocksdb")]
mod rocksdb;

#[cfg(feature = "default-rocksdb")]
pub use rocksdb::{RocksAccountStore, RocksTemplateStore};

#[cfg(feature = "default-rocksdb")]
pub use default::{DefaultRocksEnvTypes, DefaultRocksTemplateStore};

mod traits;

pub use traits::{AccountStore, ComputeAddress, TemplateHasher, TemplateStore};

/// Represents an `Template` Hash.
pub type TemplateHash = [u8; 32];

pub trait EnvTypes {
    /// [`Template`] type.
    type TemplateStore: TemplateStore;

    /// [`AccountStore`] type.
    type AccountStore: AccountStore;

    /// Compute a `Template` `Address`
    type TemplateAddressCompute: ComputeAddress<Template, Address = TemplateAddr>;

    /// Compute an `Account`'s `Address`
    type AccountAddressCompute: ComputeAddress<ExtSpawn, Address = AccountAddr>;

    /// `Template` content [`TemplateHasher`] type.
    type TemplateHasher: TemplateHasher;

    /// A `Gas` [`PriceResolver`] type.
    type Pricer: PriceResolver;
}

pub struct Env<T>
where
    T: EnvTypes,
{
    accounts: T::AccountStore,
    templates: T::TemplateStore,
}

impl<T> Env<T>
where
    T: EnvTypes,
{
    /// `Env` environment is dictated by its `Types`

    /// Creates a new [`Env`]. Injects the `TemplateStore` and `AccountStore`.
    pub fn new(account_store: T::AccountStore, template_store: T::TemplateStore) -> Self {
        Self {
            accounts: account_store,
            templates: template_store,
        }
    }

    /// Borrows environment's `TemplateStore`.
    pub fn template_store(&self) -> &T::TemplateStore {
        &self.templates
    }

    /// Borrows mutably a `TemplateStore`.
    pub fn template_store_mut(&mut self) -> &mut T::TemplateStore {
        &mut self.templates
    }

    /// Borrows the environment's [`AccountStore`].
    pub fn account_store(&self) -> &T::AccountStore {
        &self.accounts
    }

    /// Mutably Borrows the environment's [`AccountStore`].
    pub fn account_store_mut(&mut self) -> &mut T::AccountStore {
        &mut self.accounts
    }

    /// Computes the [`TemplateHash`] of `template`.
    pub fn compute_template_hash(&self, template: &Template) -> TemplateHash {
        T::TemplateHasher::hash(template)
    }

    /// Computes the `Template`'s `Address` of `template`
    pub fn compute_template_addr(&self, template: &Template) -> TemplateAddr {
        T::TemplateAddressCompute::compute(template)
    }

    /// Computes an `Account`'s `Address`
    pub fn compute_account_addr(&self, spawn: &ExtSpawn) -> AccountAddr {
        T::AccountAddressCompute::compute(spawn)
    }

    /// Parses a binary `Deploy Template` transaction
    ///
    /// On success returns [`Template`],
    /// On failure returns [`ParseError`].
    pub fn parse_deploy(
        &self,
        bytes: &[u8],
        interests: Option<HashSet<SectionKind>>,
    ) -> Result<Template, ParseError> {
        let cursor = Cursor::new(bytes);
        let template = template::decode(cursor, interests)?;

        Ok(template)
    }

    /// Parses a binary [`SpawnAccount`] transaction.
    ///
    /// On success returns [`Spawn Account`],
    /// On failure returns [`ParseError`].
    pub fn parse_spawn(&self, bytes: &[u8]) -> Result<SpawnAccount, ParseError> {
        let mut cursor = Cursor::new(bytes);
        let spawn = spawn::decode(&mut cursor)?;

        Ok(spawn)
    }

    /// Parses a binary `Call Account` (a.k.a a [`Transaction`]).
    ///
    /// On success returns [`Transaction`],
    /// On failure returns [`ParseError`].
    pub fn parse_call(&self, bytes: &[u8]) -> Result<Transaction, ParseError> {
        let mut cursor = Cursor::new(bytes);
        let call = call::decode_call(&mut cursor)?;

        Ok(call)
    }

    pub fn store_template(&mut self, template: &Template, addr: &TemplateAddr) {
        let hash = self.compute_template_hash(template);
        let store = self.template_store_mut();

        store.store(template, &addr, &hash);
    }

    /// Stores an `Account Address` -> `Account`'s `Template Address`.
    pub fn store_account(&mut self, account: &ExtAccount, addr: &AccountAddr) {
        let template = account.template_addr();

        if self.contains_template(template) {
            let store = self.account_store_mut();
            store.store(account, &addr);
        } else {
            unreachable!("Should have validated transaction's associate `Template Address` first.");
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
        self.account(addr).and_then(|account| {
            let addr = account.template_addr();
            self.template(addr, interests)
        })
    }

    /// Loads a [`Template`] given its `Address`
    #[must_use]
    pub fn template(
        &self,
        addr: &TemplateAddr,
        interests: Option<HashSet<SectionKind>>,
    ) -> Option<Template> {
        let store = self.template_store();
        store.load(&addr, interests)
    }

    /// Loads an [`ExtAccount`] given its `Address`
    #[must_use]
    pub fn account(&self, addr: &AccountAddr) -> Option<ExtAccount> {
        let store = self.account_store();
        store.load(&addr)
    }

    /// Returns whether a `Template` with the given `Address` exists.
    #[inline]
    pub fn contains_template(&self, addr: &TemplateAddr) -> bool {
        self.template(addr, None).is_some()
    }

    /// Returns whether an [`Account`] with given the `Address` exists.
    #[inline]
    pub fn contains_account(&self, addr: &AccountAddr) -> bool {
        self.account(addr).is_some()
    }
}
