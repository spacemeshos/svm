use svm_codec::api::raw;
use svm_codec::nibble::{Nibble, NibbleWriter};
use svm_runtime::receipt::Receipt;
use svm_types::{Address, State, WasmValue};

pub(crate) fn encode_is_success(receipt: &Receipt, w: &mut NibbleWriter) {
    let nib = if receipt.is_success() {
        Nibble::new(1)
    } else {
        Nibble::new(0)
    };

    w.write(&[nib])
}

pub(crate) fn encode_gas_used(receipt: &Receipt, w: &mut NibbleWriter) {
    let maybe_gas = receipt.get_gas_used();
    let gas_used = maybe_gas.unwrap_or(0);

    raw::encode_gas_used(gas_used, w);
}

pub(crate) fn encode_version(version: u32, w: &mut NibbleWriter) {
    raw::encode_version(version, w);
}

pub(crate) fn encode_returns(returns: &[WasmValue], w: &mut NibbleWriter) {
    raw::encode_func_rets(returns, w)
}

pub(crate) fn encode_addr(addr: &Address, w: &mut NibbleWriter) {
    let bytes = addr.as_slice();
    w.write_bytes(bytes)
}

pub(crate) fn encode_state(state: &State, w: &mut NibbleWriter) {
    let bytes = state.as_slice();
    w.write_bytes(bytes)
}
