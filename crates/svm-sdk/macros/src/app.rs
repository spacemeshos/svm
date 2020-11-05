extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    braced, parenthesized, token, Attribute, Error, Field, Item, ItemFn, ItemMod, ItemStruct,
    ItemType, ItemUse, Result, Token, Visibility,
};

use crate::Function;

struct Module {
    name: Ident,
    functions: Vec<Function>,
    structs: Vec<ItemStruct>,
    imports: Vec<ItemUse>,
    aliases: Vec<ItemType>,
}

pub fn transform(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let module = syn::parse2(input)?;
    let module = parse_module(module);

    let ast = quote! {
        //
    };

    Ok(ast)
}

fn parse_module(mut module: ItemMod) -> Result<Module> {
    let name = module.ident.clone();

    let mut functions = Vec::new();
    let mut structs = Vec::new();
    let mut imports = Vec::new();
    let mut aliases = Vec::new();

    let (_, content) = module.content.take().unwrap();

    for item in content {
        // TODO: Is is possible to extact the `item` real `Span`?
        let span = Span::call_site();

        match item {
            Item::Fn(item) => {
                let func = Function::new(item);
                functions.push(func);
            }
            Item::Struct(item) => structs.push(item),
            Item::Use(item) => imports.push(item),
            Item::Type(item) => aliases.push(item),
            Item::Const(item) => {
                let msg = "declaring `const` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Enum(item) => {
                let msg = "declaring `enum` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::ExternCrate(item) => {
                let msg = "using `extern crate` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::ForeignMod(item) => {
                let msg =
                    "using foreign items such as `extern \"C\"` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Impl(item) => {
                let msg = "using `impl` block inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Macro(item) => {
                let msg = "declaring `macro_rules!` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Macro2(item) => {
                let msg = "declaring `macro` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Mod(item) => {
                let msg = "declaring new modules inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Static(item) => {
                let msg = "declaring new `static` items inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Trait(item) => {
                let msg = "declaring new traits inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::TraitAlias(item) => {
                let msg = "declaring new trait aliases inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Union(item) => {
                let msg = "declaring `union` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Verbatim(item) => {
                let msg = format!("invalid Rust code: {}", item);
                return Err(Error::new(span, msg));
            }
            Item::__Nonexhaustive => unreachable!(),
        }
    }

    let module = Module {
        name,
        functions,
        structs,
        imports,
        aliases,
    };

    Ok(module)
}
