use std::io::Cursor;

use svm_codec::template;
use svm_codec::{Field, ReadExt, WriteExt};

use svm_types::{AuthorAddr, Template};

use crate::env::traits;
use crate::env::ExtTemplate;

use traits::{TemplateDeserializer, TemplateSerializer};

/// `Template` default Serializer
pub struct DefaultTemplateSerializer;

/// `Template` default Deserializer
pub struct DefaultTemplateDeserializer;

impl TemplateSerializer for DefaultTemplateSerializer {
    fn serialize(template: &ExtTemplate) -> Vec<u8> {
        let mut w = Vec::new();

        let base = template.base();
        let author = template.author();

        template::encode_deploy_template(base, &mut w);

        w.write_address(author.inner());

        w
    }
}

impl TemplateDeserializer for DefaultTemplateDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<ExtTemplate> {
        let mut cursor = Cursor::new(bytes);

        let base = match template::decode_deploy_template(&mut cursor) {
            Ok(base) => base,
            _ => return None,
        };

        let author = match cursor.read_address() {
            Ok(addr) => AuthorAddr::new(addr.into()),
            _ => return None,
        };

        let template = ExtTemplate::new(base, &author);

        Some(template)
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
    fn serialize_template() {
        let base = Template {
            version: 0,
            name: "My Template".to_string(),
            code: vec![0x0C, 0x00, 0x0D, 0x0E],
            layout: vec![10, 20, 30].into(),
            ctors: vec!["init".into(), "start".into()],
        };

        let author = Address::of("@author").into();
        let template = ExtTemplate::new(base, &author);

        let bytes = S::serialize(&template);

        let decoded = D::deserialize(&bytes[..]).unwrap();
        assert_eq!(decoded, template);
    }
}
