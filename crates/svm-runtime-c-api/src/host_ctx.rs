//!             `Host Ctx` Raw Format Version 0.0.0.0
//!  --------------------------------------------------------------
//!  |   proto    |           | field #1  |  field #1  | field #1 |
//!  |  version   |  #fields  |   index   |   length   |          |
//!  |  (4 bytes) | (2 bytes) | (2 bytes) | (2 bytes)  |   bytes  |
//!  |____________|___________|___________|____________|__________|
//!  | field #2  |  field #2  |  field #2 |                       |
//!  |   index   |   length   |           |          ...          |
//!  | (2 bytes) |  (2 bytes) |   bytes   |                       |
//!  |___________|____________|___________|_______________________|
//!

use byteorder::{BigEndian, ReadBytesExt};

use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{Cursor, Read};

pub unsafe fn parse_host_ctx(
    bytes: *const c_void,
    bytes_len: libc::c_uint,
) -> Result<HashMap<i32, Vec<u8>>, String> {
    let bytes = std::slice::from_raw_parts(bytes as _, bytes_len as usize);
    let mut cursor = Cursor::new(bytes);

    parse_version(&mut cursor);

    let mut fields = HashMap::new();

    let field_count = parse_field_count(&mut cursor);

    for _ in 0..field_count {
        let field_idx = parse_field_index(&mut cursor);
        let field_len = parse_field_len(&mut cursor);
        let field_bytes = parse_field_bytes(&mut cursor, field_len);

        fields.insert(field_idx as i32, field_bytes);
    }

    Ok(fields)
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

#[cfg(test)]
mod tests {
    use crate::testing::host_ctx;
    use maplit::hashmap;

    use super::*;

    #[test]
    fn parse_host_ctx_no_fields() {
        let mut bytes = Vec::new();

        host_ctx::write_version(&mut bytes, 0);
        host_ctx::write_field_count(&mut bytes, 0);

        let actual = unsafe { parse_host_ctx(bytes.as_ptr() as _, bytes.len() as _) };
        let expected = Ok(hashmap! {});

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_host_ctx_one_field() {
        let mut bytes = Vec::new();

        host_ctx::write_version(&mut bytes, 0);
        host_ctx::write_field_count(&mut bytes, 1);
        host_ctx::write_field(&mut bytes, 3, vec![10, 20, 30]);

        let actual = unsafe { parse_host_ctx(bytes.as_ptr() as _, bytes.len() as _) };
        let expected = Ok(hashmap! { 3 => vec![10, 20, 30] });

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_host_ctx_two_fields() {
        let mut bytes = Vec::new();

        host_ctx::write_version(&mut bytes, 0);
        host_ctx::write_field_count(&mut bytes, 2);
        host_ctx::write_field(&mut bytes, 3, vec![10, 20, 30]);
        host_ctx::write_field(&mut bytes, 5, vec![40, 50, 60, 70]);

        let actual = unsafe { parse_host_ctx(bytes.as_ptr() as _, bytes.len() as _) };
        let expected = Ok(hashmap! {
          3 => vec![10, 20, 30],
          5 => vec![40, 50, 60, 70]
        });

        assert_eq!(expected, actual);
    }
}
