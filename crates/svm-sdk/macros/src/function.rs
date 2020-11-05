extern crate proc_macro;

use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::ParseStream;
use syn::{Attribute, ItemFn};

use crate::FuncAttribute;

pub struct Function {
    raw: ItemFn,
}

pub struct Param {
    //
}

impl Function {
    pub fn new(raw: ItemFn) -> Self {
        Self { raw }
    }

    pub fn name(&self) -> Ident {
        self.raw.sig.ident.clone()
    }

    pub fn params(&self) -> &[Param] {
        todo!()
    }

    pub fn attrs(&self) -> &[FuncAttribute] {
        todo!()
    }
}

pub enum FuncAttrs {
    Endpoint,

    BeforeFund,

    Fundable(String),

    Other(Attribute),
}
