//!  `Template` Raw Format
//!
//!  +_____________________________________________________+
//!  |            |                                        |
//!  |  version   |               name                     |
//!  |  (2 bytes) |             (String)                   |
//!  +____________|________________________________________+
//!  |               |                                     |
//!  |  Code #bytes  |          Code (WASM)                |
//!  |   (4 bytes)   |                                     |
//!  +_______________|_____________________________________+
//!  |               |             |         |             |
//!  |  Data-Layout  |  var #0     |         |   var #N    |
//!  |  #variables   |  length     |  . . .  |   length    |
//!  +_______________|_____________|_________|_____________+
//!
//!

use std::io::Cursor;

use svm_layout::{Layout, LayoutBuilder};
use svm_types::Template;

use crate::common;
use crate::{Field, ParseError, ReadExt, WriteExt};

/// Encodes a raw Deploy-Template.
pub fn encode_deploy_template(template: &Template, w: &mut Vec<u8>) {
    encode_version(template, w);
    encode_name(template, w);
    encode_code(template, w);
    encode_data(template, w);
    encode_ctors(template, w);
}

/// Decodes a raw Deploy-Template.
pub fn decode_deploy_template(cursor: &mut Cursor<&[u8]>) -> Result<Template, ParseError> {
    let version = decode_version(cursor)?;
    let name = decode_name(cursor)?;
    let code = decode_code(cursor)?;
    let data = decode_data(cursor)?;
    let ctors = decode_ctors(cursor)?;

    let template = Template {
        ctors,
        version,
        name,
        code,
        layout: data,
    };

    Ok(template)
}

/// Encoders

fn encode_version(template: &Template, w: &mut Vec<u8>) {
    let v = &template.version;

    common::encode_version(*v, w);
}

fn encode_name(template: &Template, w: &mut Vec<u8>) {
    w.write_string(&template.name);
}

fn encode_data(template: &Template, w: &mut Vec<u8>) {
    let nvars = template.layout.len();

    assert!(nvars < std::u16::MAX as usize);

    w.write_u16_be(nvars as u16);

    for var in template.layout.iter() {
        w.write_u16_be(var.byte_size() as u16);
    }
}

fn encode_code(template: &Template, w: &mut Vec<u8>) {
    let code = &template.code;

    // code length
    let length = code.len();
    assert!(length < std::u32::MAX as usize);

    w.write_u32_be(length as u32);

    // code
    w.write_bytes(code);
}

fn encode_ctors(template: &Template, w: &mut Vec<u8>) {
    let count = template.ctors.len();

    assert!(count < std::u8::MAX as usize);

    w.write_byte(count as u8);

    for ctor in template.ctors.iter() {
        w.write_string(ctor);
    }
}

/// Decoders

#[inline]
fn decode_version(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    common::decode_version(cursor)
}

fn decode_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(name)) => Ok(name),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Name)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Name)),
    }
}

fn decode_data(cursor: &mut Cursor<&[u8]>) -> Result<Layout, ParseError> {
    match cursor.read_u16_be() {
        Err(..) => Err(ParseError::NotEnoughBytes(Field::VarsCount)),
        Ok(nvars) => {
            let mut builder = LayoutBuilder::with_capacity(nvars as usize);

            for _vid in 0..nvars as usize {
                match cursor.read_u16_be() {
                    Err(..) => return Err(ParseError::NotEnoughBytes(Field::VarLength)),
                    Ok(length) => builder.add_var(length as u32),
                }
            }

            let layout = builder.build();
            Ok(layout)
        }
    }
}

fn decode_code(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ParseError> {
    match cursor.read_u32_be() {
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Code)),
        Ok(length) => cursor
            .read_bytes(length as usize)
            .map_err(|_| ParseError::NotEnoughBytes(Field::Code)),
    }
}

fn decode_ctors(cursor: &mut Cursor<&[u8]>) -> Result<Vec<String>, ParseError> {
    match cursor.read_byte() {
        Err(..) => Err(ParseError::NotEnoughBytes(Field::CtorsCount)),
        Ok(count) => {
            let mut ctors = Vec::with_capacity(count as usize);

            for _ in 0..count {
                if let Ok(Ok(ctor)) = cursor.read_string() {
                    ctors.push(ctor);
                } else {
                    return Err(ParseError::NotEnoughBytes(Field::Ctor));
                }
            }

            Ok(ctors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_deploy_template() {
        let template = Template {
            version: 0,
            name: "My Template".to_string(),
            code: vec![0x0C, 0x00, 0x0D, 0x0E],
            layout: vec![5, 10].into(),
            ctors: vec!["init".into(), "start".into()],
        };

        let mut bytes = Vec::new();
        encode_deploy_template(&template, &mut bytes);

        let mut cursor = Cursor::new(&bytes[..]);

        let decoded = decode_deploy_template(&mut cursor).unwrap();

        assert_eq!(template, decoded);
    }
}
