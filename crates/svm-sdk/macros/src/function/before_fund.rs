use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Block, Error, FnArg, ItemFn, Pat, PatType, Result, ReturnType, Signature, Type,
};

use super::has_before_fund_attr;
use crate::{attr, FuncAttrKind, FuncAttribute, Function};

pub fn expand(func: &Function, attrs: &[FuncAttribute]) -> Result<TokenStream> {
    debug_assert!(has_before_fund_attr(attrs));

    validate_before_fund_func_sig(func)?;

    let func = func.stream();

    let ast = quote! {
        #[inline]
        #func
    };

    Ok(ast)
}

fn validate_before_fund_func_sig(func: &Function) -> Result<()> {
    let sig = func.raw_sig();
    let span = Span::call_site();
    let msg = "`#[before_fund]` annotated function should have signature of `fn(value: svm_sdk::Amount) -> ()`";

    if sig.inputs.len() != 1 || matches!(sig.output, ReturnType::Default) == false {
        return Err(Error::new(span, msg));
    }

    let input = sig.inputs.first().unwrap();

    if let FnArg::Typed(PatType { attrs, ty, .. }) = input {
        if !attrs.is_empty() {
            return Err(Error::new(span, msg));
        }

        let mut tokens = TokenStream::new();
        ty.to_tokens(&mut tokens);

        let ty = tokens.to_string();
        let ty = ty.as_str();

        if ty == "svm_sdk :: Amount" || ty == "Amount" {
            return Ok(());
        }
    }

    Err(Error::new(span, msg))
}
