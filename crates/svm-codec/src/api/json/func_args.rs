use svm_nibble::{NibbleIter, NibbleWriter};
use svm_types::WasmValue;

use serde_json::{json, Value};

use crate::{
    api::json::{self, JsonError},
    api::raw,
};

pub fn encode_func_args(args: &[String]) -> Result<Vec<u8>, JsonError> {
    let json = json!({ "args": args });
    let args = json::as_wasm_values(&json, "args")?;

    let mut w = NibbleWriter::new();
    raw::encode_func_args(&args, &mut w);

    Ok(w.into_bytes())
}

pub fn decode_func_args(func_args: &str) -> Result<Vec<String>, JsonError> {
    let data = json::str_to_bytes(&func_args, "func_args")?;

    let mut iter = NibbleIter::new(&data);
    let func_args: Vec<_> = raw::decode_func_args(&mut iter)
        .unwrap()
        .iter()
        .map(|v| match v {
            WasmValue::I32(v) => format!("{}i32", v),
            WasmValue::I64(v) => format!("{}i64", v),
        })
        .collect();

    Ok(func_args)
}
