use std::io::Cursor;

use svm_types::gas::MaybeGas;

use crate::error::ParseError;

macro_rules! invalid_layout {
    () => {{
        use crate::api::raw::Field;

        return Err(ParseError::UnexpectedLayout(Field::GasUsed));
    }};
}

/// Decodes the `gas_used` field of a `Receipt`.
#[allow(unused)]
pub fn decode_gas_used(cursor: &mut Cursor<&[u8]>) -> Result<MaybeGas, ParseError> {
    unimplemented!()
}
