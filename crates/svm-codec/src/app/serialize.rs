use std::io::Cursor;

use svm_types::{App, CreatorAddr, TemplateAddr};

use crate::serialize::{AppDeserializer, AppSerializer};
use crate::version;
use crate::{Field, ReadExt, WriteExt};

/// Default serializer for `App`
pub struct DefaultAppSerializer;

/// Default deserializer for `App`
pub struct DefaultAppDeserializer;

impl AppSerializer for DefaultAppSerializer {
    fn serialize(app: &App, creator: &CreatorAddr) -> Vec<u8> {
        let mut w = Vec::new();

        version::encode_version(app.version, &mut w);
        encode_template(app, &mut w);
        encode_creator(creator, &mut w);
        encode_name(app, &mut w);

        w
    }
}

fn encode_template(app: &App, w: &mut Vec<u8>) {
    let addr = app.template.inner();

    w.write_address(addr);
}

fn encode_creator(creator: &CreatorAddr, w: &mut Vec<u8>) {
    w.write_address(creator.inner());
}

fn encode_name(app: &App, w: &mut Vec<u8>) {
    w.write_string(&app.name);
}

impl AppDeserializer for DefaultAppDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<(App, CreatorAddr)> {
        let mut cursor = Cursor::new(bytes);

        let version = match version::decode_version(&mut cursor) {
            Ok(ver) => ver,
            _ => return None,
        };

        let template = match cursor.read_address() {
            Ok(addr) => TemplateAddr::new(addr),
            _ => return None,
        };

        let creator = match cursor.read_address() {
            Ok(addr) => CreatorAddr::new(addr),
            _ => return None,
        };

        let name = match cursor.read_string() {
            Ok(Ok(name)) => name,
            _ => return None,
        };

        let app = App {
            version,
            name,
            template,
        };

        Some((app, creator))
    }
}
