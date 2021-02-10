use std::io::Cursor;

use svm_types::{AuthorAddr, Template};

use crate::serialize::{TemplateDeserializer, TemplateSerializer};
use crate::template;
use crate::{Field, ReadExt, WriteExt};

/// `Template` default Serializer
pub struct DefaultTemplateSerializer;

/// `Template` default Deserializer
pub struct DefaultTemplateDeserializer;

impl TemplateSerializer for DefaultTemplateSerializer {
    fn serialize(template: &Template, author: &AuthorAddr) -> Vec<u8> {
        let mut w = Vec::new();

        template::encode_deploy_template(template, &mut w);

        w.write_address(author.inner());

        w
    }
}

impl TemplateDeserializer for DefaultTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<(Template, AuthorAddr)> {
        let mut cursor = Cursor::new(bytes);

        let template = match template::decode_deploy_template(&mut cursor) {
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
    use std::vec;

    use super::*;

    use svm_types::Address;

    use DefaultTemplateDeserializer as D;
    use DefaultTemplateSerializer as S;

    #[test]
    fn serialize_deploy_template() {
        let template = Template {
            version: 0,
            name: "My Template".to_string(),
            code: vec![0x0C, 0x00, 0x0D, 0x0E],
            data: vec![10, 20, 30].into(),
            ctors: vec!["init".into(), "start".into()],
        };

        let author = Address::of("@author").into();
        let bytes = S::serialize(&template, &author);

        let decoded = D::deserialize(&bytes[..]).unwrap();
        assert_eq!((template, author), decoded);
    }
}
