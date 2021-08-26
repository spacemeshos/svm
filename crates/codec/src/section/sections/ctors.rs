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

use svm_types::CtorsSection;

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Codec, Field, ParseError, ReadExt, WriteExt};

impl SectionEncoder for CtorsSection {
    fn encode(&self, w: &mut Vec<u8>) {
        // `#Ctors`
        let count = self.ctors().len();

        assert!(count < std::u8::MAX as usize);

        w.write_byte(count as u8);

        // Encoding each `Ctor`
        for ctor in self.ctors().iter() {
            ctor.encode(w);
        }
    }
}

impl SectionDecoder for CtorsSection {
    fn decode(cursor: &mut impl ReadExt) -> Result<Self, ParseError> {
        // Decoding each `Ctor`
        match cursor.read_byte() {
            Err(..) => Err(ParseError::Eof(Field::CtorsCount.to_string())),
            Ok(count) => {
                // `Ctors`
                let mut section = CtorsSection::with_capacity(count as usize);

                for _ in 0..count {
                    if let Ok(ctor) = String::decode(cursor) {
                        section.push(ctor);
                    } else {
                        return Err(ParseError::Eof(Field::Ctor.to_string()));
                    }
                }

                Ok(section)
            }
        }
    }
}
