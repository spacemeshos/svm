//! Encoding of a binary [`SpawnAccount`].
//!
//! ```text
//!
//!  +-----------+-------------+----------------+
//!  |           |             |                |
//!  |  Version  |  Template   |      Name      |
//!  |   (u16)   |  (Address)  |    (String)    |
//!  |           |             |                |
//!  +-----------+-------------+----------------+
//!  |           |                              |
//!  |   Ctor    |          CallData            |
//!  |  (String) |           (Blob)             |
//!  |           |                              |
//!  +-----------+------------------------------+
//!
//! ```

use std::io::Cursor;

use svm_types::{Account, SpawnAccount, TemplateAddr};

use crate::{inputdata, version};
use crate::{Field, ParseError, ReadExt, WriteExt};

/// Encodes a binary [`SpawnAccount`] transaction.
pub fn encode(spawn: &SpawnAccount, w: &mut Vec<u8>) {
    version::encode_version(spawn.version, w);
    w.write_template_addr(spawn.template_addr());
    w.write_string(spawn.account_name());
    w.write_string(spawn.ctor_name());
    inputdata::encode_inputdata(&spawn.calldata, w);
}

/// Parsing a binary [`SpawnAccount`] transaction.
///
/// Returns the parsed [`SpawnAccount`],
/// On failure, returns [`ParseError`].
pub fn decode(cursor: &mut Cursor<&[u8]>) -> Result<SpawnAccount, ParseError> {
    let version = decode_version(cursor)?;
    let template_addr = decode_template(cursor)?;
    let name = decode_name(cursor)?;
    let ctor_name = decode_ctor(cursor)?;
    let calldata = decode_ctor_calldata(cursor)?;

    Ok(SpawnAccount {
        version,
        account: Account {
            name,
            template_addr,
        },
        ctor_name,
        calldata,
    })
}

/// Decoders

#[inline]
fn decode_version(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    version::decode_version(cursor)
}

fn decode_template(cursor: &mut Cursor<&[u8]>) -> Result<TemplateAddr, ParseError> {
    cursor
        .read_template_addr()
        .map_err(|_| ParseError::NotEnoughBytes(Field::Address))
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
    inputdata::decode_inputdata(cursor)
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::TemplateAddr;

    #[test]
    fn encode_decode_spawn() {
        let spawn = SpawnAccount {
            version: 0,
            account: Account {
                name: "@account".to_string(),
                template_addr: TemplateAddr::of("@template"),
            },
            ctor_name: "initialize".to_string(),
            calldata: vec![0x10, 0x20, 0x30],
        };

        let mut bytes = Vec::new();
        encode(&spawn, &mut bytes);

        let mut cursor = Cursor::new(&bytes[..]);
        let decoded = decode(&mut cursor).unwrap();

        assert_eq!(spawn, decoded);
    }
}
