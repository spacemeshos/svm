use std::convert::TryFrom;
use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::Field;
use crate::{
    error::ParseError,
    raw::helpers,
    types::{BufferSlice, WasmType, WasmValue},
};

use svm_common::Address;

#[must_use]
#[inline]
pub fn ensure_enough_bytes<T>(res: &std::io::Result<T>, field: Field) -> Result<(), ParseError> {
    if res.is_err() {
        return Err(ParseError::NotEnoughBytes(field));
    }

    Ok(())
}

#[must_use]
pub fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParseError> {
    let res = cursor.read_u32::<BigEndian>();

    ensure_enough_bytes(&res, Field::Version)?;

    let version = res.unwrap();

    if version != 0 {
        return Err(ParseError::InvalidProtocolVersion(version as u32));
    }

    Ok(version)
}

#[must_use]
pub fn parse_address(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<Address, ParseError> {
    let mut bytes = vec![0; Address::len()];

    let res = cursor.read_exact(&mut bytes);

    ensure_enough_bytes(&res, field)?;

    let addr = Address::from(&bytes[..]);

    Ok(addr)
}

#[must_use]
pub fn parse_func_buf(cursor: &mut Cursor<&[u8]>) -> Result<Vec<BufferSlice>, ParseError> {
    let res = cursor.read_u8();

    ensure_enough_bytes(&res, Field::FuncBufSlicesCount)?;

    let arg_count = res.unwrap();
    let mut slices = Vec::new();

    for _ in 0..arg_count {
        let slice_len = read_u16(cursor, Field::FuncBufSliceLength)?;

        let data = read_buffer(cursor, slice_len as usize, Field::FuncBufSlice)?;

        let slice = BufferSlice { data };
        slices.push(slice);
    }

    Ok(slices)
}

#[must_use]
pub fn parse_func_args(cursor: &mut Cursor<&[u8]>) -> Result<Vec<WasmValue>, ParseError> {
    let arg_count = helpers::read_u8(cursor, Field::FuncArgsCount)?;

    let mut args = Vec::with_capacity(arg_count as usize);

    for _ in 0..arg_count {
        let arg = parse_func_arg(cursor)?;
        args.push(arg);
    }

    Ok(args)
}

#[must_use]
fn parse_func_arg(cursor: &mut Cursor<&[u8]>) -> Result<WasmValue, ParseError> {
    let arg_type = parse_func_arg_type(cursor)?;

    let arg = match arg_type {
        WasmType::I32 => {
            let val = helpers::read_u32(cursor, Field::WasmValue)?;
            WasmValue::I32(val)
        }
        WasmType::I64 => {
            let val = helpers::read_u64(cursor, Field::WasmValue)?;
            WasmValue::I64(val)
        }
    };

    Ok(arg)
}

#[must_use]
fn parse_func_arg_type(cursor: &mut Cursor<&[u8]>) -> Result<WasmType, ParseError> {
    let byte = helpers::read_u8(cursor, Field::WasmType)?;

    WasmType::try_from(byte).or_else(|_e| Err(ParseError::InvalidArgType(byte)))
}

#[must_use]
pub fn read_u8(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u8, ParseError> {
    let res = cursor.read_u8();

    ensure_enough_bytes(&res, field)?;

    Ok(res.unwrap())
}

#[must_use]
pub fn read_u16(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u16, ParseError> {
    let res = cursor.read_u16::<BigEndian>();

    ensure_enough_bytes(&res, field)?;

    Ok(res.unwrap())
}

#[must_use]
pub fn read_u32(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u32, ParseError> {
    let res = cursor.read_u32::<BigEndian>();

    ensure_enough_bytes(&res, field)?;

    Ok(res.unwrap())
}

#[must_use]
pub fn read_u64(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<u64, ParseError> {
    let res = cursor.read_u64::<BigEndian>();

    ensure_enough_bytes(&res, field)?;

    Ok(res.unwrap())
}

#[must_use]
pub fn read_buffer(
    cursor: &mut Cursor<&[u8]>,
    buf_len: usize,
    field: Field,
) -> Result<Vec<u8>, ParseError> {
    let mut buf = vec![0; buf_len];

    let res = cursor.read_exact(&mut buf);

    ensure_enough_bytes(&res, field)?;

    Ok(buf)
}

pub fn write_func_args(args: &Option<Vec<WasmValue>>, buf: &mut Vec<u8>) {
    if args.is_none() {
        buf.write_u8(0).unwrap();
        return;
    }

    let args = args.as_ref().unwrap();

    buf.write_u8(args.len() as u8).unwrap();

    for arg in args {
        match arg {
            WasmValue::I32(v) => {
                let arg_type = WasmType::I32.into();
                buf.write_u8(arg_type).unwrap();
                buf.write_u32::<BigEndian>(*v).unwrap();
            }
            WasmValue::I64(v) => {
                let arg_type = WasmType::I64.into();
                buf.write_u8(arg_type).unwrap();
                buf.write_u64::<BigEndian>(*v).unwrap();
            }
        }
    }
}

pub fn write_func_buf(slices: &Option<Vec<Vec<u8>>>, buf: &mut Vec<u8>) {
    if slices.is_none() {
        buf.write_u8(0).unwrap();
        return;
    }

    let slices = slices.as_ref().unwrap();

    buf.write_u8(slices.len() as u8).unwrap();

    for slice in slices {
        let len = slice.len() as u16;
        buf.write_u16::<BigEndian>(len).unwrap();

        buf.extend_from_slice(&slice);
    }
}
