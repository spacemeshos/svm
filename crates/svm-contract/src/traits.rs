use crate::{types::CodeHash, wasm::AppTemplate};

use svm_common::Address;

/// Serializing an `AppTemplate` into its raw representation trait.
pub trait AppTemplateSerializer {
    #[allow(missing_docs)]
    fn serialize(template: &AppTemplate) -> Vec<u8>;
}

/// Deserializing rawn `AppTemplate` into its in-memory representation trait.
pub trait AppTemplateDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: Vec<u8>) -> AppTemplate;
}

/// A persistent store for `AppTemplate`
pub trait AppTemplateStore<S, D>
where
    S: AppTemplateSerializer,
    D: AppTemplateDeserializer,
{
    /// Stores the `hash` -> `raw contract` association
    fn store(&mut self, template: &AppTemplate, address: &Address, hash: CodeHash);

    /// Given a `AppTemplate` account address, fetches its raw data
    /// and deserializes it. Returns `None` if `AppTemplatee` doesn't exist.
    fn load(&self, address: &Address) -> Option<AppTemplate>;
}

/// Computes an `AppTemplate` account address.
/// Algorithm must be deterministic.
pub trait AppTemplateAddressCompute {
    /// Derives the `AppTemplate` address
    fn compute(template: &AppTemplate) -> Address;
}

/// Computes Hash derived deterministically from raw `AppTemplate`.
pub trait AppTemplateHasher {
    /// Given code as bytes, derives an Hash
    fn hash(bytes: &[u8]) -> CodeHash;
}
