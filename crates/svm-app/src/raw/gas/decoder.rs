use super::super::{wasm, NibbleIter};

use crate::{error::ParseError, types::WasmValue};

macro_rules! invalid_layout {
    () => {{
        use crate::raw::Field;

        return Err(ParseError::UnexpectedLayout(Field::GasUsed));
    }};
}

/// Decodes the `gas_used` field of a `Receipt`.
#[allow(unused)]
pub fn decode_gas_used(iter: &mut NibbleIter) -> Result<u64, ParseError> {
    let nib = iter.next();

    let layout = if let Some(nib) = nib {
        nib.into()
    } else {
        invalid_layout!()
    };

    let value = wasm::decode_wasm_value(&layout, iter)?;

    if let WasmValue::I64(gas_used) = value {
        Ok(gas_used)
    } else {
        invalid_layout!()
    }
}
