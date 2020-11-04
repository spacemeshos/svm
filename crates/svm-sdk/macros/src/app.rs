extern crate proc_macro;

use proc_macro2::{Ident, TokenStream};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parenthesized, token, Attribute, Field, Result, Token, Visibility};

struct Function {
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

        Ok(Function {
            attrs: input.call(Attribute::parse_outer)?,
            fn_token: input.parse()?,
            name: input.parse()?,
            paren_token: parenthesized!(params in input),
            params: params.parse_terminated(Field::parse_named)?,
            body: input.parse()?,
        })
    }
}

fn parse_funcs(input: ParseStream) -> Result<Vec<Function>> {
    let mut funcs = Vec::new();

    let mut lookahead = input.lookahead1();

    while lookahead.peek(Token![fn]) {
        let func: Function = input.parse()?;

        funcs.push(func);

        lookahead = input.lookahead1();
    }

    Ok(funcs)
}

struct Struct {
    attrs: Vec<Attribute>,
    struct_token: Token![struct],
    name: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> Result<Self> {
        let fields;

        Ok(Struct {
            attrs: input.call(Attribute::parse_outer)?,
            struct_token: input.parse()?,
            name: input.parse()?,
            brace_token: braced!(fields in input),
            fields: fields.parse_terminated(Field::parse_named)?,
        })
    }
}

enum Item {
    Function(Vec<Attribute>, Function),

    Struct(Vec<Attribute>, Struct),
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!()
    }
}

fn parse_items(input: ParseStream) -> Result<Vec<Item>> {
    let mut items = Vec::new();

    let mut remaining_items = true;

    while remaining_items {
        let attrs = input.call(Attribute::parse_outer)?;

        let lookahead = input.lookahead1();

        if lookahead.peek(Token![fn]) {
            let func: Function = input.parse()?;
            items.push(Item::Function(attrs, func));
        } else if lookahead.peek(Token![struct]) {
            todo!()
        } else {
            todo!()
        }
    }

    Ok(items)
}

struct App {
    mod_token: Token![mod],
    name: Ident,
    items: Vec<Item>,
}

impl Parse for App {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(App {
            mod_token: input.parse()?,
            name: input.parse()?,
            items: parse_items(input)?,
        })
    }
}

pub fn parse_app(args: TokenStream, input: TokenStream) -> TokenStream {
    let app = syn::parse2::<App>(input);

    todo!();
}
