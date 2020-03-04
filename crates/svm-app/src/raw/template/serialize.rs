use crate::{
    traits::{AppTemplateDeserializer, AppTemplateSerializer},
    types::AppTemplate,
};

/// `AppTemplate` json Serializer
pub struct AppTemplateJsonSerializer;

/// `AppTemplate` json Deserialize
pub struct AppTemplateJsonDeserializer;

impl AppTemplateSerializer for AppTemplateJsonSerializer {
    fn serialize(template: &AppTemplate) -> Vec<u8> {
        todo!()
    }
}

impl AppTemplateDeserializer for AppTemplateJsonDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<AppTemplate> {
        todo!()
    }
}
