use proc_macro2::TokenStream;

use quote::quote;
use syn::Result;

use super::attr;
use attr::{find_attr, has_fundable_attr, FuncAttr, FuncAttrKind};

pub fn expand(_ast: TokenStream, attrs: &[FuncAttr]) -> Result<TokenStream> {
    debug_assert!(has_fundable_attr(attrs));

    let attr = find_attr(attrs, FuncAttrKind::Fundable);

    let fund_hook = match attr {
        FuncAttr::Fundable(s) => s,
        _ => unreachable!(),
    };

    let includes = crate::function::host_includes();

    let ast = quote! {
        {
            #includes;

            let value: svm_sdk::Amount = Node::value();

            #fund_hook(value);
        }
    };

    Ok(ast)
}
