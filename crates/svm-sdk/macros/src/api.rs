use crate::{Export, Schema, Signature, Var};

use serde_json::{json, Value};

pub fn json_api(schema: &Schema) -> Value {
    let exports = emit_exports(schema);
    let storage = emit_storage(schema);

    json!({"exports": exports, "storage": storage})
}

fn emit_exports(schema: &Schema) -> Value {
    let exports = schema
        .exports()
        .iter()
        .map(|e| {
            json!({
                "is_ctor": e.is_ctor,
                "is_fundable": e.is_fundable,
                "api_name": e.api_name,
                "wasm_name": e.wasm_name,
                "signature": emit_signature(e)
            })
        })
        .collect();

    Value::Array(exports)
}

fn emit_signature(e: &Export) -> Value {
    let sig = &e.signature;

    let mut params: Vec<Value> = sig
        .params()
        .iter()
        .map(|(name, ty)| {
            json!({
                "name": name,
                "type": ty
            })
        })
        .collect();

    let mut returns: Vec<Value> = sig
        .returns()
        .iter()
        .map(|ty| json!({ "type": ty }))
        .collect();

    json!({"params": params, "returns": returns})
}

fn emit_storage(schema: &Schema) -> Value {
    let vars = schema
        .storage()
        .iter()
        .map(|v| match v {
            Var::Primitive { .. } => emit_primitive_var(v),
            Var::Array { .. } => emit_array_var(v),
        })
        .collect();

    Value::Array(vars)
}

fn emit_primitive_var(var: &Var) -> Value {
    if let Var::Primitive {
        id,
        offset,
        name,
        byte_count,
        ty_str,
        ..
    } = var
    {
        json!({
            "id": id.0,
            "offset": offset,
            "name": name.to_string(),
            "type": ty_str,
            "byte_count": byte_count
        })
    } else {
        unreachable!()
    }
}

fn emit_array_var(var: &Var) -> Value {
    if let Var::Array {
        id,
        offset,
        name,
        byte_count,
        ty_str,
        length,
        ..
    } = var
    {
        json!({
            "id": id.0,
            "offset": offset,
            "name": name.to_string(),
            "type": format!("[{}]", ty_str),
            "length": length,
            "byte_count": byte_count
        })
    } else {
        unreachable!()
    }
}
