use crate::{
    error::ParseError,
    raw::{helpers, Field, NibbleIter},
    types::AppTemplate,
};

use svm_common::Address;

/// Returns the parsed raw app-template as `AppTemplate` struct.
#[must_use]
pub fn parse_template(bytes: &[u8], author: &Address) -> Result<AppTemplate, ParseError> {
    let mut iter = NibbleIter::new(bytes);

    helpers::decode_version(&mut iter)?;

    let name = decode_name(&mut iter)?;

    let _admins = decode_admins(&mut iter)?;
    decode_deps(&mut iter)?;

    let page_count = decode_page_count(&mut iter)?;
    let code = decode_code(&mut iter)?;

    let template = AppTemplate {
        name,
        author: author.clone(),
        page_count,
        code,
    };

    Ok(template)
}

#[must_use]
fn decode_name(iter: &mut NibbleIter) -> Result<String, ParseError> {
    todo!()
    // let res = cursor.read_u8();

    // helpers::ensure_enough_bytes(&res, Field::NameLength)?;

    // let name_len = res.unwrap() as usize;

    // if name_len == 0 {
    //     return Err(ParseError::EmptyField(Field::Name));
    // }

    // let mut buf = vec![0; name_len];
    // let res = cursor.read_exact(&mut buf);

    // if res.is_err() {
    //     return Err(ParseError::NotEnoughBytes(Field::Name));
    // }

    // String::from_utf8(buf).or_else(|_e| Err(ParseError::InvalidUTF8String(Field::Name)))
}

#[must_use]
fn decode_admins(iter: &mut NibbleIter) -> Result<Vec<Address>, ParseError> {
    todo!()
    // helpers::ensure_enough_bytes(&res, Field::AdminsCount)?;

    // let admin_count = res.unwrap() as usize;
    // if admin_count > 0 {
    //     return Err(ParseError::NotSupported(Field::Admins));
    // }

    // Ok(Vec::new())
}

#[must_use]
fn decode_deps(iter: &mut NibbleIter) -> Result<(), ParseError> {
    todo!()
    // let res = cursor.read_u16::<BigEndian>();

    // helpers::ensure_enough_bytes(&res, Field::DependenciesCount)?;

    // let deps_count = res.unwrap() as usize;

    // if deps_count > 0 {
    //     return Err(ParseError::NotSupported(Field::Dependencies));
    // }

    // Ok(())
}

#[must_use]
fn decode_page_count(iter: &mut NibbleIter) -> Result<u16, ParseError> {
    todo!()
    // helpers::read_u16(cursor, Field::PageCount)
}

#[must_use]
fn decode_code(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    todo!()
    // let res = cursor.read_u64::<BigEndian>();
    // helpers::ensure_enough_bytes(&res, Field::CodeLength)?;

    // let code_len = res.unwrap() as usize;
    // let mut code = vec![0; code_len];

    // let res = cursor.read_exact(&mut code);
    // helpers::ensure_enough_bytes(&res, Field::Code)?;

    // Ok(code)
}
