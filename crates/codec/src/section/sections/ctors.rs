//!
//! # `Ctors Section`
//!
//! +-----------+------------+---------+
//! |           |            |         |
//! |  #Ctors   |  Ctor #1   |   ...   |
//! | (1 byte)  |  (String)  |         |
//! |           |            |         |
//! +-----------+------------+---------+
//!
//!

use std::io::Cursor;

use svm_types::{CtorsSection, Section, SectionKind};

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Field, ParseError, ReadExt, WriteExt};

impl SectionEncoder for CtorsSection {
    fn encode(&self, w: &mut Vec<u8>) {
        // `#Ctors`
        let count = self.ctors().len();

        assert!(count < std::u8::MAX as usize);

        w.write_byte(count as u8);

        // Encoding each `Ctor`
        for ctor in self.ctors().iter() {
            w.write_string(ctor);
        }
    }
}

impl SectionDecoder for CtorsSection {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        // Decoding each `Ctor`
        match cursor.read_byte() {
            Err(..) => Err(ParseError::NotEnoughBytes(Field::CtorsCount)),
            Ok(count) => {
                // `Ctors`
                let mut section = CtorsSection::with_capacity(count as usize);

                for _ in 0..count {
                    if let Ok(Ok(ctor)) = cursor.read_string() {
                        section.push(ctor);
                    } else {
                        return Err(ParseError::NotEnoughBytes(Field::Ctor));
                    }
                }

                Ok(section)
            }
        }
    }
}
