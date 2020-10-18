extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Expr, ExprLit, Field, Fields, FieldsNamed,
    Lit, Path, PathArguments, Type, TypeArray, TypePath,
};

pub fn parse_endpoint(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    dbg!(args);
    dbg!("---------------");
    dbg!(input);

    (quote! {
        //
    })
    .into()
}
