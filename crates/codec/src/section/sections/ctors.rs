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

use crate::{Codec, ParseError, ReadExt, WriteExt};

impl Codec for CtorsSection {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        // `#Ctors`
        let count = self.ctors().len();

        assert!(count < std::u8::MAX as usize);

        w.write_byte(count as u8);

        // Encoding each `Ctor`
        for ctor in self.ctors().iter() {
            ctor.encode(w);
        }
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, ParseError> {
        let num_ctors = cursor.read_byte()? as usize;
        let mut section = CtorsSection::with_capacity(num_ctors);

        for _ in 0..num_ctors {
            let ctor = String::decode(cursor)?;
            section.push(ctor);
        }

        Ok(section)
    }
}
