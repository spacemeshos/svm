//!             `Host Ctx` Raw Format Version 0.0.0.0
//!  +------------------------------------------------------------+
//!  |   proto    |           | field #1  |  field #1  | field #1 |
//!  |  version   |  #fields  |   index   |   length   |          |
//!  | (4 bytes)  | (2 bytes) | (2 bytes) | (2 bytes)  |   bytes  |
//!  |____________|___________|___________|____________|__________|
//!  | field #2  |  field #2  |  field #2 |                       |
//!  |   index   |   length   |           |         ...           |
//!  | (2 bytes) |  (2 bytes) |   bytes   |                       |
//!  +___________|____________|___________|_______________________+
//!
//!
//! `Host Ctx` is for sharing data between the host and live apps.
//! It's an in-memory data that abstracts key-value pairs and thus its data-layout
//! isn't packed in order to simplify the job of `SVM` clients implementations.
//!
//!

use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use svm_types::HostCtx;

use byteorder::{BigEndian, ReadBytesExt};

use crate::error::ParseError;

pub fn decode_host_ctx(bytes: &[u8]) -> Result<HostCtx, ParseError> {
    let mut cursor = Cursor::new(bytes);

    let version = parse_version(&mut cursor);
    assert_eq!(version, 0);

    let mut fields = HashMap::new();

    let field_count = parse_field_count(&mut cursor);

    for _ in 0..field_count {
        let index = parse_field_index(&mut cursor);
        let length = parse_field_len(&mut cursor);
        let bytes = parse_field_bytes(&mut cursor, length);

        fields.insert(index as u32, bytes);
    }

    Ok(fields.into())
}

fn parse_version(cursor: &mut Cursor<&[u8]>) -> u32 {
    cursor.read_u32::<BigEndian>().unwrap()
}

fn parse_field_count(cursor: &mut Cursor<&[u8]>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

fn parse_field_index(cursor: &mut Cursor<&[u8]>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

fn parse_field_len(cursor: &mut Cursor<&[u8]>) -> u16 {
    cursor.read_u16::<BigEndian>().unwrap()
}

fn parse_field_bytes(cursor: &mut Cursor<&[u8]>, field_len: u16) -> Vec<u8> {
    let mut buf = vec![0; field_len as usize];

    cursor.read_exact(&mut buf[..]).unwrap();

    buf
}
