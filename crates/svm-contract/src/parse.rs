use crate::traits::CodeHashStore;
use crate::types::{CodeHash, Revision, Tag};
use crate::wire_contract::WireContract;

use svm_common::Address;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

pub enum ParseError {
    NoAuthors,
    DepsNotSupportedYet,
    InvalidWasm,
    MissingField(Field),
    DependencyNotFound(Revision),
    UnsupportedProtoVersion(u32),
}

pub enum Field {
    Name,
    Tag,
    Authors,
    CodeHash,
    Deps,
}

//
//          Deploy Contract Wire Protocol Version
//  -------------------------------------------------------
//  |   proto    |                |                       |
//  |  version   |  name length   |     name (UTF-8)      |
//  |  (4 bytes) |   (1 byte)     |                       |
//  |____________|________________|_______________________|
//  |       tag         |  #authors |  authors addresses  |
//  |     (4 bytes)     |  (1 byte) |   (32 bytes each)   |
//  |___________________|__________ |_____________________|
//  |           |                                         |
//  |   #deps   |           dependencies                  |
//  | (2 bytes) |              (TBD)                      |
//  |___________|_________________________________________|
//  |                |                                    |
//  |                |                                    |
//  |  code length   |           code (wasm)              |
//  |   (8 bytes)    |                                    |
//  |                |                                    |
//  |________________|____________________________________|
//

/// Parsing a on-the-wire contract given as raw bytes.
/// Returns the parsed contract as a `WireContract` struct.
pub fn parse_contract(
    bytes: &[u8],
    store: &mut impl CodeHashStore,
) -> Result<WireContract, ParseError> {
    let mut cursor = Cursor::new(bytes);

    let version = parse_version(&mut cursor)?;
    let tag = parse_tag(&mut cursor)?;
    let name = parse_name(&mut cursor)?;
    let authors = parse_authors(&mut cursor)?;
    let wasm = parse_wasm(&mut cursor)?;

    let contract = WireContract {
        name,
        wasm,
        tag,
        authors,
    };

    Ok(contract)
}

fn parse_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParseError> {
    let version = cursor.read_u32::<BigEndian>().unwrap();
    if version != 0 {
        return Err(ParseError::UnsupportedProtoVersion(version));
    }
    Ok(version)
}

fn parse_tag(cursor: &mut Cursor<&[u8]>) -> Result<Tag, ParseError> {
    let mut tag = [0; 4];
    cursor.read_exact(&mut tag).unwrap();
    Ok(Tag(tag))
}

fn parse_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    let name_len = cursor.read_u8().unwrap();
    // TODO: assert `name_len > 0`

    let mut name_buf = Vec::<u8>::with_capacity(name_len as usize);
    cursor.read_exact(&mut name_buf).unwrap();

    let name = String::from_utf8(name_buf).unwrap();
    // TODO: return error if `name` isn't a valid UTF-8 string

    Ok(name)
}

fn parse_authors(cursor: &mut Cursor<&[u8]>) -> Result<Vec<Address>, ParseError> {
    let authors_count = cursor.read_u8().unwrap() as usize;
    if authors_count == 0 {
        return Err(ParseError::NoAuthors);
    }
    let mut authors = Vec::<Address>::with_capacity(authors_count);
    for i in 0..authors_count {
        let mut addr = [0; 32];
        cursor.read_exact(&mut addr).unwrap();
        authors.push(Address(addr));
    }

    Ok(authors)
}

fn parse_deps(cursor: &mut Cursor<&[u8]>) -> Result<(), ParseError> {
    let deps_count = cursor.read_u16::<BigEndian>().unwrap();
    if deps_count > 0 {
        return Err(ParseError::DepsNotSupportedYet);
    }

    Ok(())
}

fn parse_wasm(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ParseError> {
    let wasm_len = cursor.read_u64::<BigEndian>().unwrap();
    let mut wasm = Vec::<u8>::with_capacity(wasm_len as usize);

    Ok(wasm)
}
