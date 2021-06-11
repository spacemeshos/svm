use std::io::Cursor;

use svm_types::DeploySection;

use crate::ParseError;

use super::SectionEncoder;

impl SectionEncoder for DeploySection {
    fn encode(&self, w: &mut Vec<u8>) {
        todo!()
    }

    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        todo!()
    }
}
