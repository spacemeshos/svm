use crate::{
    raw::{
        decode_deploy_template, encode_deploy_template, helpers, Field, NibbleIter, NibbleWriter,
    },
    traits::{AppTemplateDeserializer, AppTemplateSerializer},
    types::{AppTemplate, AuthorAddr},
};

use svm_layout::DataLayout;

/// `AppTemplate` default Serializer
pub struct DefaultAppTemplateSerializer;

/// `AppTemplate` default Deserializer
pub struct DefaultAppTemplateDeserializer;

impl AppTemplateSerializer for DefaultAppTemplateSerializer {
    fn serialize(template: &AppTemplate, author: &AuthorAddr) -> Vec<u8> {
        let mut w = NibbleWriter::new();

        encode_deploy_template(template, &mut w);
        helpers::encode_address(author.inner(), &mut w);

        w.into_bytes()
    }
}

impl AppTemplateDeserializer for DefaultAppTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<(AppTemplate, AuthorAddr)> {
        let mut iter = NibbleIter::new(bytes);

        let template = match decode_deploy_template(&mut iter) {
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

#[cfg(test)]
mod tests {
    use super::*;

    use svm_common::Address;

    use DefaultAppTemplateDeserializer as D;
    use DefaultAppTemplateSerializer as S;

    #[test]
    fn serialize_deploy_template() {
        let template = AppTemplate {
            version: 0,
            name: "My Template".to_string(),
            page_count: 5,
            code: vec![0x0C, 0x00, 0x0D, 0x0E],
            data: vec![].into(),
        };

        let author = Address::of("@author").into();
        let bytes = S::serialize(&template, &author);

        let decoded = D::deserialize(&bytes[..]).unwrap();
        assert_eq!((template, author), decoded);
    }
}
