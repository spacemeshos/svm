use crate::{
    error::ParseError,
    raw::{concat_nibbles, helpers, Field, NibbleIter},
    types::AppTemplate,
};

use svm_common::Address;

/// Returns the parsed raw app-template as `AppTemplate` struct.
#[must_use]
pub fn parse_template(bytes: &[u8]) -> Result<AppTemplate, ParseError> {
    let mut iter = NibbleIter::new(bytes);

    let version = helpers::decode_version(&mut iter)?;
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

#[must_use]
fn decode_name(iter: &mut NibbleIter) -> Result<String, ParseError> {
    helpers::decode_string(iter, Field::NameLength, Field::Name)
}

#[must_use]
fn decode_page_count(iter: &mut NibbleIter) -> Result<u16, ParseError> {
    helpers::decode_varuint14(iter, Field::PageCount)
}

#[must_use]
fn decode_code(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    let mut nibs = Vec::new();

    while let Some(nib) = iter.next() {
        nibs.push(nib);
    }

    // if `_rem` isn't `None` it means it's a padding nibble.
    let (code, _rem) = concat_nibbles(&nibs[..]);

    Ok(code)
}
