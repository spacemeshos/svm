use std::collections::HashSet;

use svm_types::{SectionKind, Template, TemplateAddr};

/// `Env` storage serialization types
use crate::env::ExtAccount;

/// Serializing an `Template` into its raw representation.
pub trait TemplateSerializer {
    #[allow(missing_docs)]
    fn serialize(template: &Template) -> Vec<u8>;
}

/// Deserializing stored `Template` into its in-memory representation.
pub trait TemplateDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8], interests: Option<HashSet<SectionKind>>) -> Option<Template>;
}

/// Serializing an `Account` into its binary representation.
pub trait AccountSerializer {
    #[allow(missing_docs)]
    fn serialize(account: &ExtAccount) -> Vec<u8>;
}

/// Deserializing stored `Account` into its in-memory representation.
pub trait AccountDeserializer {
    #[allow(missing_docs)]
    fn deserialize(bytes: &[u8]) -> Option<ExtAccount>;

    fn deserialize_template_addr(bytes: &[u8]) -> Option<TemplateAddr> {
        Self::deserialize(bytes).map(|account| account.template_addr().clone())
    }
}

pub trait EnvSerializers {
    /// `Template`'s Serializer
    type TemplateSerializer: TemplateSerializer;

    /// `Template`'s Deserializer
    type TemplateDeserializer: TemplateDeserializer;

    /// `Account`'s Serializer
    type AccountSerializer: AccountSerializer;

    /// `Account`'s Deserializer
    type AccountDeserializer: AccountDeserializer;
}
