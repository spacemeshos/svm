use std::io::Cursor;

use svm_types::{App, SpawnApp, TemplateAddr, WasmValue};

use crate::{calldata, common};
use crate::{Field, ParseError, ReadExt, WriteExt};

/// Encodes a raw Spawn-App transaction.
pub fn encode_spawn_app(spawn: &SpawnApp, w: &mut Vec<u8>) {
    encode_version(spawn, w);
    encode_template(spawn, w);
    encode_name(spawn, w);
    encode_ctor(spawn, w);
    encode_ctor_calldata(spawn, w);
}

/// Parsing a raw `spawn-app` transaction given as raw bytes.
/// Returns the parsed transaction as a tuple consisting of an `App` struct and `ctor_name` buffer args.
/// On failure, returns `ParseError`.
pub fn decode_spawn_app(cursor: &mut Cursor<&[u8]>) -> Result<SpawnApp, ParseError> {
    let version = decode_version(cursor)?;
    let template = decode_template(cursor)?;
    let name = decode_name(cursor)?;
    let ctor_name = decode_ctor(cursor)?;
    let calldata = decode_ctor_calldata(cursor)?;

    let app = App { name, template };

    let spawn = SpawnApp {
        version,
        app,
        ctor_name,
        calldata,
    };

    Ok(spawn)
}

/// Encoders

fn encode_version(spawn: &SpawnApp, w: &mut Vec<u8>) {
    let v = &spawn.version;

    common::encode_version(*v, w);
}

fn encode_name(spawn: &SpawnApp, w: &mut Vec<u8>) {
    let name = &spawn.app.name;

    w.write_string(name);
}

fn encode_template(spawn: &SpawnApp, w: &mut Vec<u8>) {
    let template = &spawn.app.template;

    w.write_address(template.inner());
}

fn encode_ctor(spawn: &SpawnApp, w: &mut Vec<u8>) {
    let ctor = &spawn.ctor_name;

    w.write_string(ctor);
}

fn encode_ctor_calldata(spawn: &SpawnApp, w: &mut Vec<u8>) {
    let calldata = &*spawn.calldata;

    calldata::encode_calldata(calldata, w);
}

/// Decoders

#[inline]
fn decode_version(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    common::decode_version(cursor)
}

fn decode_template(cursor: &mut Cursor<&[u8]>) -> Result<TemplateAddr, ParseError> {
    match cursor.read_address() {
        Ok(addr) => Ok(addr.into()),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Address)),
    }
}

fn decode_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(name)) => Ok(name),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Name)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Name)),
    }
}

fn decode_ctor(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(ctor)) => Ok(ctor),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Ctor)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Ctor)),
    }
}

fn decode_ctor_calldata(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ParseError> {
    calldata::decode_calldata(cursor)
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::Address;

    #[test]
    fn encode_decode_spawn_app() {
        let spawn = SpawnApp {
            version: 0,
            app: App {
                name: "my-app".to_string(),
                template: Address::of("my-template").into(),
            },
            ctor_name: "initialize".to_string(),
            calldata: vec![0x10, 0x20, 0x30],
        };

        let mut bytes = Vec::new();
        encode_spawn_app(&spawn, &mut bytes);

        let mut cursor = Cursor::new(&bytes[..]);

        let decoded = decode_spawn_app(&mut cursor).unwrap();

        assert_eq!(spawn, decoded);
    }
}
