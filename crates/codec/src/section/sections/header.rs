//!
//! # `Header Section`
//!
//! +----------------+--------------+----------------+
//! |                |              |                |
//! |  Code Version  |    Name      |   Description  |
//! |  (4 bytes)     |   (String)   |    (String)    |  
//! |                |              |                |
//! +----------------+--------------+----------------+
//!
//!

use std::io::Cursor;

use svm_types::HeaderSection;

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Field, ParseError, ReadExt, WriteExt};

impl SectionEncoder for HeaderSection {
    fn encode(&self, w: &mut Vec<u8>) {
        // `Code Version`
        encode_code_version(self.code_version(), w);

        // `Name`
        encode_name(self.name(), w);

        // `Description`
        encode_desc(self.desc(), w);
    }
}

fn encode_code_version(code_ver: u32, w: &mut Vec<u8>) {
    w.write_u32_be(code_ver);
}

fn encode_name(name: &str, w: &mut Vec<u8>) {
    w.write_string(name);
}

fn encode_desc(desc: &str, w: &mut Vec<u8>) {
    w.write_string(desc);
}

impl SectionDecoder for HeaderSection {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        let code_version = decode_code_version(cursor)?;
        let name = decode_name(cursor)?;
        let desc = decode_desc(cursor)?;

        let section = HeaderSection::new(code_version, name, desc);

        Ok(section)
    }
}

fn decode_code_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParseError> {
    let value = cursor.read_u32_be();

    value.map_err(|_| ParseError::NotEnoughBytes(Field::CodeVersion))
}

fn decode_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(name)) => Ok(name),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Name)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Name)),
    }
}

fn decode_desc(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(desc)) => Ok(desc),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Description)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Description)),
    }
}
