#![deny(unused)]

mod api;
mod function;
mod data;
mod sections;
mod r#struct;
mod template;
mod r#type;

use function::{FuncAttr, FuncAttrKind, Function};
use data::{Export, TemplateData};
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
        Ok((_schema, ast)) => ast.into(),
    }
}
