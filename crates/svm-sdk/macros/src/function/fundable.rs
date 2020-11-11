use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Block, Error, FnArg, ItemFn, Pat, PatType, Result, ReturnType, Signature, Type,
};

use crate::function::find_attr;
use crate::{attr, FuncAttrKind, FuncAttribute};

pub fn expand(ast: TokenStream, attrs: &[FuncAttribute]) -> Result<TokenStream> {
    debug_assert!(crate::function::has_fundable_attr(attrs));

    let attr = find_attr(attrs, FuncAttrKind::Fundable);

    let fund_hook = match attr {
        FuncAttribute::Fundable(s) => s,
        _ => unreachable!(),
    };

    let includes = crate::function::host_includes();

    let ast = quote! {
        {
            #includes;

            let value: svm_sdk::Amount = Node.get_value();

            #fund_hook(value);
        }
    };

    Ok(ast)
}
