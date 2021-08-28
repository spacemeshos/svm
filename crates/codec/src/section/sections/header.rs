use svm_types::HeaderSection;

use crate::{Codec, ParseError, ReadExt, WriteExt};

///
/// # `Header Section`
///
/// +----------------+--------------+----------------+
/// |                |              |                |
/// |  Code Version  |    Name      |   Description  |
/// |  (4 bytes)     |   (String)   |    (String)    |  
/// |                |              |                |
/// +----------------+--------------+----------------+
///
///
impl Codec for HeaderSection {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        (self.code_version() as u32).encode(w);
        self.name().to_string().encode(w);
        self.desc().to_string().encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, ParseError> {
        let code_version = u32::decode(reader)?;
        let name = String::decode(reader)?;
        let desc = String::decode(reader)?;

        Ok(HeaderSection::new(code_version, name, desc))
    }
}
