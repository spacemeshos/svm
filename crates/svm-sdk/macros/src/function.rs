use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{Attribute, ItemFn, Result, Signature};

use crate::FuncAttribute;

pub struct Function {
    raw: ItemFn,
}

pub struct Param {
    //
}

pub struct Return {
    //
}

impl Function {
    pub fn new(raw: ItemFn) -> Self {
        Self { raw }
    }

    pub fn attrs(&self) -> &[FuncAttribute] {
        todo!()
    }

    pub fn name(&self) -> Ident {
        self.raw_sig().ident.clone()
    }

    pub fn params(&self) -> &[Param] {
        todo!()
    }

    pub fn returns(&self) -> &[Return] {
        todo!()
    }

    pub fn raw_sig(&self) -> &Signature {
        &self.raw.sig
    }

    pub fn raw_sig_mut(&mut self) -> &mut Signature {
        &mut self.raw.sig
    }

    pub fn take_raw_attrs(&mut self) -> Vec<Attribute> {
        std::mem::replace(&mut self.raw.attrs, Vec::new())
    }
}

impl ToTokens for Function {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ast = quote! {
            //
        };

        tokens.extend(ast);
    }
}

fn endpoint_tokens(func: &Function) -> TokenStream {
    todo!()
}

fn fundable_tokens(func: &Function) -> TokenStream {
    todo!()
}

fn before_fund_tokens(func: &Function) -> TokenStream {
    todo!()
}

fn pure_tokens(func: &Function) -> TokenStream {
    func.raw.to_token_stream()
}

pub fn func_attrs(func: &mut Function) -> Result<Vec<FuncAttribute>> {
    todo!()
}

fn map_raw_attrs(func: &mut Function) -> Result<Vec<FuncAttribute>> {
    let mut attrs = Vec::new();

    for raw_attr in func.take_raw_attrs() {
        // let attr: FuncAttribute = syn::parse2(raw_attr.tokens)?;
    }

    Ok(attrs)
}
