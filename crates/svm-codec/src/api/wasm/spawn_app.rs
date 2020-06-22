use serde_json::{self as json, Value};

use svm_common::Address;
use svm_types::{App, SpawnApp, WasmValue};

use super::{
    alloc, error::into_error_buffer, free, to_wasm_buffer, wasm_buf_data_copy, wasm_buffer_data,
    BUF_OK_MARKER,
};
use crate::{app, NibbleWriter};

/// Encodes a `spawn-app` json input into SVM `spawn-app` binary transaction.
/// The json input is passed by giving WASM memory start address (`ptr` parameter).
///
/// Returns a pointer to a `transaction buffer`.
pub fn encode_spawn_app(ptr: usize) -> usize {
    let bytes = wasm_buffer_data(ptr);
    let json: json::Result<Value> = serde_json::from_slice(bytes);

    let template = Address::of("@template").into();
    let ctor_idx = 2;
    let ctor_buf = vec![0x10, 0x20, 0x30];
    let ctor_args = vec![WasmValue::I32(0x40), WasmValue::I64(0x50)];

    match json {
        Ok(json) => {
            let version: &Value = &json["version"];
            let version = version.as_u64().unwrap_or(0) as u32;

            let spawn = SpawnApp {
                app: App { version, template },
                ctor_idx,
                ctor_buf,
                ctor_args,
            };

            let mut w = NibbleWriter::new();
            w.write_bytes(&[BUF_OK_MARKER]);

            app::encode_spawn_app(&spawn, &mut w);

            let bytes = w.into_bytes();
            to_wasm_buffer(&bytes)
        }
        Err(err) => into_error_buffer(err),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::json;

    #[test]
    fn wasm_encode_spawn_app_valid() {
        let json: Value = json!({
          "Hello": "World",
        });

        let json = b"{}";

        let buf = to_wasm_buffer(json);
        let result = encode_spawn_app(buf);

        let tx = wasm_buffer_data(result);

        assert_eq!(tx[0], BUF_OK_MARKER);

        dbg!(&tx[1..]);

        // free(buf);
        // free(result);
    }

    #[test]
    fn wasm_encode_spawn_app_invalid_json() {
        //
    }
}
