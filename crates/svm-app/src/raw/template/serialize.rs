use crate::{
    raw::{helpers, NibbleWriter},
    testing::DeployAppTemplateBuilder,
    traits::{AppTemplateDeserializer, AppTemplateSerializer},
    types::AppTemplate,
};

/// `AppTemplate` default Serializer
pub struct DefaultAppTemplateSerializer;

/// `AppTemplate` default Deserializer
pub struct DefaultAppTemplateDeserializer;

impl AppTemplateSerializer for DefaultAppTemplateSerializer {
    fn serialize(template: &AppTemplate) -> Vec<u8> {
        let mut w = NibbleWriter::new();

        Self::write_version(template, &mut w);
        Self::write_name(template, &mut w);
        Self::write_page_count(template, &mut w);
        Self::write_code(template, &mut w);

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
}

impl AppTemplateDeserializer for DefaultAppTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<AppTemplate> {
        crate::raw::parse_template(bytes).ok()
    }
}
