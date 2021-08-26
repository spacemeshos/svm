use crate::{Codec, ParseError, ReadExt, WriteExt};

pub fn encode_version(version: u16, w: &mut impl WriteExt) {
    version.encode(w);
}

pub fn decode_version(cursor: &mut impl ReadExt) -> Result<u16, ParseError> {
    Ok(u16::decode(cursor)?)
}
