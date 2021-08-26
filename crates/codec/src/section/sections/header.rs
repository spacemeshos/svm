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

use svm_types::HeaderSection;

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Codec, Field, ParseError, ReadExt};

impl SectionEncoder for HeaderSection {
    fn encode(&self, w: &mut Vec<u8>) {
        self.code_version().encode(w);
        self.name().to_string().encode(w);
        self.desc().to_string().encode(w);
    }
}

impl SectionDecoder for HeaderSection {
    fn decode(reader: &mut impl ReadExt) -> Result<Self, ParseError> {
        let code_version = decode_code_version(reader)?;
        let name = String::decode(reader)?;
        let desc = String::decode(reader)?;

        let section = HeaderSection::new(code_version, name, desc);

        Ok(section)
    }
}

fn decode_code_version(cursor: &mut impl ReadExt) -> Result<u32, ParseError> {
    let value = u32::decode(cursor);

    value.map_err(|_| ParseError::Eof(Field::CodeVersion.to_string()))
}
