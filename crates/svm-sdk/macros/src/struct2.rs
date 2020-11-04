extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parenthesized, token, Attribute, Error, Field, Result, Token, Visibility};

pub struct Struct {
    attrs: Vec<Attribute>,
    struct_token: Token![struct],
    name: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> Result<Self> {
        let fields;

        let r#struct = Struct {
            attrs: input.call(Attribute::parse_outer)?,
            struct_token: input.parse()?,
            name: input.parse()?,
            brace_token: braced!(fields in input),
            fields: fields.parse_terminated(Field::parse_named)?,
        };

        Ok(r#struct)
    }
}

pub fn expand_struct(s: Struct) {
    //
}
