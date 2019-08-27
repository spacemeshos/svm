use super::error::ContractDeployError;
use super::field::Field;
use crate::wasm::WasmContract;
use svm_common::Address;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

macro_rules! ensure_enough_bytes {
    ($res: expr, $field: expr) => {{
        if $res.is_err() {
            return Err(ContractDeployError::NotEnoughBytes($field));
        }
    }};
}

/// Parsing a on-the-wire contract given as raw bytes.
/// Returns the parsed contract as a `WasmContract` struct.
#[allow(dead_code)]
pub fn parse_contract(bytes: &[u8]) -> Result<WasmContract, ContractDeployError> {
    let mut cursor = Cursor::new(bytes);

    parse_version(&mut cursor)?;

    let name = parse_name(&mut cursor)?;
    let author = parse_author(&mut cursor)?;
    let _admins = parse_admins(&mut cursor)?;
    let _deps = parse_deps(&mut cursor)?;
    let wasm = parse_code(&mut cursor)?;

    let contract = WasmContract {
        address: None,
        name,
        wasm,
        author,
    };

    Ok(contract)
}

fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ContractDeployError> {
    let res = cursor.read_u32::<BigEndian>();

    ensure_enough_bytes!(res, Field::Version);

    let version = res.unwrap();
    if version != 0 {
        return Err(ContractDeployError::UnsupportedProtoVersion(version));
    }

    Ok(version)
}

fn parse_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ContractDeployError> {
    let res = cursor.read_u8();

    ensure_enough_bytes!(res, Field::NameLength);

    let name_len = res.unwrap() as usize;
    if name_len == 0 {
        return Err(ContractDeployError::EmptyName);
    }

    let mut name_buf = vec![0; name_len];
    let res = cursor.read_exact(&mut name_buf);

    if res.is_err() {
        return Err(ContractDeployError::NotEnoughBytes(Field::Name));
    }

    // TODO: make `String::from_utf8` work without raising
    let name = unsafe { String::from_utf8_unchecked(name_buf) };

    Ok(name)
    // if name.is_err() {
    //     Ok(name.unwrap())
    // } else {
    //     Err(ContractDeployError::NameNotValidUTF8String)
    // }
}

#[inline(always)]
fn parse_author(cursor: &mut Cursor<&[u8]>) -> Result<Address, ContractDeployError> {
    parse_address(cursor, Field::Author)
}

fn parse_admins(cursor: &mut Cursor<&[u8]>) -> Result<Vec<Address>, ContractDeployError> {
    let res = cursor.read_u16::<BigEndian>();

    ensure_enough_bytes!(res, Field::AdminsCount);

    let admin_count = res.unwrap() as usize;
    if admin_count > 0 {
        return Err(ContractDeployError::AdminsNotSupportedYet);
    }

    // let mut admins = Vec::<Address>::with_capacity(admin_count);
    // for i in 0..admin_count {
    //     let addr = parse_address(addr, Field::Admins);
    //     admins.push(addr);
    // }

    Ok(Vec::new())
}

fn parse_deps(cursor: &mut Cursor<&[u8]>) -> Result<(), ContractDeployError> {
    let res = cursor.read_u16::<BigEndian>();

    ensure_enough_bytes!(res, Field::DepsCount);

    let deps_count = res.unwrap() as usize;

    if deps_count > 0 {
        return Err(ContractDeployError::DepsNotSupportedYet);
    }

    Ok(())
}

fn parse_code(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ContractDeployError> {
    let res = cursor.read_u64::<BigEndian>();
    ensure_enough_bytes!(res, Field::CodeLength);

    let code_len = res.unwrap() as usize;

    let mut code = vec![0; code_len];

    let res = cursor.read_exact(&mut code);
    ensure_enough_bytes!(res, Field::Code);

    Ok(code)
}

fn parse_address(cursor: &mut Cursor<&[u8]>, field: Field) -> Result<Address, ContractDeployError> {
    let mut addr = [0; 32];

    let res = cursor.read_exact(&mut addr);
    ensure_enough_bytes!(res, field);

    Ok(Address::from(addr.as_ref()))
}
