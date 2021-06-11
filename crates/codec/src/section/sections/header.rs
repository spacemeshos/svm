use std::io::Cursor;

use svm_types::{HeaderSection, Template};

use crate::{Field, ParseError, WriteExt};

use super::SectionEncoder;

impl SectionEncoder for HeaderSection {
    fn encode(&self, w: &mut Vec<u8>) {
        todo!()
    }

    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        todo!()
    }
}

fn encode_name(template: &Template, w: &mut Vec<u8>) {
    todo!()
    // w.write_string(template.name());
}

fn decode_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    todo!()
    // match cursor.read_string() {
    //     Ok(Ok(name)) => Ok(name),
    //     Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Name)),
    //     Err(..) => Err(ParseError::NotEnoughBytes(Field::Name)),
    // }
}
