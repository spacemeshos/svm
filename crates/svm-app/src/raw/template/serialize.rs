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
        let s = serde_json::to_string(template).unwrap();
        s.into_bytes()
    }
}

impl AppTemplateDeserializer for AppTemplateJsonDeserializer {
    fn deserialize(bytes: Vec<u8>) -> Option<AppTemplate> {
        let s = unsafe { String::from_utf8_unchecked(bytes) };

        serde_json::from_str(s.as_str()).ok()
    }
}
