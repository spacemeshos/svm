use proc_macro2::{Span, TokenStream};

use quote::{quote, ToTokens};
use syn::{Error, FnArg, PatType, Result, ReturnType};

use super::attr;
use attr::{has_fundable_hook_attr, FuncAttr};

use crate::Function;

pub fn expand(func: &Function, attrs: &[FuncAttr]) -> Result<TokenStream> {
    debug_assert!(has_fundable_hook_attr(attrs));

    validate_fundable_hook_func_sig(func)?;

    let sig = func.raw_sig();
    let body = func.raw_body();

    let ast = quote! {
        #[inline]
        #sig {
            #body
        }
    };

    Ok(ast)
}

fn validate_fundable_hook_func_sig(func: &Function) -> Result<()> {
    let sig = func.raw_sig();
    let span = Span::call_site();
    let msg = "`#[fundable_hook]` annotated function should have signature of `fn(value: svm_sdk::Amount) -> ()`";

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
