use std::fmt;

use crate::{PrimType, Type};

use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

pub enum Var {
    Primitive {
        id: VarId,
        offset: usize,
        name: Ident,
        ty: PrimType,
        byte_count: usize,
    },
    Array {
        id: VarId,
        offset: usize,
        name: Ident,
        elem_ty: PrimType,
        length: u32,
        byte_count: usize,
    },
}

impl Var {
    pub fn byte_count(&self) -> usize {
        match *self {
            Var::Primitive { byte_count, .. } => byte_count,
            Var::Array { byte_count, .. } => byte_count,
        }
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Var::Primitive { id, name, ty, .. } => {
                writeln!(f, "Var #{} - {}: {}", id.0, name, ty.as_str())
            }
            Var::Array {
                id,
                name,
                elem_ty,
                length,
                ..
            } => writeln!(
                f,
                "Var #{} - {}: [{}; {}]",
                id.0,
                name,
                elem_ty.as_str(),
                length
            ),
        }
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub struct VarId(pub u32);

impl ToTokens for &VarId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        tokens.extend(quote! { #id });
    }
}
