use crate::schema::{Var, VarType};

use serde_json::{Number, Value};

pub trait VarRenderer<V> {
    fn render(var: &Var, bytes: &[u8]) -> Option<V>;
}

pub struct JsonVarRenderer;

impl VarRenderer<Value> for JsonVarRenderer {
    /// Renders the variable's raw `bytes` using its metadata (using `var`).
    fn render(var: &Var, bytes: &[u8]) -> Option<Value> {
        match var.ty {
            VarType::Int => Self::render_int(var, bytes),
            VarType::Bool => Self::render_bool(var, bytes),
            VarType::Blob => Self::render_blob(var, bytes),
            VarType::Balance => Self::render_balance(var, bytes),
            VarType::Address => Self::render_addr(var, bytes),
            VarType::PubKey => Self::render_pubkey(var, bytes),
        }
    }
}

impl JsonVarRenderer {
    fn render_int(var: &Var, bytes: &[u8]) -> Option<Value> {
        let length = var.layout.length;

        if length > 8 {
            /// Currently, an integer can be at most 8 bytes
            return None;
        }

        let mut buf = [0; 8];

        unsafe {
            std::ptr::copy(bytes.as_ptr(), buf.as_mut_ptr(), length);
        }

        let num = u64::from_be_bytes(buf);

        Some(Value::Number(num.into()))
    }

    fn render_balance(_var: &Var, _bytes: &[u8]) -> Option<Value> {
        todo!()
    }

    fn render_bool(_var: &Var, bytes: &[u8]) -> Option<Value> {
        assert_eq!(bytes.len(), 1);

        let value = match bytes[0] {
            0 => false,
            1 => true,
            _ => return None,
        };

        Some(Value::Bool(value))
    }

    fn render_addr(_var: &Var, bytes: &[u8]) -> Option<Value> {
        Self::render_hex(bytes, "0x")
    }

    fn render_pubkey(_var: &Var, bytes: &[u8]) -> Option<Value> {
        Self::render_hex(bytes, "0x")
    }

    fn render_blob(_var: &Var, bytes: &[u8]) -> Option<Value> {
        Self::render_hex(bytes, "")
    }

    fn render_hex(bytes: &[u8], prefix: &'static str) -> Option<Value> {
        let s = hex::encode_upper(bytes);
        let s = format!("{}{}", prefix, s);

        Some(Value::String(s))
    }
}
