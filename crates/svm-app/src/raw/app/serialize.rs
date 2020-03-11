use crate::{
    raw::{helpers, Field, NibbleIter, NibbleWriter},
    traits::{AppDeserializer, AppSerializer},
    types::App,
};

use svm_common::Address;

/// Default serializer for `App`
pub struct DefaultAppSerializer;

/// Default deserializer for `App`
pub struct DefaultAppDeserializer;

impl AppSerializer for DefaultAppSerializer {
    fn serialize(app: &App, creator: &Address) -> Vec<u8> {
        let mut w = NibbleWriter::new();

        Self::encode_version(app, &mut w);
        Self::encode_template(app, &mut w);
        Self::encode_creator(creator, &mut w);

        helpers::bytes(&mut w)
    }
}

impl DefaultAppSerializer {
    fn encode_version(app: &App, w: &mut NibbleWriter) {
        helpers::encode_version(*&app.version, w);
    }

    fn encode_template(app: &App, w: &mut NibbleWriter) {
        helpers::encode_address(&app.template, w);
    }

    fn encode_creator(creator: &Address, w: &mut NibbleWriter) {
        helpers::encode_address(creator, w);
    }
}

impl AppDeserializer for DefaultAppDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<(App, Address)> {
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

        let app = App { version, template };

        Some((app, creator))
    }
}
