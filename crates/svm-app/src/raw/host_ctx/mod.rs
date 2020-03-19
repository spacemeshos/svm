//!             `Host Ctx` Raw Format Version 0.0.0.0
//!  --------------------------------------------------------------
//!  |   proto    |           | field #1  |  field #1  | field #1 |
//!  |  version   |  #fields  |   index   |   length   |          |
//!  | (4 bytes)  | (2 bytes) | (2 bytes) | (2 bytes)  |   bytes  |
//!  |____________|___________|___________|____________|__________|
//!  | field #2  |  field #2  |  field #2 |                       |
//!  |   index   |   length   |           |         ...           |
//!  | (2 bytes) |  (2 bytes) |   bytes   |                       |
//!  |___________|____________|___________|_______________________|
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

use crate::types::HostCtx;

use byteorder::{BigEndian, ReadBytesExt};

impl HostCtx {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub unsafe fn from_raw_parts(bytes: *const u8, length: u32) -> Result<HostCtx, String> {
        let bytes = std::slice::from_raw_parts(bytes as _, length as usize);

        let mut cursor = Cursor::new(bytes);

        Self::parse_version(&mut cursor);

        let mut fields = HashMap::new();

        let field_count = Self::parse_field_count(&mut cursor);

        for _ in 0..field_count {
            let index = Self::parse_field_index(&mut cursor);
            let length = Self::parse_field_len(&mut cursor);
            let bytes = Self::parse_field_bytes(&mut cursor, length);

            fields.insert(index as u32, bytes);
        }

        Ok(fields.into())
    }

    fn parse_version(cursor: &mut Cursor<&[u8]>) {
        let version = cursor.read_u32::<BigEndian>().unwrap();
        assert_eq!(0, version);
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
}
