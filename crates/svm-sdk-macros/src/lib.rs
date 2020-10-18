extern crate proc_macro;

mod storage;
use storage::parse_storage;

#[proc_macro_attribute]
pub fn storage(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    parse_storage(input)
}
