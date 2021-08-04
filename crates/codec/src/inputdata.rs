use std::io::Cursor;

use crate::{Field, ParseError, ReadExt, WriteExt};

pub fn encode_inputdata(data: &[u8], w: &mut Vec<u8>) {
    let length = data.len();

    assert!(length <= std::u8::MAX as usize);

    w.write_byte(length as u8);
    w.write_bytes(data);
}

pub fn decode_inputdata<'a>(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ParseError> {
    match cursor.read_byte() {
        Err(..) => Err(ParseError::NotEnoughBytes(Field::InputDataLength)),
        Ok(byte) => {
            let length = byte as usize;

            cursor
                .read_bytes(length)
                .map_err(|_| ParseError::NotEnoughBytes(Field::InputData))
        }
    }
}
