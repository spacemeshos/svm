use svm_types::ApiSection;

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{ParseError, ReadExt};

impl SectionEncoder for ApiSection {
    fn encode(&self, _w: &mut Vec<u8>) {
        todo!("will be implemented in a future PR...");
    }
}

impl SectionDecoder for ApiSection {
    fn decode(_cursor: &mut impl ReadExt) -> Result<Self, ParseError> {
        todo!("will be implemented in a future PR...");
    }
}
