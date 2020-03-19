use crate::error::ParseError;

use super::super::{decode_varuint14, Field, NibbleIter};

pub fn decode_func_buf(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    let buf_len = decode_varuint14(iter, Field::FuncBufLength)? as usize;

    let bytes = iter.read_bytes(buf_len);

    if bytes.len() != buf_len {
        return Err(ParseError::NotEnoughBytes(Field::FuncBuf));
    }

    debug_assert_eq!(buf_len, bytes.len());

    Ok(bytes)
}
