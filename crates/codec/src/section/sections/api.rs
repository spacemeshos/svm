use std::io::Cursor;

use svm_types::ApiSection;

use crate::ParseError;

use crate::section::{SectionDecoder, SectionEncoder};

impl SectionEncoder for ApiSection {
    fn encode(&self, w: &mut Vec<u8>) {
        todo!("will be implemented in a future PR...");
    }
}

impl SectionDecoder for ApiSection {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        todo!("will be implemented in a future PR...");
    }
}
