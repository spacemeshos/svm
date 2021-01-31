use std::io::Cursor;

use svm_types::gas::MaybeGas;

use crate::{Field, ParseError, ReadExt, WriteExt};

pub fn encode_gas_used(gas: &MaybeGas, w: &mut Vec<u8>) {
    let gas = gas.unwrap_or(0);

    w.write_u64_be(gas);
}

pub fn decode_gas_used(cursor: &mut Cursor<&[u8]>) -> Result<MaybeGas, ParseError> {
    match cursor.read_u64_be() {
        Ok(gas) => Ok(MaybeGas::with(gas)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::GasUsed)),
    }
}
