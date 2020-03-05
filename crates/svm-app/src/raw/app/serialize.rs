use crate::{
    raw::{helpers, Field, NibbleIter, NibbleWriter},
    traits::{AppDeserializer, AppSerializer},
    types::App,
};

/// Default serializer for `App`
pub struct DefaultAppSerializer;

/// Default deserializer for `App`
pub struct DefaultAppDeserializer;

impl AppSerializer for DefaultAppSerializer {
    fn serialize(app: &App) -> Vec<u8> {
        let mut w = NibbleWriter::new();

        Self::write_version(app, &mut w);
        Self::write_template(app, &mut w);
        Self::write_creator(app, &mut w);

        helpers::bytes(&mut w)
    }
}

impl DefaultAppSerializer {
    fn write_version(app: &App, w: &mut NibbleWriter) {
        helpers::encode_version(*&app.version, w);
    }

    fn write_template(app: &App, w: &mut NibbleWriter) {
        helpers::encode_address(&app.template, w);
    }

    fn write_creator(app: &App, w: &mut NibbleWriter) {
        helpers::encode_address(&app.creator, w);
    }
}

impl AppDeserializer for DefaultAppDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<App> {
        let mut iter = NibbleIter::new(bytes);

        let version = match helpers::decode_version(&mut iter) {
            Ok(ver) => ver,
            _ => return None,
        };

        let template = match helpers::decode_address(&mut iter, Field::AppTemplate) {
            Ok(addr) => addr,
            _ => return None,
        };

        let creator = match helpers::decode_address(&mut iter, Field::Creator) {
            Ok(addr) => addr,
            _ => return None,
        };

        let app = App {
            version,
            template,
            creator,
        };

        Some(app)
    }
}
