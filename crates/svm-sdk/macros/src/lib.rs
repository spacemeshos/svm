#![allow(unused)]

mod app;
mod function;
mod schema;
mod r#struct;
mod r#type;

#[cfg(feature = "api")]
mod api;

use app::App;
use function::{FuncAttr, FuncAttrKind, Function};
use r#struct::storage_vars;
use r#struct::{Struct, Var};
use r#type::{PrimType, Type};
use schema::{Export, Schema, Signature};

#[proc_macro_attribute]
pub fn app(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match app::expand(args.into(), input.into()) {
        Err(err) => err.to_compile_error().into(),
        Ok((schema, ast)) => ast.into(),
    }
}
