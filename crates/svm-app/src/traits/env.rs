use crate::{
    error::{ParseError, StoreError},
    traits::{
        AppAddressCompute, AppDeserializer, AppSerializer, AppStore, AppTemplateAddressCompute,
        AppTemplateDeserializer, AppTemplateHasher, AppTemplateSerializer, AppTemplateStore,
    },
    types::{App, AppTemplate, AppTemplateHash, AppTransaction},
};

use svm_common::Address;

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
    type TemplateStore: AppTemplateStore;

    type AppStore: AppStore;

    type AppTemplateAddressCompute: AppTemplateAddressCompute;

    type AppAddressCompute: AppAddressCompute;

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

    fn get_app_store(&self) -> &<Self::Types as EnvTypes>::AppStore;

    fn get_app_store_mut(&mut self) -> &mut <Self::Types as EnvTypes>::AppStore;

    /// Computes `AppTemplate` Hash
    fn compute_template_hash(&self, template: &AppTemplate) -> AppTemplateHash {
        <Self::Types as EnvTypes>::TemplateHasher::hash(&template.code)
    }

    /// Computes `AppTemplate` account address
    fn derive_template_address(&self, template: &AppTemplate) -> Address {
        <Self::Types as EnvTypes>::AppTemplateAddressCompute::compute(template)
    }

    /// Computes `App` account address
    fn derive_app_address(&self, app: &App) -> Address {
        <Self::Types as EnvTypes>::AppAddressCompute::compute(app)
    }

    /// * Parses a raw template into `AppTemplate`
    /// * Enriches the template with its derived address
    fn parse_template(&self, bytes: &[u8]) -> Result<AppTemplate, ParseError> {
        crate::raw::parse_template(bytes)
    }

    fn parse_app(&self, bytes: &[u8]) -> Result<App, ParseError> {
        crate::raw::parse_app(bytes)
    }

    /// Parses a raw `App` transaction
    fn parse_app_tx(&self, bytes: &[u8]) -> Result<AppTransaction, ParseError> {
        crate::raw::parse_app_tx(bytes)
    }

    /// Stores `TemplateAddress` -> `TemplateHash` -> `AppTemplate`
    #[must_use]
    fn store_template(&mut self, template: &AppTemplate) -> Result<Address, StoreError> {
        let hash = self.compute_template_hash(template);
        let addr = self.derive_template_address(template);

        let store = self.get_template_store_mut();
        store.store(template, &addr, &hash);

        Ok(addr)
    }

    #[must_use]
    fn store_app(&mut self, app: &App) -> Result<Address, StoreError> {
        match self.template_exists(&app.template) {
            false => {
                // important:
                // Normally code shuld never execute these piece.
                // The Runtime (defined at the `svm-runtime` crate) was supposed to pre-validate the existence
                // of the `AppTemplate` prior to calling the `Env` for storing the new `App`.
                let msg = format!(
                    "`AppTemplate` not found (address = `{:?}`)",
                    app.template.clone()
                );
                let err = StoreError::DataCorruption(msg);
                Err(err)
            }
            true => {
                let addr = self.derive_app_address(&app);
                let store = self.get_app_store_mut();
                store.store(app, &addr)?;

                Ok(addr)
            }
        }
    }

    fn load_template_by_app(&self, app_addr: &Address) -> Option<(AppTemplate, Address)> {
        if let Some(app) = self.load_app(app_addr) {
            if let Some(template) = self.load_template(&app.template) {
                return Some((template, app.template));
            }
        }

        None
    }

    fn load_template(&self, template_addr: &Address) -> Option<AppTemplate> {
        let store = self.get_template_store();
        store.load(&template_addr)
    }

    fn load_app(&self, app_addr: &Address) -> Option<App> {
        let store = self.get_app_store();
        store.load(&app_addr)
    }

    fn validate_template(&self, template: &AppTemplate) -> Result<(), String> {
        todo!();

        Ok(())
    }

    fn validate_app(&self, app: &App) -> Result<(), String> {
        todo!();

        // let template = self.load_template(&app.template);
        //
        // match template {
        //     Some(..) => Ok(()),
        //     None => {
        //         let err = format!("Template `{:?}` doens't exist", app.template);
        //         Err(err)
        //     }
        // }
    }

    fn validate_app_tx(&self, tx: &AppTransaction) -> Result<(), String> {
        let app = self.load_app(&tx.app);

        match app {
            Some(..) => Ok(()),
            None => {
                let err = format!("App `{:?}` doesn't exist", tx.app);
                Err(err)
            }
        }
    }

    #[inline(always)]
    fn template_exists(&self, template_addr: &Address) -> bool {
        self.load_template(template_addr).is_some()
    }

    #[inline(always)]
    fn app_exists(&self, app_addr: &Address) -> bool {
        self.load_app(app_addr).is_some()
    }
}
