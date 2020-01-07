use std::io::Cursor;

use crate::{
    error::ParseError,
    raw::{helpers, Field},
    types::App,
};

use svm_common::Address;

/// Parsing a on-the-wire `App` deploy transaction given as raw bytes.
/// Returns the parsed transaction as a `App` struct.
/// On failure, returns `ParseError`
#[must_use]
#[allow(dead_code)]
pub fn parse_app(bytes: &[u8]) -> Result<App, ParseError> {
    let mut cursor = Cursor::new(bytes);

    helpers::parse_version(&mut cursor)?;

    let template = helpers::parse_address(&mut cursor, Field::AppTemplate)?;
    let creator = helpers::parse_address(&mut cursor, Field::Creator)?;

    let app = App { template, creator };

    Ok(app)
}
