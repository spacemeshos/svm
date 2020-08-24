use svm_nibble::{NibbleIter, NibbleWriter};

use crate::api::raw::{decode_varuint14, encode_varuint14, Field};
use crate::error::ParseError;

pub fn encode_calldata(calldata: &[u8], w: &mut NibbleWriter) {
    let len = calldata.len();

    assert!(len <= std::u16::MAX as usize);

    encode_varuint14(len as u16, w);

    w.write_bytes(calldata)
}

pub fn decode_calldata<'a>(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    let len = decode_varuint14(iter, Field::CallDataLength)? as usize;

    let bytes = iter.read_bytes(len);

    if bytes.len() != len {
        return Err(ParseError::NotEnoughBytes(Field::CallData));
    }

    debug_assert_eq!(len, bytes.len());

    Ok(bytes)
}
