use std::collections::HashSet;
use std::io::Cursor;

use svm_codec::{template, Codec, ReadExt};
use svm_types::{Account, SectionKind, Template, TemplateAddr};

use crate::env::{traits, ExtAccount};
use traits::{AccountDeserializer, AccountSerializer, TemplateDeserializer, TemplateSerializer};

/// Default serializer for an [`Account`]
pub struct DefaultAccountSerializer;

/// Default deserializer for an [`Account`]
pub struct DefaultAccountDeserializer;

impl AccountSerializer for DefaultAccountSerializer {
    fn serialize(account: &ExtAccount) -> Vec<u8> {
        let mut w = Vec::new();

        account.template_addr().0.encode(&mut w);
        account.name().to_string().encode(&mut w);
        account.spawner().0.encode(&mut w);

        w
    }
}

impl AccountDeserializer for DefaultAccountDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<ExtAccount> {
        let mut reader = Cursor::new(bytes);

        let template = reader.read_bytes_prim().ok()?;
        let name = String::decode(&mut reader).ok()?;
        let spawner = reader.read_bytes_prim().ok()?;

        let base = Account::new(template, name);
        let account = ExtAccount::new(&base, &spawner);

        Some(account)
    }

    fn deserialize_template_addr(bytes: &[u8]) -> Option<TemplateAddr> {
        let mut cursor = Cursor::new(bytes);

        cursor.read_bytes_prim().ok()
    }
}

/// [`Template`] default Serializer
pub struct DefaultTemplateSerializer;

/// [`Template`] default Deserializer
pub struct DefaultTemplateDeserializer;

impl TemplateSerializer for DefaultTemplateSerializer {
    fn serialize(template: &Template) -> Vec<u8> {
        template::encode(template)
    }
}

impl TemplateDeserializer for DefaultTemplateDeserializer {
    fn deserialize(bytes: &[u8], interests: Option<HashSet<SectionKind>>) -> Option<Template> {
        let cursor = Cursor::new(bytes);
        let template = template::decode(cursor, interests);

        template.ok()
    }
}
