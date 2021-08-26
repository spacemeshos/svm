use svm_types::Gas;

use crate::{Codec, Field, ParseError, ReadExt, WriteExt};

impl Codec for Gas {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.unwrap_or(0).encode(w);
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        match u64::decode(cursor) {
            Ok(0) => Ok(Gas::new()),
            Ok(gas) => Ok(Gas::with(gas)),
            Err(..) => Err(ParseError::Eof(Field::GasUsed.to_string())),
        }
    }
}
