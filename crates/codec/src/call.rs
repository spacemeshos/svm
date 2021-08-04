//! Encoding of binary [`Transaction`].
//!
//! ```text
//!
//!  +-----------+-------------+----------------+
//!  |           |             |                |
//!  |  Version  |  Template   |      Name      |
//!  |   (u16)   |  (Address)  |    (String)    |
//!  |           |             |                |
//!  +-----------+-------------+----------------+
//!  |           |             |                |
//!  | Function  | VerifyData  |    CallData    |
//!  | (String)  |   (Blob)    |     (Blob)     |
//!  |           |             |                |
//!  +-----------+-------------+----------------+
//!
//! ```

use svm_types::{Address, Transaction};

use std::io::Cursor;

use crate::{inputdata, version};
use crate::{Field, ParseError, ReadExt, WriteExt};

/// Encodes a binary [`Transaction`]
pub fn encode_call(tx: &Transaction, w: &mut Vec<u8>) {
    encode_version(tx, w);
    encode_target(tx, w);
    encode_func(tx, w);
    encode_verifydata(tx, w);
    encode_calldata(tx, w);
}

/// Parsing a binary [`Transaction`].
///
/// Returns the parsed transaction as [`Transaction`] struct.
/// On failure, returns `ParseError`
pub fn decode_call(cursor: &mut Cursor<&[u8]>) -> Result<Transaction, ParseError> {
    let version = decode_version(cursor)?;
    let target = decode_target(cursor)?;
    let func_name = decode_func(cursor)?;
    let verifydata = inputdata::decode_inputdata(cursor)?;
    let calldata = inputdata::decode_inputdata(cursor)?;

    let tx = Transaction {
        version,
        target,
        func_name,
        verifydata,
        calldata,
    };

    Ok(tx)
}

/// Encoders

fn encode_version(tx: &Transaction, w: &mut Vec<u8>) {
    let v = &tx.version;

    version::encode_version(*v, w);
}

fn encode_target(tx: &Transaction, w: &mut Vec<u8>) {
    w.write_address(tx.target());
}

fn encode_func(tx: &Transaction, w: &mut Vec<u8>) {
    let func = tx.func_name();
    w.write_string(func);
}

fn encode_verifydata(tx: &Transaction, w: &mut Vec<u8>) {
    let verifydata = tx.verifydata();
    inputdata::encode_inputdata(verifydata, w)
}

fn encode_calldata(tx: &Transaction, w: &mut Vec<u8>) {
    let calldata = tx.calldata();
    inputdata::encode_inputdata(calldata, w)
}

/// Decoders

#[inline]
fn decode_version(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    version::decode_version(cursor)
}

fn decode_target(cursor: &mut Cursor<&[u8]>) -> Result<Address, ParseError> {
    cursor
        .read_address()
        .map_err(|_| ParseError::NotEnoughBytes(Field::TargetAddr))
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
    fn encode_decode_call() {
        let tx = Transaction {
            version: 0,
            target: Address::of("@target").into(),
            func_name: "do_work".to_string(),
            verifydata: vec![0xAA, 0xBB, 0xCC],
            calldata: vec![0x10, 0x0, 0x30],
        };

        let mut bytes = Vec::new();
        encode_call(&tx, &mut bytes);

        let mut cursor = Cursor::new(&bytes[..]);
        let decoded = decode_call(&mut cursor).unwrap();

        assert_eq!(tx, decoded);
    }
}
