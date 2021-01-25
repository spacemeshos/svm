use std::io::{Cursor, Read};

use svm_types::{AppAddr, AppTransaction};

use crate::api::raw::{decode_abi_data, decode_varuint14, decode_version, encode_abi_data, Field};

use crate::{error::ParseError, helpers};

/// Encodes a raw App transaction.
pub fn encode_exec_app(tx: &AppTransaction, w: &mut Vec<u8>) {
    encode_version(tx, w);
    encode_app(tx, w);
    encode_func(tx, w);
    encode_calldata(tx, w);
}

/// Parsing a raw `AppTransaction` transaction given as raw bytes.
/// Returns the parsed transaction as a `AppTransaction` struct.
/// On failure, returns `ParseError`.
pub fn decode_exec_app(cursor: &mut Cursor<&[u8]>) -> Result<AppTransaction, ParseError> {
    let version = decode_version(cursor)?;
    let app = decode_app(cursor)?;
    let func_name = decode_func(cursor)?;
    let calldata = decode_abi_data(cursor)?;

    let tx = AppTransaction {
        version,
        app,
        func_name,
        calldata,
    };

    Ok(tx)
}

/// Encoders

fn encode_version(tx: &AppTransaction, w: &mut Vec<u8>) {
    crate::api::raw::encode_version(tx.version, w);
}

fn encode_app(tx: &AppTransaction, w: &mut Vec<u8>) {
    let addr = tx.app.inner();

    helpers::encode_address(addr, w);
}

fn encode_func(tx: &AppTransaction, w: &mut Vec<u8>) {
    helpers::encode_string(&tx.func_name, w);
}

fn encode_calldata(tx: &AppTransaction, w: &mut Vec<u8>) {
    let calldata = &tx.calldata;

    encode_abi_data(calldata, w)
}

/// Decoders

fn decode_app(cursor: &mut Cursor<&[u8]>) -> Result<AppAddr, ParseError> {
    let addr = helpers::decode_address(cursor, Field::AppAddr)?;

    Ok(addr.into())
}

fn decode_func(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    helpers::decode_string(cursor, Field::FuncNameLength, Field::FuncName)
}

#[cfg(test)]
mod tests {
    use svm_nibble::NibbleIter;
    use svm_types::{Address, AppTransaction, WasmValue};

    use crate::api::raw::{decode_exec_app, encode_exec_app};

    #[test]
    fn encode_decode_exec_app() {
        let tx = AppTransaction {
            version: 0,
            app: Address::of("my-app").into(),
            func_name: "do_work".to_string(),
            calldata: vec![0x10, 0x0, 0x30],
        };

        let mut bytes = Vec::new();
        encode_exec_app(&tx, &mut bytes);

        let mut iter = NibbleIter::new(&bytes);
        let decoded = decode_exec_app(&mut iter).unwrap();

        assert_eq!(tx, decoded);
    }
}
