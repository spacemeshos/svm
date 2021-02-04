use std::io::Cursor;

use crate::{Field, ParseError, ReadExt, WriteExt};

pub fn encode_version(version: u16, w: &mut Vec<u8>) {
    w.write_u16_be(version);
}

pub fn decode_version(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    cursor
        .read_u16_be()
        .map_err(|_| ParseError::NotEnoughBytes(Field::Version))
}
