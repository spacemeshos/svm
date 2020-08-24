use svm_nibble::{Nibble, NibbleIter, NibbleWriter};

use crate::api::raw::{self, Field};
use crate::error::ParseError;

use svm_types::{
    gas::MaybeGas,
    receipt::{Log, Receipt, ReceiptError},
    Address, State, WasmValue,
};

/// Encoders

pub(crate) fn encode_version(version: u32, w: &mut NibbleWriter) {
    raw::encode_version(version, w)
}

pub(crate) fn encode_is_success(receipt: &Receipt, w: &mut NibbleWriter) {
    let nib = if receipt.is_success() {
        Nibble::new(1)
    } else {
        Nibble::new(0)
    };

    w.write(&[nib])
}

pub(crate) fn encode_gas_used(receipt: &Receipt, w: &mut NibbleWriter) {
    let gas_used = receipt.get_gas_used();

    raw::encode_gas_used(&gas_used, w);
}

pub(crate) fn encode_type(ty: u8, w: &mut NibbleWriter) {
    w.write_byte(ty);
}

pub(crate) fn encode_returns(returns: &[u8], w: &mut NibbleWriter) {
    raw::encode_calldata(returns, w)
}

pub(crate) fn encode_addr(addr: &Address, w: &mut NibbleWriter) {
    let bytes = addr.as_slice();
    w.write_bytes(bytes)
}

pub(crate) fn encode_state(state: &State, w: &mut NibbleWriter) {
    let bytes = state.as_slice();
    w.write_bytes(bytes)
}

/// Decoders

pub(crate) fn decode_version(iter: &mut NibbleIter) -> Result<u32, ParseError> {
    raw::decode_version(iter)
}

pub(crate) fn decode_type(iter: &mut NibbleIter) -> u8 {
    iter.read_byte()
}

pub(crate) fn decode_is_success(iter: &mut NibbleIter) -> u8 {
    let is_success: Nibble = iter.next().unwrap();
    is_success.inner()
}

pub(crate) fn decode_state(iter: &mut NibbleIter) -> State {
    let bytes = iter.read_bytes(State::len());

    State::from(&bytes[..])
}

pub(crate) fn decode_address(iter: &mut NibbleIter) -> Address {
    let bytes = iter.read_bytes(Address::len());

    Address::from(&bytes[..])
}

pub(crate) fn decode_gas_used(iter: &mut NibbleIter) -> MaybeGas {
    raw::decode_gas_used(iter).unwrap()
}
