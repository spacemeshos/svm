//! Encoding for apps-transactions.
//!
//! `Transaction` Raw Format
//!
//!  +--------------------------------------------+
//!  |             |                              |
//!  |  version    |         `Address`            |
//!  |  (2 bytes)  |         (20 bytes)           |
//!  |_____________|______________________________|
//!  |                                            |
//!  |            `Function` (String)             |
//!  |____________________________________________|
//!  |                |                           |
//!  |  `VerifyData`  |       `VerifyData`        |
//!  |   #length      |          (blob)           |
//!  +________________|___________________________+
//!  |              |                             |
//!  |  `CallData`  |         `CallData`          |
//!  |   #length    |           (blob)            |
//!  +______________|_____________________________+
//!
//!

use svm_types::{AppAddr, Transaction};

use std::io::Cursor;

use crate::{calldata, version};
use crate::{Field, ParseError, ReadExt, WriteExt};

/// Encodes a raw App transaction.
pub fn encode_exec_app(tx: &Transaction, w: &mut Vec<u8>) {
    encode_version(tx, w);
    encode_app(tx, w);
    encode_func(tx, w);
    // encode_verifydata(tx, w);
    encode_calldata(tx, w);
}

/// Parsing a raw `AppTransaction` transaction given as raw bytes.
/// Returns the parsed transaction as a `AppTransaction` struct.
/// On failure, returns `ParseError`.
pub fn decode_exec_app(cursor: &mut Cursor<&[u8]>) -> Result<Transaction, ParseError> {
    let version = decode_version(cursor)?;
    let app = decode_app(cursor)?;
    let func_name = decode_func(cursor)?;
    // let verifydata = calldata::decode_calldata(cursor)?;
    let calldata = calldata::decode_calldata(cursor)?;

    let tx = Transaction {
        version,
        app,
        func_name,
        // verifydata,
        calldata,
    };

    Ok(tx)
}

/// Encoders

fn encode_version(tx: &Transaction, w: &mut Vec<u8>) {
    let v = &tx.version;

    version::encode_version(*v, w);
}

fn encode_app(tx: &Transaction, w: &mut Vec<u8>) {
    let addr = tx.app.inner();

    w.write_address(addr);
}

fn encode_func(tx: &Transaction, w: &mut Vec<u8>) {
    let func = &tx.func_name;

    w.write_string(func);
}

// fn encode_verifydata(tx: &Transaction, w: &mut Vec<u8>) {
//     let verifydata = &tx.verifydata;

//     calldata::encode_calldata(verifydata, w)
// }

fn encode_calldata(tx: &Transaction, w: &mut Vec<u8>) {
    let calldata = &tx.calldata;

    calldata::encode_calldata(calldata, w)
}

/// Decoders

#[inline]
fn decode_version(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    version::decode_version(cursor)
}

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
        let tx = Transaction {
            version: 0,
            app: Address::of("my-app").into(),
            func_name: "do_work".to_string(),
            // verifydata: vec![0x10, 0x0, 0x30],
            calldata: vec![0x10, 0x0, 0x30],
        };

        let mut bytes = Vec::new();
        encode_exec_app(&tx, &mut bytes);

        let mut cursor = Cursor::new(&bytes[..]);
        let decoded = decode_exec_app(&mut cursor).unwrap();

        assert_eq!(tx, decoded);
    }
}
