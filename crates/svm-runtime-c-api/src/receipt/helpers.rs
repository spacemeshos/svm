use std::io::Read;

use byteorder::{BigEndian, WriteBytesExt};

use svm_app::{
    raw::{self, Nibble, NibbleWriter},
    types::WasmValue,
};
use svm_common::{Address, State};
use svm_runtime::receipt::Receipt;

use crate::svm_value_type;

pub(crate) fn encode_is_success(receipt: &Receipt, w: &mut NibbleWriter) {
    let nib = if receipt.is_success() {
        Nibble::new(1)
    } else {
        Nibble::new(0)
    };

    w.write(&[nib])
}

pub(crate) fn encode_returns(receipt: &Receipt, w: &mut NibbleWriter) {
    debug_assert!(receipt.is_success());

    let returns = receipt.get_returns();
    raw::encode_func_args(returns, w)
}

pub(crate) fn encode_addr(addr: &Address, w: &mut NibbleWriter) {
    let bytes = addr.as_slice();
    w.write_bytes(bytes)
}

pub(crate) fn encode_state(state: &State, w: &mut NibbleWriter) {
    let bytes = state.as_slice();
    w.write_bytes(bytes)
}
