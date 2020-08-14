use svm_nibble::{NibbleIter, NibbleWriter};
use svm_types::{AppAddr, AppTransaction};

use crate::api::raw::{decode_calldata, decode_varuint14, decode_version, Field};

use crate::{error::ParseError, helpers};

/// Encodes a raw App transaction.
pub fn encode_exec_app(tx: &AppTransaction, w: &mut NibbleWriter) {
    encode_version(tx, w);
    encode_app(tx, w);
    encode_func_index(tx, w);
    encode_calldata(tx, w);
}

/// Parsing a raw `AppTransaction` transaction given as raw bytes.
/// Returns the parsed transaction as a `AppTransaction` struct.
/// On failure, returns `ParseError`.
pub fn decode_exec_app(iter: &mut NibbleIter) -> Result<AppTransaction, ParseError> {
    let version = decode_version(iter)?;
    let app = decode_app(iter)?;
    let func_idx = decode_func_index(iter)?;
    let calldata = decode_calldata(iter)?;

    let tx = AppTransaction {
        version,
        app,
        func_idx,
        calldata,
    };

    Ok(tx)
}

/// Encoders

fn encode_version(tx: &AppTransaction, w: &mut NibbleWriter) {
    let ver = tx.version;
    crate::api::raw::encode_version(ver, w);
}

fn encode_app(tx: &AppTransaction, w: &mut NibbleWriter) {
    let addr = tx.app.inner();
    helpers::encode_address(addr, w);
}

fn encode_func_index(tx: &AppTransaction, w: &mut NibbleWriter) {
    let idx = tx.func_idx;
    crate::api::raw::encode_varuint14(idx, w);
}

fn encode_calldata(tx: &AppTransaction, w: &mut NibbleWriter) {
    let buf = &tx.calldata[..];
    crate::api::raw::encode_calldata(buf, w)
}

/// Decoders

fn decode_app(iter: &mut NibbleIter) -> Result<AppAddr, ParseError> {
    let addr = helpers::decode_address(iter, Field::AppAddr)?;

    Ok(addr.into())
}

fn decode_func_index(iter: &mut NibbleIter) -> Result<u16, ParseError> {
    decode_varuint14(iter, Field::FuncIndex)
}

#[cfg(test)]
mod tests {
    use svm_nibble::{NibbleIter, NibbleWriter};
    use svm_types::{Address, AppTransaction, WasmValue};

    use crate::api::raw::{decode_exec_app, encode_exec_app};

    #[test]
    fn encode_decode_exec_app() {
        let tx = AppTransaction {
            version: 0,
            app: Address::of("my-app").into(),
            func_idx: 0,
            calldata: vec![0x10, 0x0, 0x30],
        };

        let mut w = NibbleWriter::new();
        encode_exec_app(&tx, &mut w);

        let bytes = w.into_bytes();
        let mut iter = NibbleIter::new(&bytes[..]);

        let decoded = decode_exec_app(&mut iter).unwrap();

        assert_eq!(tx, decoded);
    }
}
