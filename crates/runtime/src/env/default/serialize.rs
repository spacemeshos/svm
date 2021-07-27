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

        encode_template(account, &mut w);
        encode_name(account, &mut w);
        encode_spawner(account, &mut w);

        w
    }
}

fn encode_template(account: &ExtAccount, w: &mut Vec<u8>) {
    let addr = account.template_addr();
    w.write_template_addr(addr);
}

fn encode_name(account: &ExtAccount, w: &mut Vec<u8>) {
    w.write_string(account.name());
}

fn encode_spawner(account: &ExtAccount, w: &mut Vec<u8>) {
    let spawner = account.spawner();
    w.write_address(spawner);
}

impl AccountDeserializer for DefaultAccountDeserializer {
    fn deserialize(bytes: &[u8]) -> Option<ExtAccount> {
        let mut cursor = Cursor::new(bytes);

        let template = match cursor.read_template_addr() {
            Ok(addr) => addr,
            _ => return None,
        };

        let name = match cursor.read_string() {
            Ok(Ok(name)) => name,
            _ => return None,
        };

        let spawner = match cursor.read_address() {
            Ok(addr) => addr,
            _ => return None,
        };

        let base = Account::new(template, name);
        let account = ExtAccount::new(&base, &spawner);

        Some(account)
    }

    fn deserialize_template_addr(bytes: &[u8]) -> Option<TemplateAddr> {
        let mut cursor = Cursor::new(bytes);

        cursor.read_template_addr().ok()
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
