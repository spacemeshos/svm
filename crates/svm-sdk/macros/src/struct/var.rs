use proc_macro2::{Ident, TokenStream};

use quote::{quote, ToTokens};

pub enum Var {
    Primitive {
        id: VarId,
        name: Ident,
        ty: Ident,
    },
    Array {
        id: VarId,
        name: Ident,
        ty: Ident,
        length: u32,
    },
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
