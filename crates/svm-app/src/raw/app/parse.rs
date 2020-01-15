use std::io::Cursor;

use crate::{
    error::ParseError,
    raw::{helpers, Field},
    types::{App, BufferSlice},
};

use svm_common::Address;

/// Parsing a raw `spawn-app` transaction given as raw bytes.
/// Returns the parsed transaction as a tuple consisting of an `App` struct and `ctor` buffer initials.
/// On failure, returns `ParseError`
#[must_use]
pub fn parse_app(bytes: &[u8], creator: &Address) -> Result<(App, Vec<BufferSlice>), ParseError> {
    let mut cursor = Cursor::new(bytes);

    helpers::parse_version(&mut cursor)?;

    let template = helpers::parse_address(&mut cursor, Field::AppTemplate)?;

    let app = App {
        template,
        creator: creator.clone(),
    };

    let buf_slices = Vec::new();

    Ok((app, buf_slices))
}
