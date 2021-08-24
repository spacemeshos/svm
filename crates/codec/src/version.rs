use crate::{Field, ParseError, ReadExt, WriteExt};

pub fn encode_version(version: u16, w: &mut impl WriteExt) {
    w.write_u16_be(version);
}

pub fn decode_version(cursor: &mut impl ReadExt) -> Result<u16, ParseError> {
    cursor
        .read_u16_be()
        .map_err(|_| ParseError::NotEnoughBytes(Field::Version))
}
