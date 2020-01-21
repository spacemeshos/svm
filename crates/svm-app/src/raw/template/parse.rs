use std::io::{Cursor, Read};

use crate::{
    error::ParseError,
    raw::{helpers, Field},
    types::AppTemplate,
};

use svm_common::Address;

use byteorder::{BigEndian, ReadBytesExt};

/// Returns the parsed raw app-template as `AppTemplate` struct.
#[must_use]
pub fn parse_template(bytes: &[u8], author: &Address) -> Result<AppTemplate, ParseError> {
    let mut cursor = Cursor::new(bytes);

    helpers::parse_version(&mut cursor)?;

    let name = parse_name(&mut cursor)?;
    let _admins = parse_admins(&mut cursor)?;
    parse_deps(&mut cursor)?;
    let page_count = parse_page_count(&mut cursor)?;
    let code = parse_code(&mut cursor)?;

    let template = AppTemplate {
        name,
        author: author.clone(),
        page_count,
        code,
    };

    Ok(template)
}

#[must_use]
fn parse_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    let res = cursor.read_u8();

    helpers::ensure_enough_bytes(&res, Field::NameLength)?;

    let name_len = res.unwrap() as usize;

    if name_len == 0 {
        return Err(ParseError::EmptyField(Field::Name));
    }

    let mut buf = vec![0; name_len];
    let res = cursor.read_exact(&mut buf);

    if res.is_err() {
        return Err(ParseError::NotEnoughBytes(Field::Name));
    }

    String::from_utf8(buf).or_else(|_e| Err(ParseError::InvalidUTF8String(Field::Name)))
}

#[must_use]
fn parse_admins(cursor: &mut Cursor<&[u8]>) -> Result<Vec<Address>, ParseError> {
    let res = cursor.read_u16::<BigEndian>();

    helpers::ensure_enough_bytes(&res, Field::AdminsCount)?;

    let admin_count = res.unwrap() as usize;
    if admin_count > 0 {
        return Err(ParseError::NotSupported(Field::Admins));
    }

    Ok(Vec::new())
}

#[must_use]
fn parse_deps(cursor: &mut Cursor<&[u8]>) -> Result<(), ParseError> {
    let res = cursor.read_u16::<BigEndian>();

    helpers::ensure_enough_bytes(&res, Field::DependenciesCount)?;

    let deps_count = res.unwrap() as usize;

    if deps_count > 0 {
        return Err(ParseError::NotSupported(Field::Dependencies));
    }

    Ok(())
}

#[must_use]
fn parse_page_count(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    helpers::read_u16(cursor, Field::PageCount)
}

#[must_use]
fn parse_code(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ParseError> {
    let res = cursor.read_u64::<BigEndian>();
    helpers::ensure_enough_bytes(&res, Field::CodeLength)?;

    let code_len = res.unwrap() as usize;
    let mut code = vec![0; code_len];

    let res = cursor.read_exact(&mut code);
    helpers::ensure_enough_bytes(&res, Field::Code)?;

    Ok(code)
}
