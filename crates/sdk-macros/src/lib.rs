#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![deny(rustdoc::broken_intra_doc_links)]

mod function;
mod json;
mod meta;
mod r#struct;
mod template;
mod r#type;

use function::{FuncAttr, FuncAttrKind, Function};
use meta::{Export, TemplateMeta};
use r#struct::storage_vars;
use r#struct::{Struct, Var};
use r#type::{PrimType, Type};
use template::Template;

#[proc_macro_attribute]
pub fn template(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match template::expand(args.into(), input.into()) {
        Ok((ast, meta)) => finalize_ast(ast, &meta),
        Err(err) => err.to_compile_error().into(),
    }
}

#[cfg(feature = "meta")]
fn finalize_ast(ast: proc_macro2::TokenStream, meta: &TemplateMeta) -> proc_macro::TokenStream {
    let path = format!("{}-meta.json", meta.name());
    dbg!(&path);
    dbg!(&path);
    dbg!(&path);
    let meta_json = json::meta(&meta);
    json::json_write(&path, &meta_json);

    ast.into()
}

#[cfg(not(feature = "meta"))]
fn finalize_ast(ast: proc_macro2::TokenStream, meta: &TemplateMeta) -> proc_macro::TokenStream {
    use quote::quote;

    let meta_json = json::meta(&meta);
    let meta_stream = json::to_tokens(&meta_json);

    let final_ast = quote! {
        #ast

        pub fn raw_meta() -> String {
            // We can't implement [`quote::ToTokens`] for [`serde_json::Value`] since both are defined in other crates.
            // Instead, we return a `String` and we'll use [`serde_json::from_str`] within the tests.
            #meta_stream.to_string()
        }
    };

    final_ast.into()
}
