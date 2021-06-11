use std::io::Cursor;

use svm_types::CtorsSection;

use crate::{Field, ParseError, ReadExt, WriteExt};

use super::SectionEncoder;

impl SectionEncoder for CtorsSection {
    fn encode(&self, w: &mut Vec<u8>) {
        let count = self.ctors().len();

        assert!(count < std::u8::MAX as usize);

        w.write_byte(count as u8);

        for ctor in self.ctors().iter() {
            w.write_string(ctor);
        }
    }

    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        match cursor.read_byte() {
            Err(..) => Err(ParseError::NotEnoughBytes(Field::CtorsCount)),
            Ok(count) => {
                let mut ctors = Vec::with_capacity(count as usize);

                for _ in 0..count {
                    if let Ok(Ok(ctor)) = cursor.read_string() {
                        ctors.push(ctor);
                    } else {
                        return Err(ParseError::NotEnoughBytes(Field::Ctor));
                    }
                }

                let section = CtorsSection { ctors };

                Ok(section)
            }
        }
    }
}
