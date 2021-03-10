use std::io::Cursor;

use svm_codec::{ReadExt, WriteExt};

use svm_codec::template;
use svm_types::{App, AuthorAddr, SpawnerAddr, TemplateAddr};

use crate::env::{self, traits};

use env::{ExtApp, ExtTemplate};

use traits::{AppDeserializer, AppSerializer};
use traits::{TemplateDeserializer, TemplateSerializer};

/// Default serializer for `App`
pub struct DefaultAppSerializer;

/// Default deserializer for `App`
pub struct DefaultAppDeserializer;

impl AppSerializer for DefaultAppSerializer {
    fn serialize(app: &ExtApp) -> Vec<u8> {
        let mut w = Vec::new();

        encode_template(app, &mut w);
        encode_name(app, &mut w);
        encode_spawner(app, &mut w);

        w
    }
}

fn encode_template(app: &ExtApp, w: &mut Vec<u8>) {
    let addr = app.template_addr();

    w.write_address(addr.inner());
}

fn encode_name(app: &ExtApp, w: &mut Vec<u8>) {
    w.write_string(app.name());
}

fn encode_spawner(app: &ExtApp, w: &mut Vec<u8>) {
    let spawner = app.spawner();

    w.write_address(spawner.inner());
}

impl AppDeserializer for DefaultAppDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<ExtApp> {
        let mut cursor = Cursor::new(bytes);

        let template = match cursor.read_address() {
            Ok(addr) => TemplateAddr::new(addr),
            _ => return None,
        };

        let name = match cursor.read_string() {
            Ok(Ok(name)) => name,
            _ => return None,
        };

        let spawner = match cursor.read_address() {
            Ok(addr) => SpawnerAddr::new(addr),
            _ => return None,
        };

        let base = App::new(template, name);
        let app = ExtApp::new(&base, &spawner);

        Some(app)
    }

    fn desrialize_template_addr(bytes: &[u8]) -> Option<TemplateAddr> {
        let mut cursor = Cursor::new(bytes);

        cursor.read_address().ok().map(|addr| addr.into())
    }
}

/// `Template` default Serializer
pub struct DefaultTemplateSerializer;

/// `Template` default Deserializer
pub struct DefaultTemplateDeserializer;

impl TemplateSerializer for DefaultTemplateSerializer {
    fn serialize(template: &ExtTemplate) -> Vec<u8> {
        let mut w = Vec::new();

        let base = template.base();
        let author = template.author();

        template::encode_deploy_template(base, &mut w);

        w.write_address(author.inner());

        w
    }
}

impl TemplateDeserializer for DefaultTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<ExtTemplate> {
        let mut cursor = Cursor::new(bytes);

        let base = match template::decode_deploy_template(&mut cursor) {
            Ok(base) => base,
            _ => return None,
        };

        let author = match cursor.read_address() {
            Ok(addr) => AuthorAddr::new(addr.into()),
            _ => return None,
        };

        let template = ExtTemplate::new(base, &author);

        Some(template)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    use svm_types::{Address, Template};

    use DefaultTemplateDeserializer as D;
    use DefaultTemplateSerializer as S;

    #[test]
    fn serialize_template() {
        let base = Template {
            version: 0,
            name: "My Template".to_string(),
            code: vec![0x0C, 0x00, 0x0D, 0x0E],
            layout: vec![10, 20, 30].into(),
            ctors: vec!["init".into(), "start".into()],
        };

        let author = Address::of("@author").into();
        let template = ExtTemplate::new(base, &author);

        let bytes = S::serialize(&template);

        let decoded = D::deserialize(&bytes[..]).unwrap();
        assert_eq!(decoded, template);
    }
}
