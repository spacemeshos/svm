use std::fs::File;
use std::io::{Cursor, Read};

use svm_layout::{DataLayout, DataLayoutBuilder};
use svm_types::AppTemplate;

use crate::api::raw;
use crate::common;
use crate::{Field, ParseError, ReadExt, WriteExt};

/// Encodes a raw Deploy-Template.
pub fn encode_deploy_template(template: &AppTemplate, w: &mut Vec<u8>) {
    encode_version(template, w);
    encode_name(template, w);
    encode_code(template, w);
    encode_data(template, w);
}

/// Decodes a raw Deploy-Template.
pub fn decode_deploy_template(cursor: &mut Cursor<&[u8]>) -> Result<AppTemplate, ParseError> {
    let version = raw::decode_version(cursor)?;
    let name = decode_name(cursor)?;
    let code = decode_code(cursor)?;
    let data = decode_data(cursor)?;

    let template = AppTemplate {
        version,
        name,
        code,
        data,
    };

    Ok(template)
}

/// Encoders

fn encode_version(template: &AppTemplate, w: &mut Vec<u8>) {
    let version = template.version;

    raw::encode_version(version, w);
}

fn encode_name(template: &AppTemplate, w: &mut Vec<u8>) {
    common::encode_string(&template.name, w);
}

fn encode_data(template: &AppTemplate, w: &mut Vec<u8>) {
    let nvars = template.data.len();

    assert!(nvars < std::u16::MAX as usize);

    w.write_u16_be(nvars as u16);

    for (_varid, _off, len) in template.data.iter() {
        w.write_u16_be(len as u16);
    }
}

fn encode_code(template: &AppTemplate, w: &mut Vec<u8>) {
    let code = &template.code;

    // code length
    let length = code.len();
    assert!(length < std::u32::MAX as usize);

    w.write_u32_be(length as u32);

    // code
    w.extend_from_slice(code)
}

/// Decoders

fn decode_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(name)) => Ok(name),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Name)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Name)),
    }
}

fn decode_data(cursor: &mut Cursor<&[u8]>) -> Result<DataLayout, ParseError> {
    match cursor.read_u16_be() {
        Err(..) => Err(ParseError::NotEnoughBytes(Field::DataLayoutVarsCount)),
        Ok(nvars) => {
            let mut builder = DataLayoutBuilder::with_capacity(nvars as usize);

            for _vid in 0..nvars as usize {
                match cursor.read_u16_be() {
                    Err(..) => return Err(ParseError::NotEnoughBytes(Field::DataLayoutVarLength)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_deploy_template() {
        let template = AppTemplate {
            version: 0,
            name: "My Template".to_string(),
            code: vec![0x0C, 0x00, 0x0D, 0x0E],
            data: vec![5, 10].into(),
        };

        let mut bytes = Vec::new();
        encode_deploy_template(&template, &mut bytes);

        let mut cursor = Cursor::new(&bytes[..]);

        let decoded = decode_deploy_template(&mut cursor).unwrap();

        assert_eq!(template, decoded);
    }
}
