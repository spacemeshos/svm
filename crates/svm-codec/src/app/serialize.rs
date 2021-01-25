use std::io::Cursor;

use svm_nibble::NibbleIter;
use svm_types::{App, CreatorAddr, TemplateAddr};

use crate::api::raw::{decode_version, encode_version, Field};

use crate::{
    helpers,
    traits::{AppDeserializer, AppSerializer},
};

/// Default serializer for `App`
pub struct DefaultAppSerializer;

/// Default deserializer for `App`
pub struct DefaultAppDeserializer;

impl AppSerializer for DefaultAppSerializer {
    fn serialize(app: &App, creator: &CreatorAddr) -> Vec<u8> {
        let mut w = Vec::new();

        encode_version(app.version, &mut w);
        Self::encode_template(app, &mut w);
        Self::encode_creator(creator, &mut w);
        Self::encode_name(app, &mut w);

        w
    }
}

impl DefaultAppSerializer {
    fn encode_template(app: &App, w: &mut Vec<u8>) {
        helpers::encode_address(app.template.inner(), w);
    }

    fn encode_creator(creator: &CreatorAddr, w: &mut Vec<u8>) {
        helpers::encode_address(creator.inner(), w);
    }

    fn encode_name(app: &App, w: &mut Vec<u8>) {
        helpers::encode_string(&app.name, w);
    }
}

impl AppDeserializer for DefaultAppDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<(App, CreatorAddr)> {
        let mut cursor = Cursor::new(bytes);

        let version = match decode_version(&mut cursor) {
            Ok(ver) => ver,
            _ => return None,
        };

        let template = match helpers::decode_address(&mut cursor, Field::TemplateAddr) {
            Ok(addr) => TemplateAddr::new(addr),
            _ => return None,
        };

        let creator = match helpers::decode_address(&mut cursor, Field::Creator) {
            Ok(addr) => CreatorAddr::new(addr),
            _ => return None,
        };

        let name = match helpers::decode_string(&mut cursor, Field::NameLength, Field::Name) {
            Ok(name) => name,
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
