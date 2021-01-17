use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Result;

use super::attr::{find_attr, has_fundable_attr, FuncAttr, FuncAttrKind};

use crate::function;

pub fn expand(attrs: &[FuncAttr]) -> Result<TokenStream> {
    debug_assert!(has_fundable_attr(attrs));

    let attr = find_attr(attrs, FuncAttrKind::Fundable).unwrap();

    let fund_hook = match attr {
        FuncAttr::Fundable(None) => Ident::new("svm_fund", Span::call_site()),
        FuncAttr::Fundable(Some(hook)) => Ident::new(hook, Span::call_site()),
        _ => unreachable!(),
    };

    let includes = function::host_includes();

    let ast = quote! {
        {
            #includes

            let value: svm_sdk::Amount = Node::value();

            if value > svm_sdk::Amount(0) {
                #fund_hook();
            }
        }
    };

    Ok(ast)
}
