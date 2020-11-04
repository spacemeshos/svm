extern crate proc_macro;

use proc_macro2::{Ident, TokenStream};

use quote::quote;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parenthesized, token, Attribute, Error, Field, Result, Token, Visibility};

use crate::{Function, Struct};

enum Item {
    Function(Vec<Attribute>, Function),

    Struct(Vec<Attribute>, Struct),
}

struct App {
    mod_token: Token![mod],
    name: Ident,
    items: Vec<Item>,
}

impl Parse for App {
    fn parse(input: ParseStream) -> Result<Self> {
        let app = App {
            mod_token: input.parse()?,
            name: input.parse()?,
            items: parse_items(input)?,
        };

        Ok(app)
    }
}

pub fn parse_app(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let app: App = syn::parse2::<App>(input)?;

    let ast = expand_app(app);
    Ok(ast)
}

fn expand_app(app: App) -> TokenStream {
    quote! {
        //
    }
}

fn parse_items(input: ParseStream) -> Result<Vec<Item>> {
    let content;
    let brace_token = braced!(content in input);

    let mut items = Vec::new();
    let mut remaining_items = true;

    while remaining_items {
        let attrs = content.call(Attribute::parse_outer)?;

        let lookahead = input.lookahead1();

        if lookahead.peek(Token![fn]) {
            let func = input.parse()?;
            items.push(Item::Function(attrs, func));
        } else if lookahead.peek(Token![struct]) {
            let r#struct = input.parse()?;
            items.push(Item::Struct(attrs, r#struct));
        } else {
            return Err(lookahead.error());
        }
    }

    Ok(items)
}
