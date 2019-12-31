use crate::traits::{
    AppTemplateAddressCompute, AppTemplateDeserializer, AppTemplateHasher, AppTemplateSerializer,
    AppTemplateStore,
};
use crate::{
    transaction::Transaction,
    types::CodeHash,
    wasm::AppTemplate,
    wire::{deploy::AppTemplateBuildError, exec::TransactionBuildError},
};

use svm_common::Address;

/// Aggregates types that are required by `AppTemplateEnv`
pub trait AppTemplateEnvTypes {
    /// Serializer for `AppTemplate`
    type Serializer: AppTemplateSerializer;

    /// Deserializer a `AppTemplate`
    type Deserializer: AppTemplateDeserializer;

    /// Storing / Loading an `AppTemplate`
    type Store: AppTemplateStore<Self::Serializer, Self::Deserializer>;

    /// Deriving the `AppTemplate` address
    type AddressCompute: AppTemplateAddressCompute;

    /// Deriving the Hash of the `AppTemplate` code
    type Hasher: AppTemplateHasher;
}

/// A trait for managing an `AppTemplate` environment.
/// Relies on associated `AppTemplateEnvTypes`.
pub trait AppTemplateEnv {
    /// AppTemplate environment is dictated by its `Types`
    type Types: AppTemplateEnvTypes;

    /// Borrows environment's store
    fn get_store(&self) -> &<Self::Types as AppTemplateEnvTypes>::Store;

    /// Borrows mutably environment's store
    fn get_store_mut(&mut self) -> &mut <Self::Types as AppTemplateEnvTypes>::Store;

    /// Computes `AppTemplate` Hash
    #[inline(always)]
    fn compute_hash(&self, template: &AppTemplate) -> CodeHash {
        <Self::Types as AppTemplateEnvTypes>::Hasher::hash(&template.code)
    }

    /// Computes `AppTemplate` account address
    #[inline(always)]
    fn compute_address(&self, template: &AppTemplate) -> Address {
        <Self::Types as AppTemplateEnvTypes>::AddressCompute::compute(template)
    }

    /// * Parses a raw template into `AppTemplate`
    /// * Enriches the template with its derived address
    fn parse_template(&self, bytes: &[u8]) -> Result<AppTemplate, AppTemplateBuildError> {
        let template = crate::wire::deploy::parse_template(bytes)?;

        crate::wire::deploy::validate_contract(&template)?;

        Ok(template)
    }

    /// Parses a raw transaction
    fn build_transaction(&self, bytes: &[u8]) -> Result<Transaction, TransactionBuildError> {
        let tx = crate::wire::exec::parse_transaction(bytes)?;

        Ok(tx)
    }

    /// Stores template by its `CodeHash`
    #[inline(always)]
    fn store_template(&mut self, template: &AppTemplate, addr: &Address) {
        let hash = self.compute_hash(template);
        let store = self.get_store_mut();

        store.store(template, addr, hash);
    }
}
