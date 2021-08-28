use svm_types::ApiSection;

use crate::{Codec, ParseError, ReadExt, WriteExt};

impl Codec for ApiSection {
    type Error = ParseError;

    fn encode(&self, _w: &mut impl WriteExt) {
        todo!("will be implemented in a future PR...");
    }

    fn decode(_cursor: &mut impl ReadExt) -> Result<Self, ParseError> {
        todo!("will be implemented in a future PR...");
    }
}
