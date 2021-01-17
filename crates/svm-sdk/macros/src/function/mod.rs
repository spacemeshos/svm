use proc_macro2::{Ident, Span, TokenStream};

use quote::{quote, ToTokens};
use syn::{Attribute, Block, Error, ItemFn, Result, Signature};

mod attr;
mod ctor;
mod endpoint;
mod fundable;
mod fundable_hook;

pub use attr::{
    find_attr, func_attrs, has_ctor_attr, has_default_fundable_hook_attr, has_endpoint_attr,
    has_fundable_attr, has_fundable_hook_attr,
};
pub use attr::{FuncAttr, FuncAttrKind};

use crate::schema::Schema;

pub struct Function {
    raw_func: ItemFn,

    index: usize,
}

impl Function {
    pub fn new(raw_func: ItemFn, index: usize) -> Self {
        Self { raw_func, index }
    }

    pub fn raw_name(&self) -> Ident {
        self.raw_sig().ident.clone()
    }

    pub fn raw_body(&self) -> &Block {
        &self.raw_func.block
    }

    pub fn raw_sig(&self) -> &Signature {
        &self.raw_func.sig
    }

    pub fn raw_attrs(&self) -> Vec<Attribute> {
        self.raw_func.attrs.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn export_name(&self) -> String {
        if cfg!(target_arch = "wasm32") {
            format!("_{}", self.index)
        } else {
            format!("{}", self.raw_name())
        }
    }
}

pub fn expand(func: &Function) -> Result<TokenStream> {
    let attrs = func_attrs(func)?;

    validate_attrs(&attrs)?;

    let ast = if has_ctor_attr(&attrs) {
        ctor::expand(func, &attrs)?
    } else if has_endpoint_attr(&attrs) {
        endpoint::expand(func, &attrs)?
    } else if has_fundable_hook_attr(&attrs) {
        fundable_hook::expand(func, &attrs)?
    } else {
        expand_func(func, &attrs)?
    };

    let ast = expand_other_attrs(ast, &attrs)?;

    Ok(ast)
}

fn validate_attrs(attrs: &[FuncAttr]) -> Result<()> {
    validate_attrs_no_dups(attrs)?;
    validate_attrs_usage(attrs)?;
    validate_attrs_order(attrs)?;

    Ok(())
}

pub fn host_includes() -> TokenStream {
    quote! {
        use svm_sdk::traits::Host;

        #[cfg(feature = "mock")]
        use svm_sdk::host::MockHost as Node;

        #[cfg(feature = "ffi")]
        use svm_sdk::host::ExtHost as Node;
    }
}

pub fn expand_other_attrs(ast: TokenStream, _attrs: &[FuncAttr]) -> Result<TokenStream> {
    Ok(ast)
}

pub fn expand_func(func: &Function, _attrs: &[FuncAttr]) -> Result<TokenStream> {
    let ast = func.raw_func.to_token_stream();

    Ok(ast)
}

fn validate_attrs_no_dups(attrs: &[FuncAttr]) -> Result<()> {
    let span = Span::call_site();

    let mut seen_ctor = false;
    let mut seen_endpoint = false;
    let mut seen_fundable = false;
    let mut seen_fundable_hook = false;

    for attr in attrs {
        match attr.kind() {
            FuncAttrKind::Ctor => {
                if seen_ctor {
                    return Err(Error::new(
                        span,
                        "Each function can be annotated with `#[ctor]` exactly once.",
                    ));
                }
                seen_ctor = true;
            }

            FuncAttrKind::Endpoint => {
                if seen_endpoint {
                    return Err(Error::new(
                        span,
                        "Each function can be annotated with `#[endpoint]` exactly once.",
                    ));
                }
                seen_endpoint = true;
            }
            FuncAttrKind::FundableHook => {
                if seen_fundable_hook {
                    return Err(Error::new(
                        span,
                        "Each function can be annotated with `#[fundable_hook]` exactly once.",
                    ));
                }
                seen_fundable_hook = true;
            }
            FuncAttrKind::Fundable => {
                if seen_fundable {
                    return Err(Error::new(
                        span,
                        "Each function can be annotated with `#[fundable(..)]` exactly once.",
                    ));
                }
                seen_fundable = true;
            }
            FuncAttrKind::Other => continue,
        }
    }

    Ok(())
}

fn validate_attrs_usage(attrs: &[FuncAttr]) -> Result<()> {
    let span = Span::call_site();
    let mut seen_ctor = false;
    let mut seen_endpoint = false;
    let mut seen_fundable = false;
    let mut seen_fundable_hook = false;

    for attr in attrs {
        match attr.kind() {
            FuncAttrKind::Ctor => seen_ctor = true,
            FuncAttrKind::Endpoint => seen_endpoint = true,
            FuncAttrKind::FundableHook => seen_fundable_hook = true,
            FuncAttrKind::Fundable => seen_fundable = true,
            FuncAttrKind::Other => continue,
        }
    }

    if seen_ctor && seen_endpoint {
        return Err(Error::new(
            span,
            "#[ctor]` and `#[endpoint]` can't co-exist.",
        ));
    }

    if seen_endpoint && seen_fundable_hook {
        return Err(Error::new(
            span,
            "#[endpoint]` and `#[fundable_hook]` can't co-exist.",
        ));
    }

    if seen_fundable && seen_fundable_hook {
        return Err(Error::new(
            span,
            "#[fundable_hook]` and `#[fundable(..)]` can't co-exist.",
        ));
    }

    if seen_fundable && !seen_endpoint && !seen_ctor {
        return Err(Error::new(
            span,
            "#[fundable(..)] can't be used without `#[endpoint]` or `#[ctor]`",
        ));
    }

    Ok(())
}

fn validate_attrs_order(attrs: &[FuncAttr]) -> Result<()> {
    let span = Span::call_site();

    let mut seen_ctor = false;
    let mut seen_endpoint = false;

    for attr in attrs {
        match attr.kind() {
            FuncAttrKind::Ctor => seen_ctor = true,
            FuncAttrKind::Endpoint => seen_endpoint = true,
            FuncAttrKind::FundableHook => continue,
            FuncAttrKind::Fundable => {
                if seen_ctor {
                    return Err(Error::new(
                        span,
                        "`#[fundable(..)]` should be placed above `#[ctor]`",
                    ));
                }

                if seen_endpoint {
                    return Err(Error::new(
                        span,
                        "`#[fundable(..)]` should be placed above `#[endpoint]`",
                    ));
                }
            }
            FuncAttrKind::Other => continue,
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use syn::parse_quote;

    macro_rules! assert_err {
        ($expected:expr, $($tt:tt)*) => {{
            let raw_func: ItemFn = parse_quote!( $($tt)* );
            let mut func = Function::new(raw_func, 0);

            let actual = expand(&mut func).unwrap_err();
            assert_eq!($expected, actual.to_string());
        }};
    }

    macro_rules! assert_ok {
        ($($tt:tt)*) => {{
            let raw_func: ItemFn = parse_quote!( $($tt)* );

            let mut func = Function::new(raw_func, 0);

            let res = expand(&mut func);

            if res.is_err() {
                let err = res.unwrap_err();
                panic!(err);
            }
            else {
                assert!(res.is_ok());
            }
        }};
    }

    #[test]
    fn fundable_can_not_live_alone() {
        let err = "#[fundable(..)] can't be used without `#[endpoint]` or `#[ctor]`";

        assert_err!(
            err,
            #[fundable(deny)]
            fn deny() {}
        )
    }

    #[test]
    fn ctor_and_fundable_attrs_wrong_order() {
        let err = "`#[fundable(..)]` should be placed above `#[ctor]`";

        assert_err!(
            err,
            #[ctor]
            #[fundable(deny)]
            fn get() {}
        );
    }

    #[test]
    fn endpoint_and_fundable_attrs_wrong_order() {
        let err = "`#[fundable(..)]` should be placed above `#[endpoint]`";

        assert_err!(
            err,
            #[endpoint]
            #[fundable(deny)]
            fn get() {}
        );
    }

    #[test]
    fn endpoint_and_ctor_fails() {
        let err = "#[ctor]` and `#[endpoint]` can't co-exist.";

        assert_err!(
            err,
            #[ctor]
            #[endpoint]
            fn get() {}
        );
    }

    #[test]
    fn endpoint_and_fundable_hook_fails() {
        let err = "#[endpoint]` and `#[fundable_hook]` can't co-exist.";

        assert_err!(
            err,
            #[fundable_hook]
            #[endpoint]
            fn get() {}
        );
    }

    #[test]
    fn fundable_hook_and_fundable_not_allowed() {
        let err = "#[fundable_hook]` and `#[fundable(..)]` can't co-exist.";

        assert_err!(
            err,
            #[fundable_hook]
            #[fundable(default)]
            fn get() {}
        );
    }

    #[test]
    fn ctor_used_twice_fails() {
        let err = "Each function can be annotated with `#[ctor]` exactly once.";

        assert_err!(
            err,
            #[ctor]
            #[ctor]
            fn get() {}
        );
    }

    #[test]
    fn endpoint_used_twice_fails() {
        let err = "Each function can be annotated with `#[endpoint]` exactly once.";

        assert_err!(
            err,
            #[endpoint]
            #[endpoint]
            fn get() {}
        );
    }

    #[test]
    fn fundable_hook_used_twice_fails() {
        let err = "Each function can be annotated with `#[fundable_hook]` exactly once.";

        assert_err!(
            err,
            #[fundable_hook]
            #[fundable_hook]
            fn get(value: svm_sdk::Amount) {}
        );
    }

    #[test]
    fn fundable_used_twice_fails() {
        let err = "Each function can be annotated with `#[fundable(..)]` exactly once.";

        assert_err!(
            err,
            #[fundable(allow)]
            #[fundable(allow)]
            #[endpoint]
            fn get(value: svm_sdk::Amount) {}
        );
    }

    #[test]
    fn fundable_hook_func_with_args_fails() {
        let err = "`#[fundable_hook]` annotated function should have signature of `fn() -> ()`";

        assert_err!(
            err,
            #[fundable_hook]
            fn deny(v: svm_sdk::Amount) {}
        );
    }

    #[test]
    fn fundable_hook_func_with_return_type_fails() {
        let err = "`#[fundable_hook]` annotated function should have signature of `fn() -> ()`";

        assert_err!(
            err,
            #[fundable_hook]
            fn deny() -> u32 {
                0
            }
        );
    }

    #[test]
    fn endpoint_func_valid_sig() {
        assert_ok!(
            #[endpoint]
            fn get(v: svm_sdk::Amount) {}
        );

        assert_ok!(
            #[endpoint]
            fn get(v: svm_sdk::Amount) -> (u32, svm_sdk::Address) {
                panic!()
            }
        );
    }

    #[test]
    fn fundable_func_valid_sig() {
        assert_ok!(
            #[fundable(allow)]
            #[endpoint]
            fn get(addr: svm_sdk::Address) {}
        );
    }

    #[test]
    fn fundable_hook_func_valid_sig() {
        assert_ok!(
            #[fundable_hook]
            fn allow() {}
        );
    }
}
