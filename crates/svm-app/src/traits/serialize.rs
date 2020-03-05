use crate::types::{App, AppTemplate, DeployAppTemplate, SpawnApp};

/// Serializing an `AppTemplate` into its raw representation.
pub trait AppTemplateSerializer {
    #[allow(missing_docs)]
    fn serialize(template: &DeployAppTemplate) -> Vec<u8>;
}

/// Deserializing stored `AppTemplate` into its in-memory representation.
pub trait AppTemplateDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8]) -> Option<AppTemplate>;
}

/// Serializing an `App` into its raw representation.
pub trait AppSerializer {
    #[allow(missing_docs)]
    fn serialize(app: &SpawnApp) -> Vec<u8>;
}

/// Deserializing stored `App` into its in-memory representation.
pub trait AppDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8]) -> Option<App>;
}
