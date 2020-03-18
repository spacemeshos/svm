use svm_app::{
    raw::{decode_varuint14, NibbleIter},
    types::WasmValue,
};
use svm_common::{Address, State};

use crate::svm_value_type;

pub(crate) fn decode_receipt_error(iter: &mut NibbleIter) -> String {
    let len = decode_varuint14(iter).unwrap();
    let bytes = iter.read_bytes(len as usize);

    String::from_utf8(bytes).unwrap()
}

pub(crate) fn decode_state(iter: &mut NibbleIter) -> State {
    let bytes = iter.read_bytes(State::len());

    State::from(&bytes[..])
}

pub(crate) fn decode_address(iter: &mut NibbleIter) -> Address {
    let bytes = iter.read_bytes(Address::len());

    Address::from(&bytes[..])
}

pub(crate) fn wasm_values_str(values: &[WasmValue]) -> String {
    let mut buf = String::new();

    for (i, v) in values.iter().enumerate() {
        if i != 0 {
            buf.push_str(", ");
        }
        buf.push_str(&format!("{:?}", v));
    }

    buf
}
