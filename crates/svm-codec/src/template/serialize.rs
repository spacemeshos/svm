use std::io::Cursor;

use svm_types::{AppTemplate, AuthorAddr};

use crate::api::raw::{decode_deploy_template, encode_deploy_template, Field};

use crate::{
    common,
    serialize::{AppTemplateDeserializer, AppTemplateSerializer},
};

/// `AppTemplate` default Serializer
pub struct DefaultAppTemplateSerializer;

/// `AppTemplate` default Deserializer
pub struct DefaultAppTemplateDeserializer;

impl AppTemplateSerializer for DefaultAppTemplateSerializer {
    fn serialize(template: &AppTemplate, author: &AuthorAddr) -> Vec<u8> {
        let mut w = Vec::new();

        encode_deploy_template(template, &mut w);
        common::encode_address(author.inner(), &mut w);

        w
    }
}

impl AppTemplateDeserializer for DefaultAppTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<(AppTemplate, AuthorAddr)> {
        let mut cursor = Cursor::new(bytes);

        let template = match decode_deploy_template(&mut cursor) {
            Ok(template) => template,
            _ => return None,
        };

        let author = match common::decode_address(&mut cursor, Field::Author) {
            Ok(addr) => AuthorAddr::new(addr),
            _ => return None,
        };

        Some((template, author))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::Address;

    use DefaultAppTemplateDeserializer as D;
    use DefaultAppTemplateSerializer as S;

    #[test]
    fn serialize_deploy_template() {
        let template = AppTemplate {
            version: 0,
            name: "My Template".to_string(),
            code: vec![0x0C, 0x00, 0x0D, 0x0E],
            data: vec![10, 20, 30].into(),
        };

        let author = Address::of("@author").into();
        let bytes = S::serialize(&template, &author);

        let decoded = D::deserialize(&bytes[..]).unwrap();
        assert_eq!((template, author), decoded);
    }
}
