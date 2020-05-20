use crate::{
    error::ParseError,
    raw::{helpers, Field, NibbleIter, NibbleWriter},
    types::AppTemplate,
};
use svm_layout::{DataLayout, DataLayoutBuilder, VarId};

/// Encodes a raw Deploy-Template.
pub fn encode_deploy_template(template: &AppTemplate, w: &mut NibbleWriter) {
    encode_version(template, w);
    encode_name(template, w);
    encode_code(template, w);
    encode_data(template, w);
}

/// Decodes a raw Deploy-Template.
pub fn decode_deploy_template(iter: &mut NibbleIter) -> Result<AppTemplate, ParseError> {
    let version = decode_version(iter)?;
    let name = decode_name(iter)?;
    let code = decode_code(iter)?;
    let data = decode_data(iter)?;

    let template = AppTemplate {
        version,
        name,
        code,
        data,
    };

    Ok(template)
}

/// Encoders

fn encode_version(template: &AppTemplate, w: &mut NibbleWriter) {
    let version = template.version;
    helpers::encode_version(version, w);
}

fn encode_name(template: &AppTemplate, w: &mut NibbleWriter) {
    helpers::encode_string(&template.name, w);
}

fn encode_data(template: &AppTemplate, w: &mut NibbleWriter) {
    let nvars = template.data.len() as u32;
    helpers::encode_u32_be(nvars, w);

    for (_vid, _off, len) in template.data.iter() {
        helpers::encode_varuint14(len as u16, w);
    }
}

fn encode_code(template: &AppTemplate, w: &mut NibbleWriter) {
    let code = &template.code;

    // code length
    let length = code.len();
    assert!(length < std::u32::MAX as usize);

    helpers::encode_u32_be(length as u32, w);

    // code
    w.write_bytes(code)
}

/// Decoders

fn decode_version(iter: &mut NibbleIter) -> Result<u32, ParseError> {
    helpers::decode_version(iter)
}

fn decode_name(iter: &mut NibbleIter) -> Result<String, ParseError> {
    helpers::decode_string(iter, Field::NameLength, Field::Name)
}

fn decode_data(iter: &mut NibbleIter) -> Result<DataLayout, ParseError> {
    let nvars = helpers::decode_u32_be(iter, Field::DataLayoutVarsCount)?;

    let mut builder = DataLayoutBuilder::with_capacity(nvars as usize);

    for _vid in 0..nvars as usize {
        let len = helpers::decode_varuint14(iter, Field::DataLayoutVarLength)?;

        builder.add_var(len as u32);
    }

    let layout = builder.build();

    Ok(layout)
}

fn decode_code(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    let length = helpers::decode_u32_be(iter, Field::CodeLength)?;
    let code = iter.read_bytes(length as usize);

    Ok(code)
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

        let mut w = NibbleWriter::new();
        encode_deploy_template(&template, &mut w);

        let bytes = w.into_bytes();
        let mut iter = NibbleIter::new(&bytes[..]);

        let decoded = decode_deploy_template(&mut iter).unwrap();

        assert_eq!(template, decoded);
    }
}
