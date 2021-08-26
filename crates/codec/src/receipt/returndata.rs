use crate::{Codec, EofError, ReadExt, WriteExt};

pub(crate) fn encode(returndata: &[u8], w: &mut impl WriteExt) {
    let byte_size = returndata.len();
    assert!(byte_size < std::u16::MAX as usize);

    (byte_size as u16).encode(w);
    w.write_bytes(returndata);
}

pub(crate) fn decode(reader: &mut impl ReadExt) -> Result<Vec<u8>, EofError> {
    let byte_size = u16::decode(reader)?;

    reader.read_bytes(byte_size as usize)
}
