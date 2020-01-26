//!             `Host Ctx` Raw Format Version 0.0.0.0
//!  --------------------------------------------------------------
//!  |   proto    |           | field #1  |  field #1  | field #1 |
//!  |  version   |  #fields  |   index   |   length   |          |
//!  |  (4 bytes) | (2 bytes) | (2 bytes) | (2 bytes)  |   bytes  |
//!  |____________|___________|___________|____________|__________|
//!  | field #2  |  field #2  |  field #2 |                       |
//!  |   index   |   length   |           |         ...           |
//!  | (2 bytes) |  (2 bytes) |   bytes   |                       |
//!  |___________|____________|___________|_______________________|
//!

use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{Cursor, Read};

use crate::types::HostCtx;

use byteorder::{BigEndian, ReadBytesExt};

impl HostCtx {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    #[inline]
    pub fn get(&self, field: u32) -> Option<&Vec<u8>> {
        self.inner.get(&field)
    }

    /// Parses a raw `host-context` into `HostCtx` struct.
    pub unsafe fn from_raw_parts(
        bytes: *const c_void,
        bytes_len: usize,
    ) -> Result<HostCtx, String> {
        let bytes = std::slice::from_raw_parts(bytes as _, bytes_len);
        let mut cursor = Cursor::new(bytes);

        Self::parse_version(&mut cursor);

        let mut fields = HashMap::new();

        let field_count = Self::parse_field_count(&mut cursor);

        for _ in 0..field_count {
            let field_idx = Self::parse_field_index(&mut cursor);
            let field_len = Self::parse_field_len(&mut cursor);
            let field_bytes = Self::parse_field_bytes(&mut cursor, field_len);

            fields.insert(field_idx as u32, field_bytes);
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
