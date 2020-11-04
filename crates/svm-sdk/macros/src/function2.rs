extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parenthesized, token, Attribute, Error, Field, Result, Token, Visibility};

pub struct Function {
    attrs: Vec<Attribute>,
    fn_token: Token![fn],
    name: Ident,
    paren_token: token::Paren,
    params: Punctuated<Field, Token![,]>,
    body: TokenStream,
}

impl Parse for Function {
    fn parse(input: ParseStream) -> Result<Self> {
        let params;

        let func = Function {
            attrs: input.call(Attribute::parse_outer)?,
            fn_token: input.parse()?,
            name: input.parse()?,
            paren_token: parenthesized!(params in input),
            params: params.parse_terminated(Field::parse_named)?,
            body: input.parse()?,
        };

        Ok(func)
    }
}

pub fn expand_function(func: Function) {
    //
}
