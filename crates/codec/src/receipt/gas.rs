use svm_types::Gas;

use std::io::Cursor;

use crate::{Field, ParseError, ReadExt, WriteExt};

pub fn encode_gas_used(gas: &Gas, w: &mut Vec<u8>) {
    let gas = gas.unwrap_or(0);

    w.write_u64_be(gas);
}

pub fn decode_gas_used(cursor: &mut Cursor<&[u8]>) -> Result<Gas, ParseError> {
    match cursor.read_u64_be() {
        Ok(gas) => Ok(Gas::with(gas)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::GasUsed)),
    }
}
