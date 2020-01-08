use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};

use super::Field;
use crate::error::ParseError;

use svm_common::Address;

#[must_use]
#[inline(always)]
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
    buf_len: u32,
    field: Field,
) -> Result<Vec<u8>, ParseError> {
    let mut buf = vec![0; buf_len as usize];

    let res = cursor.read_exact(&mut buf);
    ensure_enough_bytes(&res, field)?;

    Ok(buf)
}
