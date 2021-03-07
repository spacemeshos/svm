use svm_types::TemplateAddr;

/// `Env` storage serialization types
use crate::env::{ExtApp, ExtTemplate};

/// Serializing an `Template` into its raw representation.
pub trait TemplateSerializer {
    #[allow(missing_docs)]
    fn serialize(template: &ExtTemplate) -> Vec<u8>;
}

/// Deserializing stored `Template` into its in-memory representation.
pub trait TemplateDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8]) -> Option<ExtTemplate>;
}

/// Serializing an `App` into its raw representation.
pub trait AppSerializer {
    #[allow(missing_docs)]
    fn serialize(app: &ExtApp) -> Vec<u8>;
}

/// Deserializing stored `App` into its in-memory representation.
pub trait AppDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8]) -> Option<ExtApp>;

    fn desrialize_template_addr(bytes: &[u8]) -> Option<TemplateAddr> {
        Self::deserialize(bytes).map(|app| app.template_addr().clone())
    }
}

pub trait EnvSerializers {
    /// `Template`'s Serializer
    type TemplateSerializer: TemplateSerializer;

    /// `Template`'s Deserializer
    type TemplateDeserializer: TemplateDeserializer;

    /// `App`'s Serializer
    type AppSerializer: AppSerializer;

    /// `App`'s Deserializer
    type AppDeserializer: AppDeserializer;
}
