use std::io::Cursor;

use crate::{Field, ParseError, ReadExt, WriteExt};

pub fn encode_calldata(calldata: &[u8], w: &mut Vec<u8>) {
    let length = calldata.len();

    assert!(length <= std::u8::MAX as usize);

    w.write_byte(length as u8);
    w.write_bytes(calldata);
}

pub fn decode_calldata<'a>(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ParseError> {
    match cursor.read_byte() {
        Err(..) => Err(ParseError::NotEnoughBytes(Field::CallDataLength)),
        Ok(byte) => {
            let length = byte as usize;

            cursor
                .read_bytes(length)
                .map_err(|_| ParseError::NotEnoughBytes(Field::CallData))
        }
    }
}
