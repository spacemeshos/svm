use std::convert::TryFrom;
use std::io::{Cursor, Read};

use byteorder::ReadBytesExt;

use crate::{
    error::ParseError,
    raw::{helpers, Field},
    types::{AppTransaction, BufferSlice, WasmArgType, WasmArgValue},
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
    let func_args = parse_func_args(&mut cursor)?;
    let func_args_buf = parse_func_args_buf(&mut cursor)?;

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

    let mut name_buf = vec![0; name_len];
    let res = cursor.read_exact(&mut name_buf);

    if res.is_err() {
        return Err(ParseError::NotEnoughBytes(Field::FuncName));
    }

    // TODO: make `String::from_utf8` work without raising
    let name = unsafe { String::from_utf8_unchecked(name_buf) };

    Ok(name)
}

#[must_use]
fn parse_func_args(cursor: &mut Cursor<&[u8]>) -> Result<Vec<WasmArgValue>, ParseError> {
    let args_count = helpers::read_u8(cursor, Field::ArgsCount)?;

    let mut args = Vec::with_capacity(args_count as usize);

    for _ in 0..args_count {
        let arg = parse_func_arg(cursor)?;
        args.push(arg);
    }

    Ok(args)
}

#[must_use]
fn parse_func_args_buf(cursor: &mut Cursor<&[u8]>) -> Result<Vec<BufferSlice>, ParseError> {
    // TODO: ...
    Ok(Vec::new())
}

#[must_use]
fn parse_func_arg(cursor: &mut Cursor<&[u8]>) -> Result<WasmArgValue, ParseError> {
    let arg_type = parse_func_arg_type(cursor)?;

    let arg_val = match arg_type {
        WasmArgType::I32 => {
            let arg_val = helpers::read_u32(cursor, Field::ArgValue)?;
            WasmArgValue::I32(arg_val)
        }
        WasmArgType::I64 => {
            let arg_val = helpers::read_u64(cursor, Field::ArgValue)?;
            WasmArgValue::I64(arg_val)
        }
    };

    Ok(arg_val)
}

#[must_use]
fn parse_func_arg_type(cursor: &mut Cursor<&[u8]>) -> Result<WasmArgType, ParseError> {
    let byte = helpers::read_u8(cursor, Field::ArgType)?;

    let arg_type = WasmArgType::try_from(byte);

    match arg_type {
        Ok(arg_type) => Ok(arg_type),
        Err(..) => Err(ParseError::InvalidArgType(byte)),
    }
}
