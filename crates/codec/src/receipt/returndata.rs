use std::io::{Cursor, Result};

use crate::{ReadExt, WriteExt};

pub(crate) fn encode(returndata: &[u8], w: &mut impl WriteExt) {
    let byte_size = returndata.len();
    assert!(byte_size < std::u16::MAX as usize);

    w.write_u16_be(byte_size as u16);
    w.write_bytes(returndata);
}

pub(crate) fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>> {
    let byte_size = cursor.read_u16_be()?;

    cursor.read_bytes(byte_size as usize)
}
