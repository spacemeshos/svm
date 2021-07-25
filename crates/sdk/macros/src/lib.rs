#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

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
        Err(err) => err.to_compile_error().into(),
        Ok((meta, ast)) => {
            let path = format!("{}-meta.json", meta.name());

            if cfg!(target_arch = "wasm32") {
                emit_metadata(&meta, path);

                ast.into()
            } else {
                finalize_ast(&ast, &meta)
            }
        }
    }
}

fn finalize_ast(ast: &proc_macro2::TokenStream, meta: &TemplateMeta) -> proc_macro::TokenStream {
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

fn emit_metadata<P: AsRef<std::path::Path>>(meta: &TemplateMeta, path: P) {
    let meta_json = json::meta(&meta);
    json::json_write(&path, &meta_json);
}
