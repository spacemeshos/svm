use crate::{
    error::ParseError,
    raw::{helpers, Field, NibbleIter, NibbleWriter},
    types::{AppTransaction, WasmValue},
};

use svm_common::Address;

pub fn encode_exec_app(
    version: u32,
    app: &Address,
    func_idx: u16,
    func_buf: &[u8],
    func_args: &[WasmValue],
) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    encode_version(version, &mut w);
    encode_app(app, &mut w);
    encode_func_index(func_idx, &mut w);
    encode_func_buf(func_buf, &mut w);
    encode_func_args(func_args, &mut w);

    helpers::bytes(&mut w)
}

/// Parsing a raw `AppTransaction` transaction given as raw bytes.
/// Returns the parsed transaction as a `AppTransaction` struct.
/// On failure, returns `ParseError`.
#[must_use]
pub fn decode_exec_app(bytes: &[u8], sender: &Address) -> Result<AppTransaction, ParseError> {
    let mut iter = NibbleIter::new(bytes);

    let version = decode_version(&mut iter)?;
    let app = decode_app(&mut iter)?;
    let func_idx = decode_func_index(&mut iter)?;
    let func_buf = decode_func_buf(&mut iter)?;
    let func_args = decode_func_args(&mut iter)?;

    helpers::ensure_eof(&mut iter);

    let tx = AppTransaction {
        app,
        sender: sender.clone(),
        func_idx,
        func_args,
        func_buf,
    };

    Ok(tx)
}

/// Encoders

fn encode_version(version: u32, w: &mut NibbleWriter) {
    helpers::encode_version(version, w);
}

fn encode_app(app: &Address, w: &mut NibbleWriter) {
    helpers::encode_address(app, w);
}

fn encode_func_index(func_idx: u16, w: &mut NibbleWriter) {
    helpers::encode_varuint14(func_idx, w);
}

fn encode_func_buf(buf: &[u8], w: &mut NibbleWriter) {
    helpers::encode_func_buf(buf, w)
}

fn encode_func_args(buf: &[WasmValue], w: &mut NibbleWriter) {
    helpers::encode_func_args(&buf[..], w);
}

/// Decoders

fn decode_version(iter: &mut NibbleIter) -> Result<u32, ParseError> {
    helpers::decode_version(iter)
}

fn decode_app(iter: &mut NibbleIter) -> Result<Address, ParseError> {
    helpers::decode_address(iter, Field::App)
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
