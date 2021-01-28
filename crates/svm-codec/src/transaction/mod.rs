//! Execute `AppTransaction` Raw Format Version 0.0
//!
//!  +--------------------------------------------+
//!  |             |                              |
//!  |  version    |          `AppAddress`        |
//!  |             |           (Address)          |
//!  |_____________|______________________________|
//!  |                                            |
//!  |            Function (String)               |
//!  |____________________________________________|
//!  |              |                             |
//!  |  `Calldata`  |       `Calldata`            |
//!  |   #length    |         (blob)              |
//!  |____________  |_____________________________|
//!
//!

use std::io::{Cursor, Read};

use svm_types::{AppAddr, AppTransaction};

use crate::api::raw;
use crate::{Field, ParseError, ReadExt, WriteExt};

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
    let version = raw::decode_version(cursor)?;
    let app = decode_app(cursor)?;
    let func_name = decode_func(cursor)?;
    let calldata = raw::decode_calldata(cursor)?;

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

    w.write_address(addr);
}

fn encode_func(tx: &AppTransaction, w: &mut Vec<u8>) {
    let func = &tx.func_name;

    w.write_string(func);
}

fn encode_calldata(tx: &AppTransaction, w: &mut Vec<u8>) {
    let calldata = &tx.calldata;

    raw::encode_calldata(calldata, w)
}

/// Decoders

fn decode_app(cursor: &mut Cursor<&[u8]>) -> Result<AppAddr, ParseError> {
    match cursor.read_address() {
        Ok(addr) => Ok(addr.into()),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::AppAddr)),
    }
}

fn decode_func(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(func)) => Ok(func),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Function)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Function)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::Address;

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

        let mut cursor = Cursor::new(&bytes[..]);
        let decoded = decode_exec_app(&mut cursor).unwrap();

        assert_eq!(tx, decoded);
    }
}
