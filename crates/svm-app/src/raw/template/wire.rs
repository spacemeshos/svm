use crate::{
    error::ParseError,
    raw::{concat_nibbles, helpers, Field, NibbleIter, NibbleWriter},
    types::AppTemplate,
};

#[must_use]
pub fn encode_deploy_template(version: u32, name: &str, page_count: u16, code: &[u8]) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    encode_version(version, &mut w);
    encode_name(name, &mut w);
    encode_page_count(page_count, &mut w);
    encode_code(code, &mut w);

    helpers::bytes(&mut w)
}

#[must_use]
pub fn decode_deploy_template(bytes: &[u8]) -> Result<AppTemplate, ParseError> {
    let mut iter = NibbleIter::new(bytes);

    let version = decode_version(&mut iter)?;
    let name = decode_name(&mut iter)?;
    let page_count = decode_page_count(&mut iter)?;
    let code = decode_code(&mut iter)?;

    helpers::ensure_eof(&mut iter);

    let template = AppTemplate {
        version,
        name,
        page_count,
        code,
    };

    Ok(template)
}

/// Encoders

fn encode_version(version: u32, w: &mut NibbleWriter) {
    helpers::encode_version(version, w);
}

fn encode_name(name: &str, w: &mut NibbleWriter) {
    helpers::encode_string(name, w);
}

fn encode_page_count(page_count: u16, w: &mut NibbleWriter) {
    helpers::encode_varuint14(page_count, w);
}

fn encode_code(code: &[u8], w: &mut NibbleWriter) {
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
    let mut nibs = Vec::new();

    while let Some(nib) = iter.next() {
        nibs.push(nib);
    }

    // if `_rem` isn't `None` it means it's a padding nibble.
    let (code, _rem) = concat_nibbles(&nibs[..]);

    Ok(code)
}
