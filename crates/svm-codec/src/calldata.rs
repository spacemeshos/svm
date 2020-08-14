use svm_nibble::{NibbleIter, NibbleWriter};

use crate::error::ParseError;

pub fn encode_calldata(value: &[u8], w: &mut NibbleWriter) {
    todo!()
}

pub fn decode_calldata<'a>(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    todo!()
}
