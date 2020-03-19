use crate::{
    error::ParseError,
    raw::{helpers, Field, NibbleIter, NibbleWriter},
    types::AppTemplate,
};

/// Encodes a raw Deploy-Template.
#[must_use]
pub fn encode_deploy_template(template: &AppTemplate, w: &mut NibbleWriter) {
    encode_version(template, w);
    encode_name(template, w);
    encode_page_count(template, w);
    encode_code(template, w);
}

/// Decodes a raw Deploy-Template.
#[must_use]
pub fn decode_deploy_template(iter: &mut NibbleIter) -> Result<AppTemplate, ParseError> {
    let version = decode_version(iter)?;
    let name = decode_name(iter)?;
    let page_count = decode_page_count(iter)?;
    let code = decode_code(iter)?;

    let template = AppTemplate {
        version,
        name,
        page_count,
        code,
    };

    Ok(template)
}

/// Encoders

fn encode_version(template: &AppTemplate, w: &mut NibbleWriter) {
    let version = *&template.version;
    helpers::encode_version(version, w);
}

fn encode_name(template: &AppTemplate, w: &mut NibbleWriter) {
    helpers::encode_string(&template.name, w);
}

fn encode_page_count(template: &AppTemplate, w: &mut NibbleWriter) {
    let page_count = *&template.page_count;
    helpers::encode_varuint14(page_count, w);
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

fn decode_page_count(iter: &mut NibbleIter) -> Result<u16, ParseError> {
    helpers::decode_varuint14(iter, Field::PageCount)
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
            page_count: 5,
            code: vec![0x0C, 0x00, 0x0D, 0x0E],
        };

        let mut w = NibbleWriter::new();
        encode_deploy_template(&template, &mut w);

        let bytes = w.into_bytes();
        let mut iter = NibbleIter::new(&bytes[..]);

        let decoded = decode_deploy_template(&mut iter).unwrap();

        assert_eq!(template, decoded);
    }
}
