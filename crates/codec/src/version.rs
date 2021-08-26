use crate::{Codec, Field, ParseError, ReadExt, WriteExt};

pub fn encode_version(version: u16, w: &mut impl WriteExt) {
    version.encode(w);
}

pub fn decode_version(cursor: &mut impl ReadExt) -> Result<u16, ParseError> {
    u16::decode(cursor).map_err(|_| ParseError::Eof(Field::Version.to_string()))
}
