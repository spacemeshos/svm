use proc_macro2::{Ident, Span, TokenStream};

use quote::{quote, ToTokens};
use syn::{Attribute, Block, Error, ItemFn, Result, Signature};

use crate::{attr, FuncAttrKind, FuncAttribute};

mod before_fund;
mod endpoint;
mod fundable;

pub struct Function {
    raw_func: ItemFn,
}

impl Function {
    pub fn new(raw_func: ItemFn) -> Self {
        Self { raw_func }
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

    pub fn stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();

        self.raw_func.to_tokens(&mut tokens);

        tokens
    }
}

fn expand(func: &Function) -> Result<TokenStream> {
    let attrs = func_attrs(func)?;

    validate_attrs(&attrs)?;

    let ast = if has_endpoint_attr(&attrs) {
        endpoint::expand(func, &attrs)?
    } else if has_before_fund_attr(&attrs) {
        before_fund::expand(func, &attrs)?
    } else {
        expand_func(func, &attrs)?
    };

    let ast = expand_other_attrs(ast, &attrs)?;

    Ok(ast)
}

fn func_attrs(func: &Function) -> Result<Vec<FuncAttribute>> {
    let mut attrs = Vec::new();

    for attr in func.raw_attrs() {
        let attr = attr::parse_attr(attr)?;

        attrs.push(attr);
    }

    Ok(attrs)
}

fn validate_attrs(attrs: &[FuncAttribute]) -> Result<()> {
    validate_attrs_no_dups(attrs)?;
    validate_attrs_usage(attrs)?;
    validate_attrs_order(attrs)?;

    Ok(())
}

pub fn host_includes() -> TokenStream {
    quote! {
        use svm_sdk::traits::Host;

        #[cfg(test)]
        use svm_sdk::host::MockHost as Node;

        #[cfg(not(test))]
        use svm_sdk::host::ExtHost as Node;
    }
}

pub fn expand_other_attrs(ast: TokenStream, _attrs: &[FuncAttribute]) -> Result<TokenStream> {
    Ok(ast)
}

pub fn expand_func(func: &Function, _attrs: &[FuncAttribute]) -> Result<TokenStream> {
    let ast = func.raw_func.to_token_stream();

    Ok(ast)
}

fn validate_attrs_no_dups(attrs: &[FuncAttribute]) -> Result<()> {
    let span = Span::call_site();

    let mut seen_endpoint = false;
    let mut seen_fundable = false;
    let mut seen_before_fund = false;

    for attr in attrs {
        match attr.kind() {
            FuncAttrKind::Endpoint => {
                if seen_endpoint {
                    return Err(Error::new(
                        span,
                        "Each function can be annotated with `#[endpoint]` exactly once.",
                    ));
                }
                seen_endpoint = true;
            }
            FuncAttrKind::BeforeFund => {
                if seen_before_fund {
                    return Err(Error::new(
                        span,
                        "Each function can be annotated with `#[before_fund]` exactly once.",
                    ));
                }
                seen_before_fund = true;
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

fn validate_attrs_usage(attrs: &[FuncAttribute]) -> Result<()> {
    let span = Span::call_site();
    let mut seen_endpoint = false;
    let mut seen_fundable = false;
    let mut seen_before_fund = false;

    for attr in attrs {
        match attr.kind() {
            FuncAttrKind::Endpoint => seen_endpoint = true,
            FuncAttrKind::BeforeFund => seen_before_fund = true,
            FuncAttrKind::Fundable => seen_fundable = true,
            FuncAttrKind::Other => continue,
        }
    }

    if seen_endpoint && seen_before_fund {
        return Err(Error::new(
            span,
            "#[endpoint]` and `#[before_fund]` can't co-exist.",
        ));
    }

    if seen_fundable && seen_before_fund {
        return Err(Error::new(
            span,
            "#[before_fund]` and `#[fundable(..)]` can't co-exist.",
        ));
    }

    if seen_fundable && !seen_endpoint {
        return Err(Error::new(
            span,
            "#[fundable(..)] can't be used without `#[endpoint]`",
        ));
    }

    Ok(())
}

fn validate_attrs_order(attrs: &[FuncAttribute]) -> Result<()> {
    let span = Span::call_site();
    let mut seen_endpoint = false;

    for attr in attrs {
        match attr.kind() {
            FuncAttrKind::Endpoint => seen_endpoint = true,
            FuncAttrKind::BeforeFund => continue,
            FuncAttrKind::Fundable => {
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

pub fn has_endpoint_attr(attrs: &[FuncAttribute]) -> bool {
    has_attr(attrs, FuncAttrKind::Endpoint)
}

pub fn has_before_fund_attr(attrs: &[FuncAttribute]) -> bool {
    has_attr(attrs, FuncAttrKind::BeforeFund)
}

pub fn has_fundable_attr(attrs: &[FuncAttribute]) -> bool {
    has_attr(attrs, FuncAttrKind::Fundable)
}

pub fn has_other_attr(attrs: &[FuncAttribute]) -> bool {
    has_attr(attrs, FuncAttrKind::Other)
}

pub fn has_attr(attrs: &[FuncAttribute], kind: FuncAttrKind) -> bool {
    attrs.iter().any(|attr| attr.kind() == kind)
}

pub fn find_attr(attrs: &[FuncAttribute], kind: FuncAttrKind) -> &FuncAttribute {
    let attr = attrs.iter().find(|attr| attr.kind() == kind);

    attr.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    use syn::parse_quote;

    macro_rules! assert_err {
        ($expected:expr, $($tt:tt)*) => {{
            let raw_func: ItemFn = parse_quote!( $($tt)* );
            let mut func = Function::new(raw_func);

            let actual = expand(&mut func).unwrap_err();
            assert_eq!($expected, actual.to_string());
        }};
    }

    macro_rules! assert_ok {
        ($($tt:tt)*) => {{
            let raw_func: ItemFn = parse_quote!( $($tt)* );

            let mut func = Function::new(raw_func);

            let res = expand(&mut func);
            assert!(res.is_ok());
        }};
    }

    #[test]
    fn fundable_can_not_live_alone() {
        let err = "#[fundable(..)] can't be used without `#[endpoint]`";

        assert_err!(
            err,
            #[fundable(deny)]
            fn deny() {}
        )
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
    fn endpoint_and_before_fund_fails() {
        let err = "#[endpoint]` and `#[before_fund]` can't co-exist.";

        assert_err!(
            err,
            #[before_fund]
            #[endpoint]
            fn get() {}
        );
    }

    #[test]
    fn before_fund_and_fundable_not_allowed() {
        let err = "#[endpoint]` and `#[before_fund]` can't co-exist.";

        assert_err!(
            err,
            #[before_fund]
            #[endpoint]
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
    fn before_fund_used_twice_fails() {
        let err = "Each function can be annotated with `#[before_fund]` exactly once.";

        assert_err!(
            err,
            #[before_fund]
            #[before_fund]
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
    fn before_fund_func_with_no_args_falis() {
        let err = "`#[before_fund]` annotated function should have signature of `fn(value: svm_sdk::Amount) -> ()`";

        assert_err!(
            err,
            #[before_fund]
            fn deny() {}
        );
    }

    #[test]
    fn before_fund_func_has_more_than_one_args_fails() {
        let err = "`#[before_fund]` annotated function should have signature of `fn(value: svm_sdk::Amount) -> ()`";

        assert_err!(
            err,
            #[before_fund]
            fn deny(a: svm_sdk::Amount, b: svm_sdk::Amount) {}
        );
    }

    #[test]
    fn before_fund_func_with_return_type_fails() {
        let err = "`#[before_fund]` annotated function should have signature of `fn(value: svm_sdk::Amount) -> ()`";

        assert_err!(
            err,
            #[before_fund]
            fn deny(v: svm_sdk::Amount) -> u32 {
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
    fn before_fund_func_valid_sig() {
        assert_ok!(
            #[before_fund]
            fn allow(v: svm_sdk::Amount) {}
        );

        assert_ok!(
            #[before_fund]
            fn allow(v: Amount) {}
        );
    }
}
