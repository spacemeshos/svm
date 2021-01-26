use std::io::{Cursor, Read};

use crate::api::raw::{self, Field};
use crate::error::ParseError;

use svm_types::gas::MaybeGas;
use svm_types::receipt::{Log, Receipt, ReceiptError};
use svm_types::{Address, State};

/// Encoders

pub(crate) fn encode_version(version: u32, w: &mut Vec<u8>) {
    raw::encode_version(version, w)
}

pub(crate) fn encode_is_success(receipt: &Receipt, w: &mut Vec<u8>) {
    let nib = if receipt.is_success() {
        w.push(1);
    } else {
        w.push(0);
    };
}

pub(crate) fn encode_gas_used(receipt: &Receipt, w: &mut Vec<u8>) {
    let gas_used = receipt.get_gas_used();

    raw::encode_gas_used(&gas_used, w);
}

pub(crate) fn encode_type(ty: u8, w: &mut Vec<u8>) {
    w.push(ty);
}

pub(crate) fn encode_abi_data(returns: &[u8], w: &mut Vec<u8>) {
    raw::encode_calldata(returns, w)
}

pub(crate) fn encode_addr(addr: &Address, w: &mut Vec<u8>) {
    let bytes = addr.as_slice();

    w.extend_from_slice(bytes)
}

pub(crate) fn encode_state(state: &State, w: &mut Vec<u8>) {
    let bytes = state.as_slice();

    w.extend_from_slice(bytes)
}

/// Decoders

pub(crate) fn decode_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParseError> {
    raw::decode_version(cursor)
}

pub(crate) fn decode_type(cursor: &mut Cursor<&[u8]>) -> Result<u8, ParseError> {
    let mut buf = [0; 1];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::ReceiptType));
    }

    Ok(buf[0])
}

pub(crate) fn decode_is_success(cursor: &mut Cursor<&[u8]>) -> Result<u8, ParseError> {
    let mut buf = [0; 1];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::ReceiptStatus));
    }

    Ok(buf[0])
}

pub(crate) fn decode_state(cursor: &mut Cursor<&[u8]>) -> Result<State, ParseError> {
    let mut buf = [0; State::len()];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::State));
    }

    let state = State::from(&buf[..]);
    Ok(state)
}

pub(crate) fn decode_address(cursor: &mut Cursor<&[u8]>) -> Result<Address, ParseError> {
    let mut buf = [0; Address::len()];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::Address));
    }

    let addr = Address::from(&buf[..]);
    Ok(addr)
}

pub(crate) fn decode_gas_used(cursor: &mut Cursor<&[u8]>) -> Result<MaybeGas, ParseError> {
    raw::decode_gas_used(cursor)
}
