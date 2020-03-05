use crate::{
    error::ParseError,
    raw::{helpers, Field, NibbleIter, NibbleWriter},
    types::{App, SpawnApp, WasmValue},
};

use svm_common::Address;

pub fn encode_spawn_app(
    version: u32,
    template: &Address,
    ctor_idx: u16,
    ctor_buf: &[u8],
    ctor_args: &[WasmValue],
) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    encode_version(version, &mut w);
    encode_template(template, &mut w);
    encode_ctor_index(ctor_idx, &mut w);
    encode_ctor_buf(ctor_buf, &mut w);
    encode_ctor_args(ctor_args, &mut w);

    helpers::bytes(&mut w)
}

/// Parsing a raw `spawn-app` transaction given as raw bytes.
/// Returns the parsed transaction as a tuple consisting of an `App` struct and `ctor` buffer args.
/// On failure, returns `ParseError`
#[must_use]
pub fn decode_spawn_app(bytes: &[u8], creator: &Address) -> Result<SpawnApp, ParseError> {
    let mut iter = NibbleIter::new(bytes);

    let version = decode_version(&mut iter)?;
    let template = decode_template(&mut iter)?;
    let ctor_idx = decode_ctor_index(&mut iter)?;
    let ctor_buf = decode_ctor_buf(&mut iter)?;
    let ctor_args = decode_ctor_args(&mut iter)?;

    helpers::ensure_eof(&mut iter);

    let app = App {
        version,
        template,
        creator: creator.clone(),
    };

    let spawn_app = SpawnApp {
        app,
        ctor_idx,
        ctor_buf,
        ctor_args,
    };

    Ok(spawn_app)
}

/// Encoders

fn encode_version(version: u32, w: &mut NibbleWriter) {
    helpers::encode_version(version, w);
}

fn encode_template(template: &Address, w: &mut NibbleWriter) {
    helpers::encode_address(template, w);
}

fn encode_ctor_index(ctor_idx: u16, writer: &mut NibbleWriter) {
    helpers::encode_varuint14(ctor_idx, writer);
}

fn encode_ctor_buf(ctor_buf: &[u8], writer: &mut NibbleWriter) {
    helpers::encode_func_buf(ctor_buf, writer);
}

fn encode_ctor_args(args: &[WasmValue], writer: &mut NibbleWriter) {
    helpers::encode_func_args(args, writer);
}

/// Decoders

fn decode_version(iter: &mut NibbleIter) -> Result<u32, ParseError> {
    helpers::decode_version(iter)
}

fn decode_template(iter: &mut NibbleIter) -> Result<Address, ParseError> {
    helpers::decode_address(iter, Field::AppTemplate)
}

fn decode_ctor_index(iter: &mut NibbleIter) -> Result<u16, ParseError> {
    helpers::decode_varuint14(iter, Field::FuncIndex)
}

fn decode_ctor_buf(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    helpers::decode_func_buf(iter)
}

fn decode_ctor_args(iter: &mut NibbleIter) -> Result<Vec<WasmValue>, ParseError> {
    helpers::decode_func_args(iter)
}
