use std::collections::HashSet;
use std::io::Cursor;

use svm_codec::{ReadExt, WriteExt};

use svm_codec::template;
use svm_types::{Account, SectionKind, Template, TemplateAddr};

use crate::env::{self, traits};

use env::ExtAccount;

use traits::{AccountDeserializer, AccountSerializer};
use traits::{TemplateDeserializer, TemplateSerializer};

/// Default serializer for an [`Account`]
pub struct DefaultAccountSerializer;

/// Default deserializer for an [`Account`]
pub struct DefaultAccountDeserializer;

impl AccountSerializer for DefaultAccountSerializer {
    fn serialize(account: &ExtAccount) -> Vec<u8> {
        let mut w = Vec::new();

        w.write_bytes_prim(account.template_addr());
        w.write_string(account.name());
        w.write_bytes_prim(account.spawner());

        w
    }
}

impl AccountDeserializer for DefaultAccountDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<ExtAccount> {
        let mut cursor = Cursor::new(bytes);

        let template = cursor.read_bytes_prim().ok()?;
        let name = cursor.read_string().ok()?.ok()?;
        let spawner = cursor.read_bytes_prim().ok()?;

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
