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

use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use crate::api::raw;
use crate::error::ParseError;

use svm_types::HostCtx;

use byteorder::{BigEndian, ByteOrder, ReadBytesExt};

pub fn encode_host_ctx(host_ctx: &HostCtx) -> Vec<u8> {
    let map = host_ctx.inner();

    let nvalues = map.values().len();
    let values_size = map.values().fold(0, |acc, v| acc + v.len());
    let buf_size = 4 + 2 + values_size + nvalues * 2;

    let mut buf: Vec<u8> = vec![0; buf_size];

    // `version`
    BigEndian::write_u32(&mut buf, 0);

    let nfields = map.len();
    assert!(nfields <= std::u16::MAX as usize);

    // `#fields`
    BigEndian::write_u16(&mut buf, nfields as u16);

    for (k, v) in map.iter() {
        assert!(*k <= std::u16::MAX as u32);

        // `field index`
        BigEndian::write_u16(&mut buf, *k as u16);

        // `field value`
        buf.extend_from_slice(v);
    }

    buf
}

pub fn decode_host_ctx(bytes: &[u8]) -> Result<HostCtx, ParseError> {
    let mut cursor = Cursor::new(bytes);

    let version = decode_version(&mut cursor)?;
    assert_eq!(version, 0);

    let mut fields = HashMap::new();

    let field_count = decode_field_count(&mut cursor)?;

    for _ in 0..field_count {
        let index = decode_field_index(&mut cursor)?;
        let length = decode_field_length(&mut cursor)?;
        let bytes = decode_field_value(&mut cursor, length)?;

        fields.insert(index as u32, bytes);
    }

    Ok(fields.into())
}

fn decode_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParseError> {
    let version = cursor.read_u32::<BigEndian>().unwrap();
    Ok(version)
}

fn decode_field_count(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    let nfields = cursor.read_u16::<BigEndian>().unwrap();
    Ok(nfields)
}

fn decode_field_index(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    let index = cursor.read_u16::<BigEndian>().unwrap();
    Ok(index)
}

fn decode_field_length(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    let length = cursor.read_u16::<BigEndian>().unwrap();
    Ok(length)
}

fn decode_field_value(cursor: &mut Cursor<&[u8]>, field_len: u16) -> Result<Vec<u8>, ParseError> {
    let mut buf = vec![0; field_len as usize];

    cursor.read_exact(&mut buf[..]).unwrap();

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    use maplit::hashmap;

    macro_rules! assert_decode {
        ($buf:expr, $expected:expr) => {{
            let actual = decode_host_ctx(&$buf).unwrap();

            assert_eq!(actual, $expected);
        }};
    }

    #[test]
    fn encode_host_ctx_empty() {
        let map = hashmap! {};

        let host_ctx: HostCtx = map.into();
        let buf = encode_host_ctx(&host_ctx);

        assert_decode!(buf, host_ctx);
    }

    #[test]
    fn encode_host_ctx_one_field() {
        let map = hashmap! {
            0 => vec![0x10, 0x20],
        };

        let host_ctx: HostCtx = map.into();
        let buf = encode_host_ctx(&host_ctx);

        assert_decode!(buf, host_ctx);
    }
}
