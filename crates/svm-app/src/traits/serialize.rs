use crate::types::{App, AppTemplate};

/// Serializing an `AppTemplate` into its raw representation.
pub trait AppTemplateSerializer {
    #[allow(missing_docs)]
    fn serialize(template: &AppTemplate) -> Vec<u8>;
}

/// Deserializing rawn `AppTemplate` into its in-memory representation.
pub trait AppTemplateDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8]) -> Option<AppTemplate>;
}

/// Serializing an `App` into its raw representation.
pub trait AppSerializer {
    #[allow(missing_docs)]
    fn serialize(app: &App) -> Vec<u8>;
}

/// Deserializing rawn `App` into its in-memory representation.
pub trait AppDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8]) -> Option<App>;
}
