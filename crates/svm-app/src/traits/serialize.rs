use crate::types::{App, AppTemplate, AuthorAddr, CreatorAddr};

use svm_common::Address;

/// Serializing an `AppTemplate` into its raw representation.
pub trait AppTemplateSerializer {
    #[allow(missing_docs)]
    fn serialize(template: &AppTemplate, author: &AuthorAddr) -> Vec<u8>;
}

/// Deserializing stored `AppTemplate` into its in-memory representation.
pub trait AppTemplateDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8]) -> Option<(AppTemplate, AuthorAddr)>;
}

/// Serializing an `App` into its raw representation.
pub trait AppSerializer {
    #[allow(missing_docs)]
    fn serialize(app: &App, creator: &CreatorAddr) -> Vec<u8>;
}

/// Deserializing stored `App` into its in-memory representation.
pub trait AppDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8]) -> Option<(App, CreatorAddr)>;
}
