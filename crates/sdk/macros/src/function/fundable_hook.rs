use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Error, FnArg, PatType, Result, ReturnType};

use super::attr;
use attr::{has_fundable_hook_attr, FuncAttr};

use crate::{function, Function};

pub fn expand(func: &Function, attrs: &[FuncAttr]) -> Result<TokenStream> {
    debug_assert!(has_fundable_hook_attr(attrs));

    validate_fundable_hook_func_sig(func)?;

    let sig = func.raw_sig();
    let body = func.raw_body();

    let includes = function::host_includes();

    let ast = quote! {
        #[inline]
        #sig {
            #includes

            #body
        }
    };

    Ok(ast)
}

pub fn expand_default() -> Result<TokenStream> {
    let ast = quote! {
        #[no_mangle]
        pub extern "C" fn svm_fund() { }
    };

    Ok(ast)
}

fn validate_fundable_hook_func_sig(func: &Function) -> Result<()> {
    let sig = func.raw_sig();
    let span = Span::call_site();

    if sig.inputs.len() != 0 || matches!(sig.output, ReturnType::Default) == false {
        let msg = "`#[fundable_hook]` annotated function should have signature of `fn() -> ()`";
        return Err(Error::new(span, msg));
    }

    Ok(())
}
