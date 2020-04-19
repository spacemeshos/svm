use crate::schema::{Var, VarType};

pub struct VarRenderer;

impl VarRenderer {
    pub fn render(var: &Var, bytes: &[u8]) -> Option<String> {
        match var.ty {
            VarType::Int(..) => Self::render_int(var, bytes),
            VarType::Bool => Self::render_bool(var, bytes),
            VarType::Blob => Self::render_blob(var, bytes),
            VarType::String => Self::render_str(var, bytes),
            VarType::Balance => Self::render_balance(var, bytes),
            VarType::Address => Self::render_addr(var, bytes),
            VarType::PubKey => Self::render_pubkey(var, bytes),
        }
    }

    fn render_int(var: &Var, bytes: &[u8]) -> Option<String> {
        todo!()
    }

    fn render_str(var: &Var, bytes: &[u8]) -> Option<String> {
        todo!()
    }

    fn render_bool(var: &Var, bytes: &[u8]) -> Option<String> {
        todo!()
    }

    fn render_balance(var: &Var, bytes: &[u8]) -> Option<String> {
        todo!()
    }

    fn render_addr(var: &Var, bytes: &[u8]) -> Option<String> {
        todo!()
    }

    fn render_pubkey(var: &Var, bytes: &[u8]) -> Option<String> {
        todo!()
    }

    fn render_blob(var: &Var, bytes: &[u8]) -> Option<String> {
        todo!()
    }
}
