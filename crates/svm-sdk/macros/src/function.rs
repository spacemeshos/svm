use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{Attribute, ItemFn, Result, Signature};

use crate::{attr, FuncAttrKind, FuncAttribute};

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

fn pure_tokens(func: &Function) -> TokenStream {
    func.raw.to_token_stream()
}

pub fn func_attrs(func: &mut Function) -> Result<Vec<FuncAttribute>> {
    let mut attrs = Vec::new();

    for attr in func.take_raw_attrs() {
        let attr = attr::parse_attr(attr)?;

        attrs.push(attr);
    }

    Ok(attrs)
}

fn rewrite_func(func: &mut Function) -> Result<TokenStream> {
    let attrs = func_attrs(func)?;

    validate_attrs(&attrs);

    let ast = quote! {
        //
    };

    Ok(ast)
}

fn validate_attrs(attrs: &[FuncAttribute]) {
    let mut seen_endpoint = false;
    let mut seen_fundable = false;
    let mut seen_before_fund = false;

    for attr in attrs {
        match attr.kind() {
            FuncAttrKind::Endpoint => {
                if seen_endpoint {
                    panic!("Each function can be annotated with `#[endpoint]` exactly once.")
                }
                seen_endpoint = true;
            }
            FuncAttrKind::BeforeFund => {
                if seen_before_fund {
                    panic!("Each function can be annotated with `#[before_fund]` exactly once.")
                }
                seen_before_fund = true;
            }
            FuncAttrKind::Fundable => {
                if seen_fundable {
                    panic!("Each function can be annotated with `#[fundable]` exactly once.")
                }
                seen_fundable = true;
            }
            FuncAttrKind::Other => continue,
        }
    }
}
