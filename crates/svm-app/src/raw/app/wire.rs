use crate::{
    raw::{helpers, NibbleWriter},
    types::WasmValue,
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

    write_version(version, &mut w);
    write_template(template, &mut w);
    write_ctor_index(ctor_idx, &mut w);
    write_ctor_buf(ctor_buf, &mut w);
    write_ctor_args(ctor_args, &mut w);

    helpers::bytes(&mut w)
}

fn write_version(version: u32, w: &mut NibbleWriter) {
    helpers::encode_version(version, w);
}

fn write_template(template: &Address, w: &mut NibbleWriter) {
    helpers::encode_address(template, w);
}

fn write_ctor_index(ctor_idx: u16, writer: &mut NibbleWriter) {
    helpers::encode_varuint14(ctor_idx, writer);
}

fn write_ctor_buf(ctor_buf: &[u8], writer: &mut NibbleWriter) {
    helpers::encode_func_buf(ctor_buf, writer);
}

fn write_ctor_args(args: &[WasmValue], writer: &mut NibbleWriter) {
    helpers::encode_func_args(args, writer);
}
