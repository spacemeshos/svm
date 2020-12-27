#![allow(unused)]

mod app;
mod function;
mod schema;
mod r#struct;

use app::App;
use function::Function;
use r#struct::storage_vars;
use r#struct::{Struct, Var};

use schema::{AppSchema, Export};

#[proc_macro_attribute]
pub fn app(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match app::expand(args.into(), input.into()) {
        Err(err) => err.to_compile_error().into(),
        Ok(output) => output.into(),
    }
}
