use super::{
    nibble::{NibbleIter, NibbleWriter},
    Field,
};
use crate::{error::ParseError, types::WasmValue};

use svm_common::Address;

/// Encoders

pub fn encode_func_buf(buf: &[u8], writer: &mut NibbleWriter) {
    todo!()
}

pub fn encode_func_args(args: &[WasmValue], writer: &mut NibbleWriter) {
    todo!()
}

pub fn encode_version(version: u32, writer: &mut NibbleWriter) {
    todo!()
}

pub fn encode_varuint14(num: u16, writer: &mut NibbleWriter) {
    todo!()
}

pub fn encode_address(addr: &Address, writer: &mut NibbleWriter) {
    todo!()
}

pub fn encode_string(s: &str, writer: &mut NibbleWriter) {
    todo!()
}

/// Decoders

#[must_use]
pub fn decode_func_buf(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    todo!();
}

#[must_use]
pub fn decode_func_args(iter: &mut NibbleIter) -> Result<Vec<WasmValue>, ParseError> {
    todo!()
}

#[must_use]
pub fn decode_version(iter: &mut NibbleIter) -> Result<u32, ParseError> {
    todo!()
}

pub fn decode_varuint14(version: u32, writer: &mut NibbleWriter) {
    todo!()
}

#[must_use]
pub fn decode_address(iter: &mut NibbleIter, field: Field) -> Result<Address, ParseError> {
    todo!();
}

#[must_use]
pub fn decode_string(iter: &mut NibbleIter, field: Field) -> Result<String, ParseError> {
    todo!()
}
