use crate::{
    error::{ParseError, StoreError},
    raw::NibbleIter,
    traits::{
        AppAddressCompute, AppDeserializer, AppSerializer, AppStore, AppTemplateAddressCompute,
        AppTemplateDeserializer, AppTemplateHasher, AppTemplateSerializer, AppTemplateStore,
    },
    types::{
        App, AppAddr, AppTemplate, AppTemplateHash, AppTransaction, AuthorAddr, CreatorAddr,
        HostCtx, SpawnApp, TemplateAddr,
    },
};

use svm_common::Address;

/// `Env` storage serialization types
pub trait EnvSerializerTypes {
    /// `AppTemplate`'s Serializer
    type TemplateSerializer: AppTemplateSerializer;

    /// `AppTemplate`'s Deserializer
    type TemplateDeserializer: AppTemplateDeserializer;

    /// `App`'s Serializer
    type AppSerializer: AppSerializer;

    /// `App`'s Deserializer
    type AppDeserializer: AppDeserializer;
}

/// Aggregates types that are required by `Env`
pub trait EnvTypes {
    /// `AppTemplate` store type.
    type TemplateStore: AppTemplateStore;

    /// `AppStore` store type.
    type AppStore: AppStore;

    /// Compute `AppTemplate` address type.
    type AppTemplateAddressCompute: AppTemplateAddressCompute;

    /// Compute `App` address type.
    type AppAddressCompute: AppAddressCompute;

    /// `AppTemplate` content hasher type.
    type TemplateHasher: AppTemplateHasher;
}

/// A trait for managing the `SVM` app environment.
pub trait Env {
    /// `Env` environment is dictated by its `Types`
    type Types: EnvTypes;

    /// Borrows environment's `AppTemplate`(s) store
    fn get_template_store(&self) -> &<Self::Types as EnvTypes>::TemplateStore;

    /// Borrows mutably environment's `AppTemplate`(s) store
    fn get_template_store_mut(&mut self) -> &mut <Self::Types as EnvTypes>::TemplateStore;

    /// Borrows environment's `App`(s) store
    fn get_app_store(&self) -> &<Self::Types as EnvTypes>::AppStore;

    /// Borrows mutably environment's `App`(s) store
    fn get_app_store_mut(&mut self) -> &mut <Self::Types as EnvTypes>::AppStore;

    /// Computes `AppTemplate` Hash
    fn compute_template_hash(&self, template: &AppTemplate) -> AppTemplateHash {
        <Self::Types as EnvTypes>::TemplateHasher::hash(template)
    }

    /// Computes `AppTemplate` account address
    fn derive_template_address(&self, template: &AppTemplate, host_ctx: &HostCtx) -> TemplateAddr {
        <Self::Types as EnvTypes>::AppTemplateAddressCompute::compute(template, host_ctx)
    }

    /// Computes `App` account address
    fn derive_app_address(&self, spawn: &SpawnApp, host_ctx: &HostCtx) -> AppAddr {
        <Self::Types as EnvTypes>::AppAddressCompute::compute(spawn, host_ctx)
    }

    /// Wire

    fn parse_deploy_template(&self, bytes: &[u8]) -> Result<AppTemplate, ParseError> {
        let mut iter = NibbleIter::new(bytes);

        crate::raw::decode_deploy_template(&mut iter)
    }

    fn parse_spawn_app(&self, bytes: &[u8]) -> Result<SpawnApp, ParseError> {
        let mut iter = NibbleIter::new(bytes);

        crate::raw::decode_spawn_app(&mut iter)
    }

    fn parse_exec_app(&self, bytes: &[u8]) -> Result<AppTransaction, ParseError> {
        let mut iter = NibbleIter::new(bytes);

        crate::raw::decode_exec_app(&mut iter)
    }

    /// Stores the following:
    /// * `TemplateAddress` -> `TemplateHash`
    /// * `TemplateHash` -> `AppTemplate` data
    #[must_use]
    fn store_template(
        &mut self,
        template: &AppTemplate,
        author: &AuthorAddr,
        host_ctx: &HostCtx,
    ) -> Result<TemplateAddr, StoreError> {
        let addr = self.derive_template_address(template, host_ctx);
        let hash = self.compute_template_hash(template);

        let store = self.get_template_store_mut();
        store.store(template, author, &addr, &hash)?;

        Ok(addr)
    }

    /// Stores `app address` -> `app-template address` relation.
    #[must_use]
    fn store_app(
        &mut self,
        spawn: &SpawnApp,
        creator: &CreatorAddr,
        host_ctx: &HostCtx,
    ) -> Result<AppAddr, StoreError> {
        let app = &spawn.app;
        let template = &app.template;

        if self.template_exists(template) {
            let addr = self.derive_app_address(spawn, host_ctx);
            let store = self.get_app_store_mut();

            store.store(app, creator, &addr)?;

            Ok(addr)
        } else {
            // important:
            // Normally code shuld never execute this piece.
            // The Runtime (defined at the `svm-runtime` crate) was supposed to pre-validate the existence
            // of the `AppTemplate` prior to calling the `Env` for storing the new `App`.

            let msg = format!(
                "`AppTemplate` not found (address = `{:?}`)",
                app.template.inner()
            );

            let err = StoreError::DataCorruption(msg);
            Err(err)
        }
    }

    /// Given an `App` address, loads the `AppTemplate` the app is associated with.
    fn load_template_by_app(
        &self,
        addr: &AppAddr,
    ) -> Option<(AppTemplate, TemplateAddr, AuthorAddr, CreatorAddr)> {
        if let Some((app, creator)) = self.load_app(addr) {
            if let Some((template, author)) = self.load_template(&app.template) {
                return Some((template, app.template, author, creator));
            }
        }

        None
    }

    /// Loads an `AppTemplate` given its `Address`
    #[must_use]
    fn load_template(&self, addr: &TemplateAddr) -> Option<(AppTemplate, AuthorAddr)> {
        let store = self.get_template_store();
        store.load(&addr)
    }

    /// Loads an `App` given its `Address`
    #[must_use]
    fn load_app(&self, addr: &AppAddr) -> Option<(App, CreatorAddr)> {
        let store = self.get_app_store();
        store.load(&addr)
    }

    #[inline]
    fn template_exists(&self, addr: &TemplateAddr) -> bool {
        self.load_template(addr).is_some()
    }

    /// Given an `Address`, returns whether it's associated with some `App`
    #[inline]
    fn app_exists(&self, addr: &AppAddr) -> bool {
        self.load_app(addr).is_some()
    }
}
