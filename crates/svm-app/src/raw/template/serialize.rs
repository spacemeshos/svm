use crate::{
    raw::{
        decode_deploy_template_iter, encode_deploy_template, helpers, Field, NibbleIter,
        NibbleWriter,
    },
    traits::{AppTemplateDeserializer, AppTemplateSerializer},
    types::{AppTemplate, AuthorAddr},
};

use svm_common::Address;

use super::wire;

/// `AppTemplate` default Serializer
pub struct DefaultAppTemplateSerializer;

/// `AppTemplate` default Deserializer
pub struct DefaultAppTemplateDeserializer;

impl AppTemplateSerializer for DefaultAppTemplateSerializer {
    fn serialize(template: &AppTemplate, author: &AuthorAddr) -> Vec<u8> {
        let bytes = encode_deploy_template(template);

        let mut w = NibbleWriter::new();
        w.write_bytes(&bytes[..]);

        helpers::encode_address(author.inner(), &mut w);

        helpers::bytes(&mut w)
    }
}

impl AppTemplateDeserializer for DefaultAppTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<(AppTemplate, AuthorAddr)> {
        let mut iter = NibbleIter::new(bytes);

        let template = match decode_deploy_template_iter(&mut iter) {
            Ok(template) => template,
            _ => return None,
        };

        let author = match helpers::decode_address(&mut iter, Field::Author) {
            Ok(addr) => AuthorAddr::new(addr),
            _ => return None,
        };

        Some((template, author))
    }
}
