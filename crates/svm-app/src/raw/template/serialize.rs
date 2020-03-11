use crate::{
    raw::{helpers, NibbleWriter},
    traits::{AppTemplateDeserializer, AppTemplateSerializer},
    types::AppTemplate,
};

use svm_common::Address;

use super::wire;

/// `AppTemplate` default Serializer
pub struct DefaultAppTemplateSerializer;

/// `AppTemplate` default Deserializer
pub struct DefaultAppTemplateDeserializer;

impl AppTemplateSerializer for DefaultAppTemplateSerializer {
    fn serialize(template: &AppTemplate, author: &Address) -> Vec<u8> {
        todo!()
    }
}

impl AppTemplateDeserializer for DefaultAppTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<(AppTemplate, Address)> {
        todo!()
    }
}
