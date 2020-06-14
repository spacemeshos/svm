use svm_types::{App, SpawnApp, TemplateAddr, WasmValue};

use crate::{
    decode_func_args, decode_func_buf, decode_varuint14, decode_version, encode_func_args,
    encode_func_buf, encode_varuint14, helpers, Field, NibbleIter, NibbleWriter, ParseError,
};

/// Encodes a raw Spawn-App transaction.
pub fn encode_spawn_app(spawn: &SpawnApp, w: &mut NibbleWriter) {
    encode_version(spawn, w);
    encode_template(spawn, w);
    encode_ctor_index(spawn, w);
    encode_ctor_buf(spawn, w);
    encode_ctor_args(spawn, w);
}

/// Parsing a raw `spawn-app` transaction given as raw bytes.
/// Returns the parsed transaction as a tuple consisting of an `App` struct and `ctor` buffer args.
/// On failure, returns `ParseError`.
pub fn decode_spawn_app(iter: &mut NibbleIter) -> Result<SpawnApp, ParseError> {
    let version = decode_version(iter)?;
    let template = decode_template(iter)?;
    let ctor_idx = decode_ctor_index(iter)?;
    let ctor_buf = decode_ctor_buf(iter)?;
    let ctor_args = decode_ctor_args(iter)?;

    let app = App { version, template };

    let spawn = SpawnApp {
        app,
        ctor_idx,
        ctor_buf,
        ctor_args,
    };

    Ok(spawn)
}

/// Encoders

fn encode_version(spawn: &SpawnApp, w: &mut NibbleWriter) {
    let version = spawn.app.version;
    crate::encode_version(version, w);
}

fn encode_template(spawn: &SpawnApp, w: &mut NibbleWriter) {
    let template = &spawn.app.template;
    helpers::encode_address(template.inner(), w);
}

fn encode_ctor_index(spawn: &SpawnApp, w: &mut NibbleWriter) {
    let ctor_idx = spawn.ctor_idx;
    encode_varuint14(ctor_idx, w);
}

fn encode_ctor_buf(spawn: &SpawnApp, w: &mut NibbleWriter) {
    let ctor_buf = &*spawn.ctor_buf;
    encode_func_buf(ctor_buf, w);
}

fn encode_ctor_args(spawn: &SpawnApp, w: &mut NibbleWriter) {
    let args = &spawn.ctor_args;
    encode_func_args(args, w);
}

/// Decoders

fn decode_template(iter: &mut NibbleIter) -> Result<TemplateAddr, ParseError> {
    let addr = helpers::decode_address(iter, Field::AppTemplate)?;

    Ok(TemplateAddr::new(addr))
}

fn decode_ctor_index(iter: &mut NibbleIter) -> Result<u16, ParseError> {
    decode_varuint14(iter, Field::FuncIndex)
}

fn decode_ctor_buf(iter: &mut NibbleIter) -> Result<Vec<u8>, ParseError> {
    decode_func_buf(iter)
}

fn decode_ctor_args(iter: &mut NibbleIter) -> Result<Vec<WasmValue>, ParseError> {
    decode_func_args(iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_common::Address;

    #[test]
    fn encode_decode_spawn_app() {
        let spawn = SpawnApp {
            app: App {
                version: 0,
                template: Address::of("my-template").into(),
            },
            ctor_idx: 10,
            ctor_buf: vec![0x10, 0x20, 0x30],
            ctor_args: vec![WasmValue::I32(20), WasmValue::I64(30)],
        };

        let mut w = NibbleWriter::new();
        encode_spawn_app(&spawn, &mut w);

        let bytes = w.into_bytes();
        let mut iter = NibbleIter::new(&bytes[..]);

        let decoded = decode_spawn_app(&mut iter).unwrap();

        assert_eq!(spawn, decoded);
    }
}
