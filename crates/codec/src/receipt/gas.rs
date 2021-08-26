use svm_types::Gas;

use crate::{Codec, ParseError, ReadExt, WriteExt};

impl Codec for Gas {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.unwrap_or(0).encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        match u64::decode(reader)? {
            0 => Ok(Gas::new()),
            x => Ok(Gas::with(x)),
        }
    }
}
