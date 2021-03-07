use std::io::Cursor;

use svm_codec::{ReadExt, WriteExt};
use svm_types::{App, SpawnerAddr, TemplateAddr};

use crate::env::{traits, ExtApp};

use traits::{AppDeserializer, AppSerializer};

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
