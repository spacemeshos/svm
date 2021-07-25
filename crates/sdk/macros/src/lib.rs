#![allow(unused)]

mod api;
mod function;
mod program;
mod autogen;
mod r#struct;
mod template;
mod r#type;

use function::{FuncAttr, FuncAttrKind, Function};
use r#struct::storage_vars;
use r#struct::{Struct, Var};
use r#type::{PrimType, Type};
use program::{Export, Program, Signature};
use template::{parse_template, Template};

#[proc_macro_attribute]
pub fn template(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match template::expand(args.into(), input.into()) {
        Err(err) => err.to_compile_error().into(),
        Ok((schema, ast)) => ast.into(),
    }
}
