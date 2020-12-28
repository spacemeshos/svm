use proc_macro2::TokenStream;

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Error, Ident, Result};

use crate::Struct;

#[derive(Debug, PartialEq)]
pub enum StructFieldAttrKind {
    ByteCount,

    Other,
}

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

#[derive(Debug)]
pub enum StructFieldAttr {
    ByteCount(usize),

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
impl StructFieldAttr {
    pub fn kind(&self) -> StructFieldAttrKind {
        match self {
            StructFieldAttr::ByteCount(..) => StructFieldAttrKind::ByteCount,
            StructFieldAttr::Other(..) => StructFieldAttrKind::Other,
        }
    }
}

pub fn struct_attrs(raw_attrs: &[Attribute]) -> Result<Vec<StructAttr>> {
    let mut attrs = Vec::new();

    for attr in raw_attrs {
        let attr = parse_struct_attr(attr)?;

        attrs.push(attr);
    }

    Ok(attrs)
}

pub fn struct_field_attrs(raw_attrs: &[Attribute]) -> Result<Vec<StructFieldAttr>> {
    let mut attrs = Vec::new();

    for attr in raw_attrs {
        let attr = parse_struct_field_attr(attr)?;

        attrs.push(attr);
    }

    Ok(attrs)
}

pub fn parse_struct_attr(attr: &Attribute) -> Result<StructAttr> {
    let kind = parse_struct_attr_kind(&attr)?;

    let attr = match kind {
        StructAttrKind::Storage => {
            assert!(attr.tokens.is_empty());

            StructAttr::Storage
        }
        StructAttrKind::Other => StructAttr::Other(quote! { #attr }),
    };

    Ok(attr)
}

pub fn parse_struct_field_attr(attr: &Attribute) -> Result<StructFieldAttr> {
    let kind = parse_struct_field_attr_kind(&attr)?;

    let attr = match kind {
        StructFieldAttrKind::ByteCount => {
            assert!(attr.tokens.is_empty());

            StructFieldAttr::ByteCount(0)
        }
        StructFieldAttrKind::Other => StructFieldAttr::Other(quote! { #attr }),
    };

    Ok(attr)
}

fn parse_struct_attr_kind(attr: &Attribute) -> Result<StructAttrKind> {
    let mut tokens = TokenStream::new();

    let path = &attr.path;
    path.to_tokens(&mut tokens);

    syn::parse2::<StructAttrKind>(tokens)
}

fn parse_struct_field_attr_kind(attr: &Attribute) -> Result<StructFieldAttrKind> {
    let mut tokens = TokenStream::new();

    let path = &attr.path;
    path.to_tokens(&mut tokens);

    syn::parse2::<StructFieldAttrKind>(tokens)
}

impl Parse for StructAttrKind {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let ident_str = ident.to_string();
        let ident_str = ident_str.as_str();

        let kind = match ident_str {
            "storage" => StructAttrKind::Storage,
            _ => StructAttrKind::Other,
        };

        Ok(kind)
    }
}

impl Parse for StructFieldAttrKind {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let ident_str = ident.to_string();
        let ident_str = ident_str.as_str();

        let kind = match ident_str {
            "byte_count" => StructFieldAttrKind::ByteCount,
            _ => StructFieldAttrKind::Other,
        };

        Ok(kind)
    }
}

pub fn has_storage_attr(attrs: &[StructAttr]) -> bool {
    struct_has_attr(attrs, StructAttrKind::Storage)
}

pub fn has_byte_count_attr(attrs: &[StructFieldAttr]) -> bool {
    struct_field_has_attr(attrs, StructFieldAttrKind::ByteCount)
}

pub fn struct_has_attr(attrs: &[StructAttr], kind: StructAttrKind) -> bool {
    attrs.iter().any(|attr| attr.kind() == kind)
}

pub fn struct_field_has_attr(attrs: &[StructFieldAttr], kind: StructFieldAttrKind) -> bool {
    attrs.iter().any(|attr| attr.kind() == kind)
}
