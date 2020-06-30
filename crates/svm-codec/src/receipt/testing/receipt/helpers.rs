use crate::api::raw::{self, Field};
use crate::nibble::{Nibble, NibbleIter};

use svm_types::{Address, State};

pub(crate) fn decode_is_success(iter: &mut NibbleIter) -> u8 {
    let is_success: Nibble = iter.next().unwrap();
    is_success.inner()
}

pub(crate) fn decode_receipt_error(iter: &mut NibbleIter) -> String {
    let len = raw::decode_varuint14(iter, Field::ErrorLength).unwrap();
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

pub(crate) fn decode_gas_used(iter: &mut NibbleIter) -> u64 {
    raw::decode_gas_used(iter).unwrap()
}
