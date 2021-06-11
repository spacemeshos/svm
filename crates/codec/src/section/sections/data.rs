use std::io::Cursor;

use svm_layout::{FixedLayout, LayoutBuilder};
use svm_types::DataSection;

use crate::{Field, ParseError, ReadExt, WriteExt};

use super::SectionEncoder;

impl SectionEncoder for DataSection {
    fn encode(&self, w: &mut Vec<u8>) {
        let layout = self.fixed_layout();
        let var_count = layout.count();

        assert!(var_count < std::u16::MAX as usize);

        w.write_u16_be(var_count as u16);

        for var in layout.iter() {
            w.write_u16_be(var.byte_size() as u16);
        }
    }

    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        match cursor.read_u16_be() {
            Err(..) => Err(ParseError::NotEnoughBytes(Field::RawVarCount)),
            Ok(nvars) => {
                let mut builder = LayoutBuilder::with_capacity(nvars as usize);

                for _vid in 0..nvars as usize {
                    match cursor.read_u16_be() {
                        Err(..) => return Err(ParseError::NotEnoughBytes(Field::RawVarSize)),
                        Ok(length) => builder.push(length as u32),
                    }
                }

                let fixed = builder.build();
                let section = DataSection::new(fixed);

                Ok(section)
            }
        }
    }
}
