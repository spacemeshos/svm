use crate::{
    raw::{helpers, NibbleWriter},
    testing::DeployAppTemplateBuilder,
    traits::{AppTemplateDeserializer, AppTemplateSerializer},
    types::AppTemplate,
};

use super::wire;

/// `AppTemplate` default Serializer
pub struct DefaultAppTemplateSerializer;

/// `AppTemplate` default Deserializer
pub struct DefaultAppTemplateDeserializer;

impl AppTemplateSerializer for DefaultAppTemplateSerializer {
    fn serialize(deploy_template: &AppTemplate) -> Vec<u8> {
        todo!()
    }
}

impl DefaultAppTemplateSerializer {}

impl AppTemplateDeserializer for DefaultAppTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<AppTemplate> {
        todo!()
    }
}
