use crate::{
    error::ParseError,
    raw::{helpers, Field, NibbleIter},
    types::{App, SpawnApp},
};

use svm_common::Address;

/// Parsing a raw `spawn-app` transaction given as raw bytes.
/// Returns the parsed transaction as a tuple consisting of an `App` struct and `ctor` buffer args.
/// On failure, returns `ParseError`
#[must_use]
pub fn parse_app(bytes: &[u8], creator: &Address) -> Result<SpawnApp, ParseError> {
    let mut iter = NibbleIter::new(bytes);

    helpers::decode_version(&mut iter)?;

    let template = helpers::decode_address(&mut iter, Field::AppTemplate)?;
    let ctor_buf = helpers::decode_func_buf(&mut iter)?;
    let ctor_args = helpers::decode_func_args(&mut iter)?;

    helpers::ensure_eof(&mut iter);

    let app = App {
        template,
        creator: creator.clone(),
    };

    let spawn_app = SpawnApp {
        app,
        ctor_buf,
        ctor_args,
    };

    Ok(spawn_app)
}
