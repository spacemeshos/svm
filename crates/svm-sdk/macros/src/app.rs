use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use syn::{
     Error, Item, ItemMod, ItemStruct,
    ItemType, ItemUse, Result, 
};

use crate::{Struct, Function};
use super::{r#struct, function};

pub struct App {
    pub name: Ident,
    pub functions: Vec<Function>,
    pub structs: Vec<Struct>,
    pub imports: Vec<ItemUse>,
    pub aliases: Vec<ItemType>,
}

pub fn expand(_args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let module = syn::parse2(input)?;
    let app = parse_app(module)?;

    let imports = &app.imports;
    let aliases = &app.aliases;

    let structs = expand_structs(&app)?;
    let functions = expand_functions(&app)?;

    let ast = quote! {
        // #(#imports)*

        // #(#aliases)*

        #structs

        #functions
    };

    Ok(ast)
}

pub fn parse_app(mut raw_app: ItemMod) -> Result<App> {
    let name = raw_app.ident.clone();

    let mut functions = Vec::new();
    let mut structs = Vec::new();
    let mut imports = Vec::new();
    let mut aliases = Vec::new();

    let (_, content) = raw_app.content.take().unwrap();

    for item in content {
        // TODO: Is is possible to extract the `item` real `Span`?
        let span = Span::call_site();

        match item {
            Item::Fn(item) => {
                let func = Function::new(item);
                functions.push(func);
            }
            Item::Struct(item) => {
                let strukt = Struct::new(item);
                structs.push(strukt);
            }
            Item::Use(item) => imports.push(item),
            Item::Type(item) => aliases.push(item),
            Item::Const(..) => {
                let msg = "declaring `const` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Enum(..) => {
                let msg = "declaring `enum` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::ExternCrate(..) => {
                let msg = "using `extern crate` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::ForeignMod(..) => {
                let msg =
                    "using foreign items such as `extern \"C\"` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Impl(..) => {
                let msg = "using `impl` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Macro(..) => {
                let msg = "declaring `macro_rules!` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Macro2(..) => {
                let msg = "declaring `macro` inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Mod(..) => {
                let msg = "declaring new modules inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Static(..) => {
                let msg = "declaring new `static` items inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Trait(..) => {
                let msg = "declaring new traits inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::TraitAlias(..) => {
                let msg = "using trait aliases inside `#[app]` is not supported.";
                return Err(Error::new(span, msg));
            }
            Item::Union(..) => {
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

    let app = App {
        name,
        functions,
        structs,
        imports,
        aliases,
    };

    Ok(app)
}

pub fn expand_structs(app: &App) -> Result<TokenStream> {
    let mut structs = Vec::new();

    for strukt in &app.structs {
        let strukt = r#struct::expand(strukt)?;

        structs.push(strukt);
    }

    let ast = quote! {
        #(#structs)*
    };

    Ok(ast)
}

pub fn expand_functions(app: &App) -> Result<TokenStream> {
    let mut funcs = Vec::new();

    for func in &app.functions {
        let func = function::expand(func)?;
        
        funcs.push(func);
    } 

    let ast = quote! {
        #(#funcs)*
    };

    Ok(ast)
}

#[cfg(test)]
mod test {
    use super::*;

    use syn::parse_quote;

    macro_rules! assert_err {
        ($expected:expr, $($tt:tt)*) => {{
            let raw_app: ItemMod = parse_quote!( $($tt)* );

            let res = parse_app(raw_app);

            // we can't use `unwrap_err()` since `App`
            // doesn't implement `std::fmt::Debug`
            let actual = res.err().unwrap();

            assert_eq!($expected, actual.to_string());
        }};
    }

    #[test]
    fn app_empty() {
        let raw_app: ItemMod = parse_quote! {
            #[app]
            mod my_app {}
        };

        let res = parse_app(raw_app);
        assert!(res.is_ok());
    }

    #[test]
    fn app_declaring_const_not_allowed() {
        let err = "declaring `const` inside `#[app]` is not supported.";

        assert_err!(
            err,
            #[app]
            mod my_app {
                const N: u32 = 10;
            }
        );
    }

    #[test]
    fn app_declaring_static_not_allowed() {
        let err = "declaring new `static` items inside `#[app]` is not supported.";

        assert_err!(
            err,
            #[app]
            mod my_app {
                static N: u32 = 10;
            }
        );
    }

    #[test]
    fn app_declaring_enum_not_allowed() {
        let err = "declaring `enum` inside `#[app]` is not supported.";

        assert_err!(
            err,
            #[app]
            mod my_app {
                enum MyEum {}
            }
        );
    }

    #[test]
    fn app_using_extern_crate_not_allowed() {
        let err = "using `extern crate` inside `#[app]` is not supported.";

        assert_err!(
            err,
            #[app]
            mod my_app {
                extern crate alloc;
            }
        );
    }

    #[test]
    fn app_using_ffi_not_allowed() {
        let err = "using foreign items such as `extern \"C\"` inside `#[app]` is not supported.";

        assert_err!(
            err,
            #[app]
            mod my_app {
                extern "C" {}
            }
        );
    }

    #[test]
    fn app_using_impl_not_allowed() {
        let err = "using `impl` inside `#[app]` is not supported.";

        assert_err!(
            err,
            #[app]
            mod my_app {
                struct S;

                impl S {}
            }
        );
    }

    #[test]
    fn app_using_macro_rules_not_allowed() {
        let err = "declaring `macro_rules!` inside `#[app]` is not supported.";

        assert_err!(
            err,
            #[app]
            mod my_app {
                macro_rules! print {}
            }
        );
    }

    #[test]
    fn app_declaring_traits_not_allowed() {
         let err = "declaring new traits inside `#[app]` is not supported.";

        assert_err!(err,
            #[app]
            mod my_app {
                trait Print {}
            }
        );
    }

    #[test]
    fn app_declaring_union_not_allowed() {
        let err = "declaring `union` inside `#[app]` is not supported.";

        assert_err!(
            err,
            #[app]
            mod my_app {
                union U {}
            }
        );
    }
}