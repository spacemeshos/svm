use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Block, Error, FnArg, ItemFn, Pat, PatType, Result, ReturnType, Signature, Type,
};

use crate::{attr, FuncAttrKind, FuncAttribute};

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

fn rewrite_func(func: &mut Function) -> Result<TokenStream> {
    let attrs = func_attrs(func)?;

    validate_attrs(&attrs)?;

    let ast = if has_endpoint_attr(&attrs) {
        expand_endpoint_attr(func, &attrs)?
    } else if has_before_fund_attr(&attrs) {
        expand_before_fund_attr(func, &attrs)?
    } else {
        expand_func(func, &attrs)?
    };

    let ast = expand_other_attrs(ast, &attrs)?;

    Ok(ast)
}

fn func_attrs(func: &mut Function) -> Result<Vec<FuncAttribute>> {
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

fn expand_endpoint_attr(func: &Function, attrs: &[FuncAttribute]) -> Result<TokenStream> {
    debug_assert!(has_endpoint_attr(attrs));

    let name = func.raw_name();
    let prologue = expand_endpoint_prologue(func)?;
    let epilogue = expand_endpoint_epilogue(func)?;
    let returns = expand_endpoint_returns(func)?;
    let body = func.raw_body();

    let ast = quote! {
        #[no_mangle]
        pub extern "C" fn #name() {

            fn __inner__() #returns {
                #prologue

                #body
            }

            #epilogue
        }
    };

    let ast = if has_fundable_attr(attrs) {
        expand_fundable_attr(ast, &attrs)?
    } else {
        ast
    };

    Ok(ast)
}

fn host_includes() -> TokenStream {
    quote! {
        use svm_sdk::traits::Host;

        #[cfg(test)]
        use svm_sdk::host::MockHost as Node;

        #[cfg(not(test))]
        use svm_sdk::host::ExtHost as Node;
    }
}

fn expand_endpoint_prologue(func: &Function) -> Result<TokenStream> {
    let includes = host_includes();

    let init = quote! {
        let bytes = Node.get_calldata();
        let mut calldata = svm_sdk::CallData::new(bytes);
    };

    let mut assigns: Vec<TokenStream> = Vec::new();
    let sig = func.raw_sig();

    for input in &sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = input {
            let assign = quote! {
                let #pat: #ty = calldata.next_1();
            };

            assigns.push(assign.into());
        } else {
            unreachable!()
        }
    }

    let ast = quote! {
        #includes

        #init

        #(#assigns)*
    }
    .into();

    Ok(ast)
}

fn expand_endpoint_epilogue(func: &Function) -> Result<TokenStream> {
    let includes = host_includes();

    let ast = quote! {
        {
            #includes

            use svm_sdk::traits::Encoder;

            let mut bytes = Vec::new();

            let rets = __inner__();
            rets.encode(&mut bytes);

            Node.set_returndata(&bytes);
        }
    };

    Ok(ast)
}

fn expand_endpoint_returns(func: &Function) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();

    let sig = func.raw_sig();
    sig.output.to_tokens(&mut tokens);

    Ok(tokens)
}

fn expand_fundable_attr(ast: TokenStream, attrs: &[FuncAttribute]) -> Result<TokenStream> {
    debug_assert!(has_fundable_attr(attrs));

    let attr = find_attr(attrs, FuncAttrKind::Fundable);

    let fund_hook = match attr {
        FuncAttribute::Fundable(s) => s,
        _ => unreachable!(),
    };

    let includes = host_includes();

    let ast = quote! {
        {
            #includes;

            let value: svm_sdk::Amount = Node.get_value();

            #fund_hook(value);
        }
    };

    Ok(ast)
}

fn expand_before_fund_attr(func: &Function, attrs: &[FuncAttribute]) -> Result<TokenStream> {
    debug_assert!(has_before_fund_attr(attrs));

    validate_before_fund_func_sig(func)?;

    let func = func.stream();

    let ast = quote! {
        #[inline]
        #func
    };

    Ok(ast)
}

fn expand_other_attrs(ast: TokenStream, attrs: &[FuncAttribute]) -> Result<TokenStream> {
    Ok(ast)
}

fn expand_func(func: &Function, _attrs: &[FuncAttribute]) -> Result<TokenStream> {
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
                        ("Each function can be annotated with `#[before_fund]` exactly once."),
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
    let mut seen_fundable = false;

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

                seen_fundable = true;
            }
            FuncAttrKind::Other => continue,
        }
    }

    Ok(())
}

fn validate_endpoint_sig(func: &Function) -> Result<()> {
    let sig = func.raw_sig();
    let span = Span::call_site();

    if sig.constness.is_some() {
        return Err(Error::new(span, "`endpoint` function can't be `const`"));
    }

    if sig.asyncness.is_some() {
        return Err(Error::new(span, "`endpoint` function can't be `async`"));
    }

    if sig.unsafety.is_some() {
        return Err(Error::new(span, "`endpoint` function can't be `unsafe`"));
    }

    if sig.abi.is_some() {
        return Err(Error::new(span, "`endpoint` function can't be `extern`"));
    }

    if !sig.generics.params.is_empty() {
        return Err(Error::new(span, "`endpoint` function can't use generics."));
    }

    if sig.variadic.is_some() {
        return Err(Error::new(span, "`endpoint` function can't use variadics."));
    }

    if sig.receiver().is_some() {
        return Err(Error::new(span, "`endpoint` function can't use `self`"));
    }

    for arg in &sig.inputs {
        if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = arg {
            if !attrs.is_empty() {
                return Err(Error::new(span, "`endpoint` params can't have attributes."));
            }

            validate_arg_pat(pat)?;
            validate_arg_type(ty)?;
        } else {
            unreachable!()
        }
    }

    validate_ret_type(&sig.output)?;

    Ok(())
}

fn validate_arg_pat(pat: &Box<Pat>) -> Result<()> {
    match **pat {
        Pat::Type(..) => Ok(()),
        _ => {
            let span = Span::call_site();

            Err(Error::new(
                span,
                "`endpoint` parameters definitions are expected to be of pattern: `name: type`",
            ))
        }
    }
}

fn validate_arg_type(ty: &Box<Type>) -> Result<()> {
    let span = Span::call_site();

    match **ty {
        Type::BareFn(..) => Err(Error::new(
            span,
            "`endpoint` can't have a bare function as a parameter type",
        )),
        Type::ImplTrait(..) => Err(Error::new(
            span,
            "`endpoint` can't use an `impl` for its parameters types",
        )),
        Type::Macro(..) => Err(Error::new(
            span,
            "`endpoint` can't use an macros within it parameters types",
        )),
        Type::Never(..) => Err(Error::new(
            span,
            "`endpoint` can't use `!` for its parameters types",
        )),
        Type::Paren(..) => Err(Error::new(
            span,
            "`endpoint` can't use parentheses for its parameters types",
        )),
        Type::Ptr(..) => Err(Error::new(
            span,
            "`endpoint` can't use raw_func pointers for its parameters types",
        )),
        Type::Reference(..) => Err(Error::new(
            span,
            "`endpoint` can't use references for its parameters types",
        )),
        Type::Slice(..) => Err(Error::new(
            span,
            "`endpoint` can't use dynamically sized slices for its parameters types",
        )),
        Type::TraitObject(..) => Err(Error::new(
            span,
            "`endpoint` can't use trait objects for its parameters types",
        )),
        Type::Tuple(..) => Err(Error::new(
            span,
            "`endpoint` can't use right now tuples for its parameters types",
        )),
        _ => Ok(()),
    }
}

fn validate_ret_type(ty: &ReturnType) -> Result<()> {
    match ty {
        ReturnType::Default => Ok(()),
        ReturnType::Type(.., ty) => {
            let span = Span::call_site();

            match **ty {
                Type::BareFn(..) => Err(Error::new(
                    span,
                    "`endpoint` can't have a bare function as a return type",
                )),
                Type::ImplTrait(..) => Err(Error::new(
                    span,
                    "`endpoint` can't use an `impl` for its return type",
                )),
                Type::Macro(..) => Err(Error::new(
                    span,
                    "`endpoint` can't use an macros for its return type",
                )),
                Type::Never(..) => Err(Error::new(
                    span,
                    "`endpoint` can't use `!` for its parameters types",
                )),
                Type::Paren(..) => Err(Error::new(
                    span,
                    "`endpoint` can't use parentheses for its parameters types",
                )),
                Type::Ptr(..) => Err(Error::new(
                    span,
                    "`endpoint` can't use raw_func pointers for its parameters types",
                )),
                Type::Reference(..) => Err(Error::new(
                    span,
                    "`endpoint` can't use reference for its parameters types",
                )),
                Type::Slice(..) => Err(Error::new(
                    span,
                    "`endpoint` can't use dynamically-sized slices for its parameters types",
                )),
                Type::TraitObject(..) => Err(Error::new(
                    span,
                    "`endpoint` can't use trait objects for its parameters types",
                )),
                _ => Ok(()),
            }
        }
    }
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

fn has_endpoint_attr(attrs: &[FuncAttribute]) -> bool {
    has_attr(attrs, FuncAttrKind::Endpoint)
}

fn has_before_fund_attr(attrs: &[FuncAttribute]) -> bool {
    has_attr(attrs, FuncAttrKind::BeforeFund)
}

fn has_fundable_attr(attrs: &[FuncAttribute]) -> bool {
    has_attr(attrs, FuncAttrKind::Fundable)
}

fn has_other_attr(attrs: &[FuncAttribute]) -> bool {
    has_attr(attrs, FuncAttrKind::Other)
}

fn has_attr(attrs: &[FuncAttribute], kind: FuncAttrKind) -> bool {
    attrs.iter().any(|attr| attr.kind() == kind)
}

fn find_attr(attrs: &[FuncAttribute], kind: FuncAttrKind) -> &FuncAttribute {
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

            let actual = rewrite_func(&mut func).unwrap_err();
            assert_eq!($expected, actual.to_string());
        }};
    }

    macro_rules! assert_ok {
        ($($tt:tt)*) => {{
            let raw_func: ItemFn = parse_quote!( $($tt)* );

            let mut func = Function::new(raw_func);

            let res = rewrite_func(&mut func);
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
