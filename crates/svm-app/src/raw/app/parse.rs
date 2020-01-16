use std::io::Cursor;

use crate::{
    error::ParseError,
    raw::{helpers, Field},
    types::{App, BufferSlice, SpawnApp},
};

use svm_common::Address;

/// Parsing a raw `spawn-app` transaction given as raw bytes.
/// Returns the parsed transaction as a tuple consisting of an `App` struct and `ctor` buffer args.
/// On failure, returns `ParseError`
#[must_use]
pub fn parse_app(bytes: &[u8], creator: &Address) -> Result<SpawnApp, ParseError> {
    let mut cursor = Cursor::new(bytes);

    helpers::parse_version(&mut cursor)?;

    let template = helpers::parse_address(&mut cursor, Field::AppTemplate)?;
    let ctor_buf_slices = helpers::parse_buffer_slices(&mut cursor)?;
    let ctor_args = helpers::parse_func_args(&mut cursor)?;

    let app = App {
        template,
        creator: creator.clone(),
    };

    let spawn_app = SpawnApp {
        app,
        ctor_buf_slices,
        ctor_args,
    };

    Ok(spawn_app)
}
