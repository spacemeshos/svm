#![allow(unused)]

extern crate proc_macro;

mod endpoint;
mod storage;

use endpoint::parse_endpoint;
use storage::parse_storage;

#[proc_macro_attribute]
pub fn storage(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    parse_storage(input)
}

#[proc_macro_attribute]
pub fn endpoint(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    parse_endpoint(input)
}
