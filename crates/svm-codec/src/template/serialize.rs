use std::io::Cursor;

use svm_types::{AppTemplate, AuthorAddr};

use crate::api::raw;
use crate::serialize::{AppTemplateDeserializer, AppTemplateSerializer};
use crate::{Field, ReadExt, WriteExt};

/// `AppTemplate` default Serializer
pub struct DefaultAppTemplateSerializer;

/// `AppTemplate` default Deserializer
pub struct DefaultAppTemplateDeserializer;

impl AppTemplateSerializer for DefaultAppTemplateSerializer {
    fn serialize(template: &AppTemplate, author: &AuthorAddr) -> Vec<u8> {
        let mut w = Vec::new();

        raw::encode_deploy_template(template, &mut w);

        w.write_address(author.inner());

        w
    }
}

impl AppTemplateDeserializer for DefaultAppTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<(AppTemplate, AuthorAddr)> {
        let mut cursor = Cursor::new(bytes);

        let template = match raw::decode_deploy_template(&mut cursor) {
            Ok(template) => template,
            _ => return None,
        };

        let author = match cursor.read_address() {
            Ok(addr) => AuthorAddr::new(addr.into()),
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
