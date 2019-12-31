use super::{error::AppTemplateBuildError, field::Field};

use crate::wasm::AppTemplate;
use svm_common::Address;

use byteorder::{BigEndian, ReadBytesExt};

use log::{debug, error};
use std::io::{Cursor, Read};

macro_rules! ensure_enough_bytes {
    ($res: expr, $field: expr) => {{
        if $res.is_err() {
            error!("    parse failed. not enough bytes for field: {}", $field);

            return Err(AppTemplateBuildError::NotEnoughBytes($field));
        }
    }};
}

/// Parsing a on-the-wire `AppTemplate` given as raw bytes.
/// Returns the parsed contract as a `AppTemplate` struct.
#[allow(dead_code)]
pub fn parse_template(bytes: &[u8]) -> Result<AppTemplate, AppTemplateBuildError> {
    let mut cursor = Cursor::new(bytes);

    parse_version(&mut cursor)?;

    let name = parse_name(&mut cursor)?;
    let author = parse_author(&mut cursor)?;
    let _admins = parse_admins(&mut cursor)?;
    parse_deps(&mut cursor)?;
    let code = parse_code(&mut cursor)?;

    let contract = AppTemplate { name, code, author };

    Ok(contract)
}

fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, AppTemplateBuildError> {
    let res = cursor.read_u32::<BigEndian>();

    ensure_enough_bytes!(res, Field::Version);

    let version = res.unwrap();
    if version != 0 {
        return Err(AppTemplateBuildError::UnsupportedProtoVersion(version));
    }

    debug!("    parsed raw contract version: {:?}", version);

    Ok(version)
}

fn parse_name(cursor: &mut Cursor<&[u8]>) -> Result<String, AppTemplateBuildError> {
    let res = cursor.read_u8();

    ensure_enough_bytes!(res, Field::NameLength);

    let name_len = res.unwrap() as usize;

    if name_len == 0 {
        return Err(AppTemplateBuildError::EmptyName);
    }

    let mut name_buf = vec![0; name_len];
    let res = cursor.read_exact(&mut name_buf);

    if res.is_err() {
        return Err(AppTemplateBuildError::NotEnoughBytes(Field::Name));
    }

    // TODO: make `String::from_utf8` work without raising
    let name = unsafe { String::from_utf8_unchecked(name_buf) };

    Ok(name)
}

#[inline(always)]
fn parse_author(cursor: &mut Cursor<&[u8]>) -> Result<Address, AppTemplateBuildError> {
    parse_address(cursor, Field::Author)
}

fn parse_admins(cursor: &mut Cursor<&[u8]>) -> Result<Vec<Address>, AppTemplateBuildError> {
    let res = cursor.read_u16::<BigEndian>();

    ensure_enough_bytes!(res, Field::AdminsCount);

    let admin_count = res.unwrap() as usize;
    if admin_count > 0 {
        return Err(AppTemplateBuildError::AdminsNotSupportedYet);
    }

    // let mut admins = Vec::<Address>::with_capacity(admin_count);
    // for i in 0..admin_count {
    //     let addr = parse_address(addr, Field::Admins);
    //     admins.push(addr);
    // }

    Ok(Vec::new())
}

fn parse_deps(cursor: &mut Cursor<&[u8]>) -> Result<(), AppTemplateBuildError> {
    let res = cursor.read_u16::<BigEndian>();

    ensure_enough_bytes!(res, Field::DepsCount);

    let deps_count = res.unwrap() as usize;

    if deps_count > 0 {
        return Err(AppTemplateBuildError::DepsNotSupportedYet);
    }

    Ok(())
}

fn parse_code(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, AppTemplateBuildError> {
    let res = cursor.read_u64::<BigEndian>();
    ensure_enough_bytes!(res, Field::CodeLength);

    let code_len = res.unwrap() as usize;
    let mut code = vec![0; code_len];

    let res = cursor.read_exact(&mut code);
    ensure_enough_bytes!(res, Field::Code);

    Ok(code)
}

fn parse_address(
    cursor: &mut Cursor<&[u8]>,
    field: Field,
) -> Result<Address, AppTemplateBuildError> {
    let mut addr = vec![0; Address::len()];

    let res = cursor.read_exact(&mut addr);
    ensure_enough_bytes!(res, field);

    debug!("    parsed address (field={}) {:?}", field, addr);

    Ok(Address::from(addr.as_ref()))
}
