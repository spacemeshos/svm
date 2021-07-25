use std::io::Write;

use crate::{Export, PrimType, Program, Signature, Type, Var};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde_json::{json, Value};

pub fn json_api(schema: &Program) -> Value {
    let exports = exports_api(schema);
    let storage = storage_api(schema);

    json!({"exports": exports, "storage": storage})
}

pub fn json_data_layout(schema: &Program) -> Value {
    let data: Vec<usize> = schema
        .storage()
        .iter()
        .fold(Vec::new(), |mut acc, v| match v {
            Var::Primitive { byte_count, .. } => {
                acc.push(*byte_count);
                acc
            }
            Var::Array {
                byte_count, length, ..
            } => {
                acc.extend(vec![*byte_count; *length as usize]);
                acc
            }
        });

    json!({ "data": data })
}

pub fn json_write(file_name: &str, json: &Value) {
    let bytes = serde_json::to_vec(json).unwrap();

    std::fs::write(file_name, bytes);
}

pub fn json_tokenstream(json: &Value) -> TokenStream {
    let json = json.to_string();

    quote! { #json }
}

fn exports_api(schema: &Program) -> Value {
    let exports = schema
        .exports()
        .map(|e| {
            json!({
                "is_ctor": e.is_ctor,
                "is_fundable": e.is_fundable,
                "api_name": e.api_name,
                "wasm_name": e.export_name,
                "doc": e.doc,
                "signature": emit_signature(e)
            })
        })
        .collect();

    Value::Array(exports)
}

fn emit_signature(e: &Export) -> Value {
    let sig = &e.signature;

    let mut params: Vec<Value> = sig.params().iter().map(emit_param).collect();
    let mut returns = emit_output(sig.output());

    json!({"params": params, "returns": returns})
}

fn emit_param(param: &(String, Type)) -> Value {
    let name = &param.0;
    let ty = &param.1;

    match ty {
        Type::Primitive(prim) => json!({"name": name, "type": prim.as_str()}),
        Type::Array {
            elem_ty: elem,
            length,
            ..
        } => {
            json!({"name": name, "type": format!("[{}]", elem.as_str()), "length": length})
        }
        Type::Tuple { .. } => unreachable!(),
    }
}

fn emit_output(ty: Option<&Type>) -> Value {
    if let Some(ty) = ty {
        match ty {
            Type::Primitive(..) | Type::Array { .. } => emit_output_type(ty),
            Type::Tuple { elems, .. } => {
                let elems = elems.iter().map(|ty| emit_output_type(&*ty)).collect();

                Value::Array(elems)
            }
        }
    } else {
        json!({})
    }
}

fn emit_output_type(ty: &Type) -> Value {
    match ty {
        Type::Primitive(prim) => json!({ "type": prim.as_str() }),
        Type::Array {
            elem_ty: elem,
            length,
            ..
        } => {
            json!({ "type": elem.as_str(), "length": length })
        }
        Type::Tuple { .. } => unreachable!("Nested tuples are not allowed"),
    }
}

fn storage_api(schema: &Program) -> Value {
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
        ty,
        ..
    } = var
    {
        json!({
            "id": id.0,
            "offset": offset,
            "name": name.to_string(),
            "type": typify(ty),
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
        elem_ty,
        length,
        ..
    } = var
    {
        json!({
            "id": id.0,
            "offset": offset,
            "name": name.to_string(),
            "type": format!("[{}]", typify(elem_ty)),
            "length": length,
            "byte_count": byte_count
        })
    } else {
        unreachable!()
    }
}

fn typify(ty: &PrimType) -> String {
    match ty.as_str() {
        "svm_sdk :: Amount" => "Amount".to_string(),
        "svm_sdk :: Address" => "Address".to_string(),
        _ => ty.as_str().to_string(),
    }
}
