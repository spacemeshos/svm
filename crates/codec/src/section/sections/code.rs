use std::io::Cursor;

use svm_types::{ByteCodeKind, CodeSection};

use crate::{Field, ParseError, ReadExt, WriteExt};

use super::SectionEncoder;

impl SectionEncoder for CodeSection {
    fn encode(&self, w: &mut Vec<u8>) {
        let code = self.code();

        // code length
        let length = code.len();
        assert!(length < std::u32::MAX as usize);

        w.write_u32_be(length as u32);

        // code
        w.write_bytes(code);
    }

    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, crate::ParseError> {
        todo!("decode the bytecode kind...");

        match cursor.read_u32_be() {
            Err(..) => Err(ParseError::NotEnoughBytes(Field::Code)),
            Ok(length) => match cursor.read_bytes(length as usize) {
                Ok(code) => {
                    let section = CodeSection {
                        code,
                        kind: ByteCodeKind::Wasm,
                    };

                    Ok(section)
                }
                Err(..) => Err(ParseError::NotEnoughBytes(Field::Code)),
            },
        }
    }
}
