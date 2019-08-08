use super::contract::WireContract;
use super::field::Field;
use crate::types::Tag;
use svm_common::Address;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

pub enum ParseError {
    EmptyName,
    NameNotValidUTF8String,
    DepsNotSupportedYet,
    AdminsNotSupportedYet,
    NotEnoughBytes(Field),
    UnsupportedProtoVersion(u32),
}

macro_rules! ensure_enough_bytes {
    ($res: expr, $field: expr) => {{
        if $res.is_err() {
            return Err(ParseError::NotEnoughBytes($field));
        }
    }};
}

/// Parsing a on-the-wire contract given as raw bytes.
/// Returns the parsed contract as a `WireContract` struct.
#[allow(dead_code)]
pub fn parse_contract(bytes: &[u8]) -> Result<WireContract, ParseError> {
    let mut cursor = Cursor::new(bytes);

    parse_version(&mut cursor)?;
    let tag = parse_tag(&mut cursor)?;
    let name = parse_name(&mut cursor)?;
    let author = parse_author(&mut cursor)?;
    let admins = parse_admins(&mut cursor)?;
    let _deps = parse_deps(&mut cursor)?;
    let wasm = parse_code(&mut cursor)?;

    let contract = WireContract {
        name,
        wasm,
        tag,
        author,
        admins,
    };

    Ok(contract)
}

fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParseError> {
    let res = cursor.read_u32::<BigEndian>();

    ensure_enough_bytes!(res, Field::Version);

    let version = res.unwrap();
    if version != 0 {
        return Err(ParseError::UnsupportedProtoVersion(version));
    }

    Ok(version)
}

fn parse_tag(cursor: &mut Cursor<&[u8]>) -> Result<Tag, ParseError> {
    let mut tag = [0; 4];
    let res = cursor.read_exact(&mut tag);
    ensure_enough_bytes!(res, Field::Tag);

    Ok(Tag(tag))
}

fn parse_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    let res = cursor.read_u8();

    ensure_enough_bytes!(res, Field::NameLength);

    let name_len = res.unwrap() as usize;
    if name_len == 0 {
        return Err(ParseError::EmptyName);
    }

    let mut name_buf = Vec::<u8>::with_capacity(name_len);
    let res = cursor.read_exact(&mut name_buf);

    if res.is_err() {
        return Err(ParseError::NotEnoughBytes(Field::Name));
    }

    let name = String::from_utf8(name_buf);
    if name.is_err() {
        Ok(name.unwrap())
    } else {
        Err(ParseError::NameNotValidUTF8String)
    }
}

#[inline(always)]
fn parse_author(cursor: &mut Cursor<&[u8]>) -> Result<Address, ParseError> {
    parse_address(cursor, Field::Author)
}

fn parse_admins(cursor: &mut Cursor<&[u8]>) -> Result<Vec<Address>, ParseError> {
    let res = cursor.read_u8();

    ensure_enough_bytes!(res, Field::AdminsCount);

    let admin_count = res.unwrap() as usize;
    if admin_count > 0 {
        return Err(ParseError::AdminsNotSupportedYet);
    }
    // let mut admins = Vec::<Address>::with_capacity(admin_count);
    // for i in 0..admin_count {
    //     let addr = parse_address(addr, Field::Admins);
    //     admins.push(addr);
    // }

    Ok(Vec::new())
}

fn parse_deps(cursor: &mut Cursor<&[u8]>) -> Result<(), ParseError> {
    let res = cursor.read_u16::<BigEndian>();

    ensure_enough_bytes!(res, Field::DepsCount);

    let deps_count = res.unwrap() as usize;
    if deps_count > 0 {
        return Err(ParseError::DepsNotSupportedYet);
    }

    Ok(())
}

fn parse_code(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ParseError> {
    let res = cursor.read_u64::<BigEndian>();
    ensure_enough_bytes!(res, Field::CodeLength);

    let code_len = res.unwrap() as usize;
    let mut code = Vec::<u8>::with_capacity(code_len);

    let res = cursor.read_exact(&mut code);
    ensure_enough_bytes!(res, Field::Code);

    Ok(code)
}

fn parse_address(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<Address, ParseError> {
    let mut addr = [0; 32];
    let res = cursor.read_exact(&mut addr);
    ensure_enough_bytes!(res, field);

    Ok(Address(addr))
}
