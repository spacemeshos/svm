#![allow(unused)]

mod app;
mod function;
mod r#struct;

use function::Function;
use r#struct::Struct;

#[proc_macro_attribute]
pub fn app(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match app::expand(args.into(), input.into()) {
        Err(err) => {
            dbg!(err);

            panic!("...")
        }
        Ok(output) => output.into(),
    }
}
