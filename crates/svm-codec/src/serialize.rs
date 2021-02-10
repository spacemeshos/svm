use svm_types::{App, AuthorAddr, CreatorAddr, Template};

/// Serializing an `Template` into its raw representation.
pub trait TemplateSerializer {
    #[allow(missing_docs)]
    fn serialize(template: &Template, author: &AuthorAddr) -> Vec<u8>;
}

/// Deserializing stored `Template` into its in-memory representation.
pub trait TemplateDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8]) -> Option<(Template, AuthorAddr)>;
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
