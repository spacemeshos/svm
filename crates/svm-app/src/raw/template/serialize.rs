use crate::{
    raw::{helpers, NibbleWriter},
    testing::DeployAppTemplateBuilder,
    traits::{AppTemplateDeserializer, AppTemplateSerializer},
    types::{AppTemplate, DeployAppTemplate},
};

/// `AppTemplate` default Serializer
pub struct DefaultAppTemplateSerializer;

/// `AppTemplate` default Deserializer
pub struct DefaultAppTemplateDeserializer;

impl AppTemplateSerializer for DefaultAppTemplateSerializer {
    fn serialize(deploy_template: &DeployAppTemplate) -> Vec<u8> {
        let mut w = NibbleWriter::new();

        let template = &deploy_template.template;

        Self::write_version(template, &mut w);
        Self::write_name(template, &mut w);
        Self::write_page_count(template, &mut w);
        Self::write_code(template, &mut w);
        Self::write_author(deploy_template, &mut w);

        helpers::bytes(&mut w)
    }
}

impl DefaultAppTemplateSerializer {
    fn write_version(template: &AppTemplate, w: &mut NibbleWriter) {
        helpers::encode_version(*&template.version, w);
    }

    fn write_name(template: &AppTemplate, w: &mut NibbleWriter) {
        helpers::encode_string(&template.name, w);
    }

    fn write_page_count(template: &AppTemplate, w: &mut NibbleWriter) {
        helpers::encode_varuint14(template.page_count, w);
    }

    fn write_code(template: &AppTemplate, w: &mut NibbleWriter) {
        w.write_bytes(&template.code[..])
    }

    fn write_author(template: &DeployAppTemplate, w: &mut NibbleWriter) {
        helpers::encode_address(&template.author, w);
    }
}

impl AppTemplateDeserializer for DefaultAppTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<AppTemplate> {
        todo!()
    }
}
