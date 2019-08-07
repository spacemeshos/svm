use crate::contract::Contract;
use crate::traits::CodeHashStore;
use crate::types::{Address, CodeHash, Revision, Tag};

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
//          Deploy Contract Wire Protocol Version 0.0
//  -------------------------------------------------------
//  |   proto    |                |                       |
//  |  version   |  name length   |     name (UTF-8)      |
//  |  (4 bytes) |   (1 byte)     |                       |
//  |____________|________________|_______________________|
//  |       tag         |  #authors |  authors addresses  |
//  |     (4 bytes)     |  (1 byte) |   (32 bytes each)   |
//  |___________________|__________ |_____________________|
//  |           |                                         |
//  |   #deps   |           dependency #1                 |
//  | (2 bytes) |               TBD                       |
//  |___________|_________________________________________|
//  |                |                                    |
//  |                |                                    |
//  |  code length   |           code (wasm)              |
//  |   (8 bytes)    |                                    |
//  |                |                                    |
//  |________________|____________________________________|
//

/// Parsing a serialized deploy contract given as raw bytes.
/// Returned the parsed
pub fn parse_contract(
    bytes: &[u8],
    store: &mut impl CodeHashStore,
) -> Result<Contract, ParseError> {
    let mut cursor = Cursor::new(bytes);

    let proto_ver = cursor.read_u32::<BigEndian>().unwrap();
    if proto_ver != 0 {
        return Err(ParseError::UnsupportedProtoVersion(proto_ver));
    }

    let name_len = cursor.read_u8().unwrap();
    let mut name = Vec::<u8>::with_capacity(name_len as usize);
    cursor.read_exact(&mut name).unwrap();

    let tag = cursor.read_u32::<BigEndian>().unwrap();
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

    let deps_count = cursor.read_u16::<BigEndian>().unwrap();
    if deps_count > 0 {
        return Err(ParseError::DepsNotSupportedYet);
    }

    let wasm_len = cursor.read_u64::<BigEndian>().unwrap();
    let mut wasm = Vec::<u8>::with_capacity(wasm_len as usize);

    // let contract = Contract {};
    // return Ok(contract);
    panic!()
}
