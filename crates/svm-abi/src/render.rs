use crate::schema::{Var, VarType};

pub struct VarRenderer;

impl VarRenderer {
    /// Renders the variable's raw `bytes` using its metadata (using `var`).
    pub fn render(var: &Var, bytes: &[u8]) -> Option<String> {
        match var.ty {
            VarType::Int => Self::render_int(var, bytes),
            VarType::Bool => Self::render_bool(var, bytes),
            VarType::Blob => Self::render_blob(var, bytes),
            VarType::Balance => Self::render_balance(var, bytes),
            VarType::Address => Self::render_addr(var, bytes),
            VarType::PubKey => Self::render_pubkey(var, bytes),
        }
    }

    fn render_int(var: &Var, bytes: &[u8]) -> Option<String> {
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

        Some(num.to_string())
    }

    fn render_balance(_var: &Var, _bytes: &[u8]) -> Option<String> {
        todo!()
    }

    fn render_bool(_var: &Var, bytes: &[u8]) -> Option<String> {
        assert_eq!(bytes.len(), 1);

        match bytes[0] {
            0 => Some("False".to_string()),
            1 => Some("True".to_string()),
            _ => None,
        }
    }

    fn render_addr(_var: &Var, bytes: &[u8]) -> Option<String> {
        Self::render_hex(bytes, "0x")
    }

    fn render_pubkey(_var: &Var, bytes: &[u8]) -> Option<String> {
        Self::render_hex(bytes, "0x")
    }

    fn render_blob(_var: &Var, bytes: &[u8]) -> Option<String> {
        Self::render_hex(bytes, "")
    }

    fn render_hex(bytes: &[u8], prefix: &'static str) -> Option<String> {
        let s = hex::encode_upper(bytes);
        Some(format!("{}{}", prefix, s))
    }
}
