use std::io::Cursor;

use crate::env::{hash, serialize, traits};
use crate::env::{ExtApp, ExtSpawnApp, ExtTemplate};

use hash::TemplateHash;
use traits::{
    AppAddressCompute, AppDeserializer, AppSerializer, AppStore, TemplateAddressCompute,
    TemplateDeserializer, TemplateHasher, TemplateSerializer, TemplateStore,
};

use svm_codec::ParseError;
use svm_codec::{app, template, transaction};
use svm_types::{
    App, AppAddr, AuthorAddr, SpawnApp, SpawnerAddr, Template, TemplateAddr, Transaction,
};

/// `Env` storage serialization types
pub trait EnvSerializerTypes {
    /// `Template`'s Serializer
    type TemplateSerializer: TemplateSerializer;

    /// `Template`'s Deserializer
    type TemplateDeserializer: TemplateDeserializer;

    /// `App`'s Serializer
    type AppSerializer: AppSerializer;

    /// `App`'s Deserializer
    type AppDeserializer: AppDeserializer;
}

/// Aggregates types that are required by `Env`
pub trait EnvTypes {
    /// `Template` store type.
    type TemplateStore: TemplateStore;

    /// `AppStore` store type.
    type AppStore: AppStore;

    /// Compute `Template` address type.
    type TemplateAddressCompute: TemplateAddressCompute;

    /// Compute `App` address type.
    type AppAddressCompute: AppAddressCompute;

    /// `Template` content hasher type.
    type TemplateHasher: TemplateHasher;
}

/// A trait for managing the `SVM` app environment.
pub trait Env {
    /// `Env` environment is dictated by its `Types`
    type Types: EnvTypes;

    /// Borrows environment's `Template`(s) store
    fn get_template_store(&self) -> &<Self::Types as EnvTypes>::TemplateStore;

    /// Borrows mutably environment's `TemplateStore`
    fn get_template_store_mut(&mut self) -> &mut <Self::Types as EnvTypes>::TemplateStore;

    /// Borrows environment's `App`(s) store
    fn get_app_store(&self) -> &<Self::Types as EnvTypes>::AppStore;

    /// Borrows mutably environment's `App`(s) store
    fn get_app_store_mut(&mut self) -> &mut <Self::Types as EnvTypes>::AppStore;

    /// Computes `Template` Hash
    fn compute_template_hash(&self, template: &ExtTemplate) -> TemplateHash {
        <Self::Types as EnvTypes>::TemplateHasher::hash(template.base())
    }

    /// Computes `Template` account address
    fn derive_template_address(&self, template: &ExtTemplate) -> TemplateAddr {
        <Self::Types as EnvTypes>::TemplateAddressCompute::compute(template)
    }

    /// Computes `App` account address
    fn derive_app_address(&self, spawn: &ExtSpawnApp) -> AppAddr {
        <Self::Types as EnvTypes>::AppAddressCompute::compute(spawn)
    }

    /// Wire

    /// Parses raw a deploy-template.
    /// On success returns `Template`,
    /// On failure returns `ParseError`.
    fn parse_deploy_template(&self, bytes: &[u8]) -> Result<Template, ParseError> {
        let mut cursor = Cursor::new(bytes);

        let template = template::decode_deploy_template(&mut cursor)?;

        Ok(template)
    }

    /// Parses raw a spawned-app.
    /// On success returns `SpawnApp`,
    /// On failure returns `ParseError`.
    fn parse_spawn_app(&self, bytes: &[u8]) -> Result<SpawnApp, ParseError> {
        let mut cursor = Cursor::new(bytes);

        let spawn = app::decode_spawn_app(&mut cursor)?;

        Ok(spawn)
    }

    /// Parses raw a app-transation to execute.
    /// On success returns `AppTransaction`,
    /// On failure returns `ParseError`.
    fn parse_exec_app(&self, bytes: &[u8]) -> Result<Transaction, ParseError> {
        let mut cursor = Cursor::new(bytes);

        let tx = transaction::decode_exec_app(&mut cursor)?;

        Ok(tx)
    }

    fn store_template(&mut self, template: &ExtTemplate, addr: &TemplateAddr) {
        let hash = self.compute_template_hash(template);

        let store = self.get_template_store_mut();

        store.store(template, &addr, &hash);
    }

    /// Stores `app address` -> `app-template address` relation.
    fn store_app(&mut self, app: &ExtApp, addr: &AppAddr) {
        let template = app.template_addr();

        if self.template_exists(template) {
            let store = self.get_app_store_mut();

            store.store(app, &addr);
        } else {
            unreachable!("Should have validated template transaction first.");
        }
    }

    /// Given an `App` address, loads the `Template` the app is associated with.
    fn load_template_by_app(&self, addr: &AppAddr) -> Option<ExtTemplate> {
        self.load_app(addr).and_then(|app| {
            let addr = app.template_addr();

            self.load_template(addr)
        })
    }

    /// Loads an `Template` given its `Address`
    #[must_use]
    fn load_template(&self, addr: &TemplateAddr) -> Option<ExtTemplate> {
        let store = self.get_template_store();
        store.load(&addr)
    }

    /// Loads an `App` given its `Address`
    #[must_use]
    fn load_app(&self, addr: &AppAddr) -> Option<ExtApp> {
        let store = self.get_app_store();
        store.load(&addr)
    }

    /// Returns whether a `Template` with given the `Address` exists.
    #[inline]
    fn template_exists(&self, addr: &TemplateAddr) -> bool {
        self.load_template(addr).is_some()
    }

    /// Returns whether an `App` with given the `Address` exists.
    #[inline]
    fn app_exists(&self, addr: &AppAddr) -> bool {
        self.load_app(addr).is_some()
    }
}
