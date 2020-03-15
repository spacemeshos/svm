use crate::{
    error::ParseError,
    raw::{helpers, Field, NibbleIter, NibbleWriter},
    types::{AppAddr, AppTransaction, WasmValue},
};

use svm_common::Address;

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
#[must_use]
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
    let ver = *&tx.version;

    helpers::encode_version(ver, w);
}

fn encode_app(tx: &AppTransaction, w: &mut NibbleWriter) {
    let addr = tx.app.inner();
    helpers::encode_address(addr, w);
}

fn encode_func_index(tx: &AppTransaction, w: &mut NibbleWriter) {
    let idx = *&tx.func_idx;
    helpers::encode_varuint14(idx, w);
}

fn encode_func_buf(tx: &AppTransaction, w: &mut NibbleWriter) {
    let buf = &tx.func_buf[..];
    helpers::encode_func_buf(buf, w)
}

fn encode_func_args(tx: &AppTransaction, w: &mut NibbleWriter) {
    let args = &tx.func_args[..];
    helpers::encode_func_args(args, w);
}

/// Decoders

fn decode_version(iter: &mut NibbleIter) -> Result<u32, ParseError> {
    helpers::decode_version(iter)
}

fn decode_app(iter: &mut NibbleIter) -> Result<AppAddr, ParseError> {
    let addr = helpers::decode_address(iter, Field::App)?;

    Ok(addr.into())
}

fn decode_func_index(iter: &mut NibbleIter) -> Result<u16, ParseError> {
    helpers::decode_varuint14(iter, Field::FuncIndex)
}

fn decode_func_buf(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    helpers::decode_func_buf(iter)
}

fn decode_func_args(iter: &mut NibbleIter) -> Result<Vec<WasmValue>, ParseError> {
    helpers::decode_func_args(iter)
}
