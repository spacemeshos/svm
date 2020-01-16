use std::io::{Cursor, Read};

use byteorder::ReadBytesExt;

use crate::{
    error::ParseError,
    raw::{helpers, Field},
    types::{AppTransaction, BufferSlice, WasmType, WasmValue},
};

use svm_common::Address;

/// Parsing a raw `AppTransaction` transaction given as raw bytes.
/// Returns the parsed transaction as a `AppTransaction` struct.
/// On failure, returns `ParseError`.
#[must_use]
pub fn parse_app_tx(bytes: &[u8], sender: &Address) -> Result<AppTransaction, ParseError> {
    let mut cursor = Cursor::new(bytes);

    helpers::parse_version(&mut cursor)?;

    let app = helpers::parse_address(&mut cursor, Field::App)?;
    let func_name = parse_func_name(&mut cursor)?;
    let func_args_buf = helpers::parse_buffer_slices(&mut cursor)?;
    let func_args = helpers::parse_func_args(&mut cursor)?;

    let tx = AppTransaction {
        app,
        sender: sender.clone(),
        func_name,
        func_args,
        func_args_buf,
    };

    Ok(tx)
}

#[must_use]
fn parse_func_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    let res = cursor.read_u8();

    helpers::ensure_enough_bytes(&res, Field::FuncNameLength)?;

    let name_len = res.unwrap() as usize;
    if name_len == 0 {
        return Err(ParseError::EmptyField(Field::FuncName));
    }

    let mut buf = vec![0; name_len];
    let res = cursor.read_exact(&mut buf);

    if res.is_err() {
        return Err(ParseError::NotEnoughBytes(Field::FuncName));
    }

    String::from_utf8(buf).or_else(|_e| Err(ParseError::InvalidUTF8String(Field::Name)))
}
