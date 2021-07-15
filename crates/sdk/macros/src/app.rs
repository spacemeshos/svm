use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use serde_json::Value;

use syn::{Error, Item, ItemMod, ItemStruct, ItemType, ItemUse, Result};

use super::{function, r#struct};
use crate::{schema, Function, Schema, Struct};

#[cfg(feature = "api")]
use crate::api;

use r#function::{func_attrs, has_default_fundable_hook_attr};
use r#struct::has_storage_attr;

pub struct App {
    name: Ident,
    functions: Vec<Function>,
    structs: Vec<Struct>,
    imports: Vec<ItemUse>,
    aliases: Vec<ItemType>,
    default_fundable_hook: Option<Ident>,
}

impl App {
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn structs(&self) -> &[Struct] {
        &self.structs
    }

    pub fn imports(&self) -> &[ItemUse] {
        &self.imports
    }

    pub fn aliases(&self) -> &[ItemType] {
        &self.aliases
    }

    pub fn default_fundable_hook(&self) -> Option<Ident> {
        self.default_fundable_hook.clone()
    }

    pub fn set_default_fundable_hook(&mut self, hook: Ident) {
        self.default_fundable_hook = Some(hook)
    }
}

pub fn expand(_args: TokenStream, input: TokenStream) -> Result<(Schema, TokenStream)> {
    let module = syn::parse2(input)?;
    let app = parse_app(module)?;
    let schema = schema::app_schema(&app)?;

    let imports = app.imports();
    let aliases = app.aliases();

    let structs = expand_structs(&app)?;
    let functions = expand_functions(&app)?;
    let alloc_func = alloc_func_ast();

    #[cfg(feature = "api")]
    let api = api::json_api(&schema);

    #[cfg(feature = "api")]
    let stream = api::json_tokenstream(&api);

    #[cfg(not(feature = "api"))]
    let stream = quote! { "" };

    #[cfg(feature = "api")]
    let data = api::json_data_layout(&schema);

    #[cfg(feature = "api")]
    write_schema(&app, &api, &data);

    let ast = quote! {
        // #(#imports)*

        // #(#aliases)*

        #alloc_func

        #structs

        #functions

        #[cfg(all(feature = "api", not(target_arch = "wasm32")))]
        pub fn raw_schema() -> String {
            #stream.to_string()
        }
    };

    Ok((schema, ast))
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
        // See <https://github.com/spacemeshos/svm/issues/278>.
        let span = Span::call_site();

        match item {
            Item::Fn(item) => {
                let func = Function::new(item, functions.len());
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
            Item::__TestExhaustive(..) => unreachable!(),
        }
    }

    let mut app = App {
        name,
        functions,
        structs,
        imports,
        aliases,
        default_fundable_hook: None,
    };

    let default = extract_default_fundable_hook(&app)?;

    if default.is_some() {
        app.set_default_fundable_hook(default.unwrap());
    }

    Ok(app)
}

fn extract_default_fundable_hook(app: &App) -> Result<Option<Ident>> {
    let span = Span::call_site();
    let mut seen_default_fundable_hook = false;
    let mut default = None;

    for func in app.functions().iter() {
        let attrs = func_attrs(func).unwrap();

        if has_default_fundable_hook_attr(&attrs) {
            if seen_default_fundable_hook {
                return Err(Error::new(
                    span,
                    "There can be only a single default `fundable hook`",
                ));
            }

            seen_default_fundable_hook = true;

            default = Some(func.raw_name());
        }
    }

    Ok(default)
}

#[cfg(all(feature = "api", target_arch = "wasm32"))]
fn write_schema(app: &App, api: &Value, data: &Value) {
    api::json_write(&format!("{}-api.json", app.name()), api);
    api::json_write(&format!("{}-data.json", app.name()), data);
}

#[cfg(any(not(feature = "api"), not(target_arch = "wasm32")))]
fn write_schema(app: &App, api: &Value, data: &Value) {
    //
}

fn expand_structs(app: &App) -> Result<TokenStream> {
    let mut structs = Vec::new();

    validate_structs(app)?;

    for strukt in app.structs() {
        let strukt = r#struct::expand(strukt)?;

        structs.push(strukt);
    }

    let ast = quote! {
        #(#structs)*
    };

    Ok(ast)
}

fn validate_structs(app: &App) -> Result<()> {
    let mut seen_storage = false;

    for strukt in app.structs() {
        match strukt.attrs() {
            Ok(attrs) => {
                if has_storage_attr(attrs) {
                    if seen_storage {
                        let msg = format!("an App can have only a single `#[storage]`");
                        let span = Span::call_site();

                        return Err(Error::new(span, msg));
                    }

                    seen_storage = true;
                }
            }
            Err(err) => return Err(err.clone()),
        }
    }

    Ok(())
}

fn expand_functions(app: &App) -> Result<TokenStream> {
    validate_funcs(app)?;

    let mut funcs = Vec::new();

    for func in app.functions() {
        let func = function::expand(func, app)?;

        funcs.push(func);
    }

    let implicit_fundable_hook = if app.default_fundable_hook().is_some() {
        quote! {}
    } else {
        function::fundable_hook::expand_default()?
    };

    let ast = quote! {
        #(#funcs)*

        #implicit_fundable_hook
    };

    Ok(ast)
}

fn validate_funcs(app: &App) -> Result<()> {
    Ok(())
}

fn alloc_func_ast() -> TokenStream {
    quote! {
        extern crate svm_sdk;

        #[no_mangle]
        pub extern "C" fn svm_alloc(size: u32) -> u32 {
            let ptr = svm_sdk::alloc(size as usize);

            ptr.offset() as u32
        }
    }
}
