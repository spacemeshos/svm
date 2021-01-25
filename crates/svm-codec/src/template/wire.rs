use std::{
    fs::File,
    io::{Cursor, Read},
};

use svm_types::AppTemplate;

use crate::api::raw::{decode_varuint14, decode_version, encode_varuint14, Field};

use crate::{error::ParseError, helpers};

use svm_layout::{DataLayout, DataLayoutBuilder};

/// Encodes a raw Deploy-Template.
pub fn encode_deploy_template(template: &AppTemplate, w: &mut Vec<u8>) {
    encode_version(template, w);
    encode_name(template, w);
    encode_code(template, w);
    encode_data(template, w);
}

/// Decodes a raw Deploy-Template.
pub fn decode_deploy_template(cursor: &mut Cursor<&[u8]>) -> Result<AppTemplate, ParseError> {
    let version = decode_version(cursor)?;
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
    crate::api::raw::encode_version(version, w);
}

fn encode_name(template: &AppTemplate, w: &mut Vec<u8>) {
    helpers::encode_string(&template.name, w);
}

fn encode_data(template: &AppTemplate, w: &mut Vec<u8>) {
    let nvars = template.data.len() as u32;
    encode_varuint14(nvars as u16, w);

    for (_varid, _off, len) in template.data.iter() {
        encode_varuint14(len as u16, w);
    }
}

fn encode_code(template: &AppTemplate, w: &mut Vec<u8>) {
    let code = &template.code;

    // code length
    let length = code.len();
    assert!(length < std::u32::MAX as usize);

    helpers::encode_u32_be(length as u32, w);

    // code
    w.extend_from_slice(code)
}

/// Decoders

fn decode_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    helpers::decode_string(cursor, Field::NameLength, Field::Name)
}

fn decode_data(cursor: &mut Cursor<&[u8]>) -> Result<DataLayout, ParseError> {
    let nvars = decode_varuint14(cursor, Field::DataLayoutVarsCount)?;

    let mut builder = DataLayoutBuilder::with_capacity(nvars as usize);

    for _vid in 0..nvars as usize {
        let len = decode_varuint14(cursor, Field::DataLayoutVarLength)?;

        builder.add_var(len as u32);
    }

    let layout = builder.build();

    Ok(layout)
}

fn decode_code(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ParseError> {
    let length = helpers::decode_u32_be(cursor, Field::CodeSize)?;

    let mut buf = Vec::with_capacity(length as usize);

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::Code));
    }

    Ok(buf)
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
        encode_deploy_template(&template, &mut w);

        let mut iter = NibbleIter::new(&bytes);

        let decoded = decode_deploy_template(&mut iter).unwrap();

        assert_eq!(template, decoded);
    }
}
