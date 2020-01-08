use std::convert::TryFrom;
use std::io::{Cursor, Read};

use byteorder::ReadBytesExt;

use crate::{
    error::ParseError,
    raw::{helpers, Field},
    types::{AppTransaction, WasmArgType, WasmArgValue, WasmIntType},
};

/// Parsing a on-the-wire `AppTransaction` deploy transaction given as raw bytes.
/// Returns the parsed transaction as a `AppTransaction` struct.
/// On failure, returns `ParseError`.
#[must_use]
pub fn parse_app_tx(bytes: &[u8]) -> Result<AppTransaction, ParseError> {
    let mut cursor = Cursor::new(bytes);

    helpers::parse_version(&mut cursor)?;

    let app = helpers::parse_address(&mut cursor, Field::App)?;
    let sender = helpers::parse_address(&mut cursor, Field::Sender)?;
    let func_name = parse_func_name(&mut cursor)?;
    let func_args = parse_func_args(&mut cursor)?;

    let tx = AppTransaction {
        app,
        sender,
        func_name,
        func_args,
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
        WasmArgType::Fixed => {
            let fixed_byte_length = helpers::read_u32(cursor, Field::ArgLength)?;
            let offset_int_type = parse_func_arg_int_type(cursor)?;

            let buf = helpers::read_buffer(cursor, fixed_byte_length, Field::ArgValue)?;

            WasmArgValue::Fixed(offset_int_type, buf)
        }
        WasmArgType::Slice => todo!(),
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

#[must_use]
fn parse_func_arg_int_type(cursor: &mut Cursor<&[u8]>) -> Result<WasmIntType, ParseError> {
    let arg_type = parse_func_arg_type(cursor)?;

    match arg_type {
        WasmArgType::I32 => Ok(WasmIntType::I32),
        WasmArgType::I64 => Ok(WasmIntType::I64),
        _ => Err(ParseError::InvalidArgIntType),
    }
}
