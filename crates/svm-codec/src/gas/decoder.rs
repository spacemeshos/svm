use svm_nibble::NibbleIter;
use svm_types::{gas::MaybeGas, WasmValue};

use crate::{error::ParseError, wasm};

macro_rules! invalid_layout {
    () => {{
        use crate::api::raw::Field;

        return Err(ParseError::UnexpectedLayout(Field::GasUsed));
    }};
}

/// Decodes the `gas_used` field of a `Receipt`.
#[allow(unused)]
pub fn decode_gas_used(iter: &mut NibbleIter) -> Result<MaybeGas, ParseError> {
    let nib = iter.next();

    let layout = if let Some(nib) = nib {
        nib.into()
    } else {
        invalid_layout!()
    };

    let value = wasm::decode_wasm_value(&layout, iter)?;

    if let WasmValue::I64(gas_used) = value {
        let gas_used = MaybeGas::with(gas_used);

        Ok(gas_used)
    } else {
        invalid_layout!()
    }
}
