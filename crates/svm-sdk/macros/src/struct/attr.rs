use proc_macro2::TokenStream;

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Error, Ident, Result};

use crate::Struct;

#[derive(Debug, PartialEq)]
pub enum StructAttrKind {
    Storage,

    Other,
}

#[derive(Debug)]
pub enum StructAttr {
    Storage,

    Other(TokenStream),
}

impl StructAttr {
    pub fn kind(&self) -> StructAttrKind {
        match self {
            StructAttr::Storage => StructAttrKind::Storage,
            StructAttr::Other(..) => StructAttrKind::Other,
        }
    }
}

pub fn struct_attrs(strukt: &Struct) -> Result<Vec<StructAttr>> {
    let mut attrs = Vec::new();

    for attr in strukt.raw_attrs() {
        let attr = parse_attr(attr)?;

        attrs.push(attr);
    }

    Ok(attrs)
}

pub fn parse_attr(attr: Attribute) -> Result<StructAttr> {
    let kind = parse_attr_kind(&attr)?;

    let attr = match kind {
        StructAttrKind::Storage => {
            assert!(attr.tokens.is_empty());

            StructAttr::Storage
        }
        StructAttrKind::Other => StructAttr::Other(quote! { #attr }),
    };

    Ok(attr)
}

fn parse_attr_kind(attr: &Attribute) -> Result<StructAttrKind> {
    let mut tokens = TokenStream::new();

    let path = &attr.path;
    path.to_tokens(&mut tokens);

    syn::parse2::<StructAttrKind>(tokens)
}

impl Parse for StructAttrKind {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let ident_str = format!("{}", ident);
        let ident_str = ident_str.as_str();

        let kind = match ident_str {
            "storage" => StructAttrKind::Storage,
            _ => StructAttrKind::Other,
        };

        Ok(kind)
    }
}

pub fn has_storage_attr(attrs: &[StructAttr]) -> bool {
    has_attr(attrs, StructAttrKind::Storage)
}

pub fn has_attr(attrs: &[StructAttr], kind: StructAttrKind) -> bool {
    attrs.iter().any(|attr| attr.kind() == kind)
}
