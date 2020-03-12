use crate::{
    raw::{helpers, Field, NibbleIter, NibbleWriter},
    traits::{AppDeserializer, AppSerializer},
    types::{App, CreatorAddr, TemplateAddr},
};

use svm_common::Address;

/// Default serializer for `App`
pub struct DefaultAppSerializer;

/// Default deserializer for `App`
pub struct DefaultAppDeserializer;

impl AppSerializer for DefaultAppSerializer {
    fn serialize(app: &App, creator: &CreatorAddr) -> Vec<u8> {
        let mut w = NibbleWriter::new();

        Self::encode_version(app, &mut w);
        Self::encode_template(app, &mut w);
        Self::encode_creator(creator, &mut w);

        w.into_bytes()
    }
}

impl DefaultAppSerializer {
    fn encode_version(app: &App, w: &mut NibbleWriter) {
        helpers::encode_version(*&app.version, w);
    }

    fn encode_template(app: &App, w: &mut NibbleWriter) {
        helpers::encode_address(app.template.inner(), w);
    }

    fn encode_creator(creator: &CreatorAddr, w: &mut NibbleWriter) {
        helpers::encode_address(creator.inner(), w);
    }
}

impl AppDeserializer for DefaultAppDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<(App, CreatorAddr)> {
        let mut iter = NibbleIter::new(bytes);

        let version = match helpers::decode_version(&mut iter) {
            Ok(ver) => ver,
            _ => return None,
        };

        let template = match helpers::decode_address(&mut iter, Field::AppTemplate) {
            Ok(addr) => TemplateAddr::new(addr),
            _ => return None,
        };

        let creator = match helpers::decode_address(&mut iter, Field::Creator) {
            Ok(addr) => CreatorAddr::new(addr),
            _ => return None,
        };

        let app = App { version, template };

        Some((app, creator))
    }
}
