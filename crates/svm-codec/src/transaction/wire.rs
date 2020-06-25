use svm_types::{AppAddr, AppTransaction};

use crate::{
    decode_func_args, decode_func_buf, decode_varuint14, decode_version, error::ParseError,
    helpers, Field, NibbleIter, NibbleWriter,
};

/// Encodes a raw App transaction.
pub fn encode_exec_app(tx: &AppTransaction, w: &mut NibbleWriter) {
    encode_version(tx, w);
    encode_app(tx, w);
    encode_func_index(tx, w);
    encode_func_buf(tx, w);
    encode_func_args(tx, w);
}

/// Parsing a raw `AppTransaction` transaction given as raw bytes.
/// Returns the parsed transaction as a `AppTransaction` struct.
/// On failure, returns `ParseError`.
pub fn decode_exec_app(iter: &mut NibbleIter) -> Result<AppTransaction, ParseError> {
    let version = decode_version(iter)?;
    let app = decode_app(iter)?;
    let func_idx = decode_func_index(iter)?;
    let func_buf = decode_func_buf(iter)?;
    let func_args = decode_func_args(iter)?;

    let tx = AppTransaction {
        version,
        app,
        func_idx,
        func_args,
        func_buf,
    };

    Ok(tx)
}

/// Encoders

fn encode_version(tx: &AppTransaction, w: &mut NibbleWriter) {
    let ver = tx.version;
    crate::encode_version(ver, w);
}

fn encode_app(tx: &AppTransaction, w: &mut NibbleWriter) {
    let addr = tx.app.inner();
    helpers::encode_address(addr, w);
}

fn encode_func_index(tx: &AppTransaction, w: &mut NibbleWriter) {
    let idx = tx.func_idx;
    crate::encode_varuint14(idx, w);
}

fn encode_func_buf(tx: &AppTransaction, w: &mut NibbleWriter) {
    let buf = &tx.func_buf[..];
    crate::encode_func_buf(buf, w)
}

fn encode_func_args(tx: &AppTransaction, w: &mut NibbleWriter) {
    let args = &tx.func_args[..];
    crate::encode_func_args(args, w);
}

/// Decoders

fn decode_app(iter: &mut NibbleIter) -> Result<AppAddr, ParseError> {
    let addr = helpers::decode_address(iter, Field::App)?;

    Ok(addr.into())
}

fn decode_func_index(iter: &mut NibbleIter) -> Result<u16, ParseError> {
    decode_varuint14(iter, Field::FuncIndex)
}

#[cfg(test)]
mod tests {
    use svm_common::Address;
    use svm_types::{AppTransaction, WasmValue};

    use crate::{decode_exec_app, encode_exec_app, NibbleIter, NibbleWriter};

    #[test]
    fn encode_decode_exec_app() {
        let tx = AppTransaction {
            version: 0,
            app: Address::of("my-app").into(),
            func_idx: 0,
            func_buf: vec![0x10, 0x0, 0x30],
            func_args: vec![WasmValue::I32(20), WasmValue::I64(30)],
        };

        let mut w = NibbleWriter::new();
        encode_exec_app(&tx, &mut w);

        let bytes = w.into_bytes();
        let mut iter = NibbleIter::new(&bytes[..]);

        let decoded = decode_exec_app(&mut iter).unwrap();

        assert_eq!(tx, decoded);
    }
}
