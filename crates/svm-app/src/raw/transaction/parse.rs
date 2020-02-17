use crate::{
    error::ParseError,
    raw::{helpers, Field, NibbleIter},
    types::{AppTransaction, WasmValue},
};

use svm_common::Address;

/// Parsing a raw `AppTransaction` transaction given as raw bytes.
/// Returns the parsed transaction as a `AppTransaction` struct.
/// On failure, returns `ParseError`.
#[must_use]
pub fn parse_app_tx(bytes: &[u8], sender: &Address) -> Result<AppTransaction, ParseError> {
    let mut iter = NibbleIter::new(bytes);

    helpers::decode_version(&mut iter)?;

    let app = helpers::decode_address(&mut iter, Field::App)?;
    let func_idx = decode_func_index(&mut iter)?;
    let func_buf = helpers::decode_func_buf(&mut iter)?;
    let func_args = helpers::decode_func_args(&mut iter)?;

    helpers::ensure_eof(&mut iter);

    let tx = AppTransaction {
        app,
        sender: sender.clone(),
        func_idx,
        func_args,
        func_buf,
    };

    Ok(tx)
}

#[must_use]
fn decode_func_index(iter: &mut NibbleIter) -> Result<u16, ParseError> {
    helpers::decode_varuint14(iter, Field::FuncIndex)
}
