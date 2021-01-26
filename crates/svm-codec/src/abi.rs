use std::io::{Cursor, Read};

use crate::api::raw::Field;
use crate::error::ParseError;

pub fn encode_abi_data(calldata: &[u8], w: &mut Vec<u8>) {
    let len = calldata.len();

    assert!(len <= std::u8::MAX as usize);

    w.push(len as u8);
    w.extend_from_slice(calldata);
}

pub fn decode_abi_data<'a>(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ParseError> {
    let mut buf = [0; 1];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::CallDataLength));
    }

    let len = buf[0] as usize;
    let mut buf = Vec::with_capacity(len);

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::CallData));
    }

    debug_assert_eq!(len, buf.len());

    Ok(buf)
}
