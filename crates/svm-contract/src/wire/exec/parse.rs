use super::error::ContractExecError;
use super::field::Field;

use crate::wasm::{WasmArgType, WasmArgValue, WasmIntType};
use crate::Tx;
use svm_common::Address;

use byteorder::{BigEndian, ReadBytesExt};

use std::convert::TryFrom;
use std::io::{Cursor, Read};

macro_rules! ensure_enough_bytes {
    ($res: expr, $field: expr) => {{
        if $res.is_err() {
            return Err(ContractExecError::NotEnoughBytes($field));
        }
    }};
}

/// Parsing a on-the-wire smart-contract transaction given as raw bytes.
/// Returns the parsed contract as a `WasmContract` struct.
#[allow(dead_code)]
pub fn parse_tx(bytes: &[u8]) -> Result<Tx, ContractExecError> {
    let mut cursor = Cursor::new(bytes);

    parse_version(&mut cursor)?;

    let contract = parse_address(&mut cursor, Field::Contract)?;
    let sender = parse_address(&mut cursor, Field::Sender)?;
    let func_name = parse_func_name(&mut cursor)?;
    let func_args = parse_func_args(&mut cursor)?;

    let tx = Tx {
        contract,
        sender,
        func_name,
        func_args,
    };

    Ok(tx)
}

fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ContractExecError> {
    let res = cursor.read_u32::<BigEndian>();

    ensure_enough_bytes!(res, Field::Version);

    let version = res.unwrap();
    if version != 0 {
        return Err(ContractExecError::UnsupportedProtoVersion(version));
    }

    Ok(version)
}

fn parse_address(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<Address, ContractExecError> {
    let mut bytes = vec![0; 32];

    let res = cursor.read_exact(&mut bytes);
    ensure_enough_bytes!(res, field);

    let addr = Address::from(&bytes[..]);

    Ok(addr)
}

fn parse_func_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ContractExecError> {
    let res = cursor.read_u8();

    ensure_enough_bytes!(res, Field::FuncName);

    let name_len = res.unwrap() as usize;
    if name_len == 0 {
        return Err(ContractExecError::EmptyFuncName);
    }

    let mut name_buf = vec![0; name_len];
    let res = cursor.read_exact(&mut name_buf);

    if res.is_err() {
        return Err(ContractExecError::NotEnoughBytes(Field::FuncName));
    }

    // TODO: make `String::from_utf8` work without raising
    let name = unsafe { String::from_utf8_unchecked(name_buf) };

    Ok(name)
}

fn parse_func_args(cursor: &mut Cursor<&[u8]>) -> Result<Vec<WasmArgValue>, ContractExecError> {
    let args_count = read_u8(cursor, Field::ArgsCount)?;

    let mut args = Vec::with_capacity(args_count as usize);

    for _ in 0..args_count {
        let arg = parse_func_arg(cursor)?;
        args.push(arg);
    }

    Ok(args)
}

fn parse_func_arg(cursor: &mut Cursor<&[u8]>) -> Result<WasmArgValue, ContractExecError> {
    let arg_type = parse_func_arg_type(cursor)?;

    let arg_val = match arg_type {
        WasmArgType::I32 => {
            let arg_val = read_u32(cursor, Field::ArgValue)?;
            WasmArgValue::I32(arg_val)
        }
        WasmArgType::I64 => {
            let arg_val = read_u64(cursor, Field::ArgValue)?;
            WasmArgValue::I64(arg_val)
        }
        WasmArgType::Fixed => {
            let fixed_byte_length = read_u32(cursor, Field::ArgLength)?;
            let offset_int_type = parse_func_arg_int_type(cursor)?;

            let buf = read_buffer(cursor, fixed_byte_length, Field::ArgValue)?;

            WasmArgValue::Fixed(offset_int_type, buf)
        }
        WasmArgType::Slice => {
            // TODO: implement
            unimplemented!()
        }
    };

    Ok(arg_val)
}

fn parse_func_arg_type(cursor: &mut Cursor<&[u8]>) -> Result<WasmArgType, ContractExecError> {
    let byte = read_u8(cursor, Field::ArgType)?;

    let arg_type = WasmArgType::try_from(byte);

    match arg_type {
        Ok(arg_type) => Ok(arg_type),
        Err(_) => Err(ContractExecError::InvalidArgType(byte)),
    }
}

fn parse_func_arg_int_type(cursor: &mut Cursor<&[u8]>) -> Result<WasmIntType, ContractExecError> {
    let arg_type = parse_func_arg_type(cursor)?;

    match arg_type {
        WasmArgType::I32 => Ok(WasmIntType::I32),
        WasmArgType::I64 => Ok(WasmIntType::I64),
        _ => Err(ContractExecError::InvalidArgIntType),
    }
}

fn read_u8(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u8, ContractExecError> {
    let res = cursor.read_u8();

    ensure_enough_bytes!(res, field);

    Ok(res.unwrap())
}

fn read_u32(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u32, ContractExecError> {
    let res = cursor.read_u32::<BigEndian>();

    ensure_enough_bytes!(res, field);

    Ok(res.unwrap())
}

fn read_u64(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u64, ContractExecError> {
    let res = cursor.read_u64::<BigEndian>();

    ensure_enough_bytes!(res, field);

    Ok(res.unwrap())
}

fn read_buffer(
    cursor: &mut Cursor<&[u8]>,
    buf_len: u32,
    field: Field,
) -> Result<Vec<u8>, ContractExecError> {
    let mut buf = vec![0; buf_len as usize];

    let res = cursor.read_exact(&mut buf);
    ensure_enough_bytes!(res, field);

    Ok(buf)
}
