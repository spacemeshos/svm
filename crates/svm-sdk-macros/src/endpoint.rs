extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use syn::parse::ParseStream;
use syn::{
    Data, DataStruct, DeriveInput, Expr, ExprLit, Field, Fields, FieldsNamed, Lit, Path,
    PathArguments, Type, TypeArray, TypePath,
};

pub fn parse_endpoint(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let includes = includes_ast();
    let input: TokenStream = input.into();

    (quote! {
        #includes

        #input
    })
    .into()
}

fn parse_params(input: &TokenStream) -> TokenStream {
    todo!()
}

fn includes_ast() -> TokenStream {
    quote! {
        #[cfg(test)]
        use svm_sdk::host::MockHost;

        #[cfg(not(test))]
        use svm_sdk::host::ExtHost;

        use svm_sdk::{Amount, Address, LayerId, ensure, log};
    }
}
