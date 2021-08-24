use svm_types::Gas;

use crate::{Codec, Field, ParseError, ReadExt, WriteExt};

impl Codec for Gas {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_u64_be(self.unwrap_or(0));
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        match cursor.read_u64_be() {
            Ok(0) => Ok(Gas::new()),
            Ok(gas) => Ok(Gas::with(gas)),
            Err(..) => Err(ParseError::NotEnoughBytes(Field::GasUsed)),
        }
    }
}
