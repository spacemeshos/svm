use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Block, Error, FnArg, ItemFn, Pat, PatType, Result, ReturnType, Signature, Type,
};

use crate::{attr, FuncAttrKind, FuncAttribute};

pub struct Function {
    raw: ItemFn,
}

impl Function {
    pub fn new(raw: ItemFn) -> Self {
        Self { raw }
    }

    pub fn name(&self) -> Ident {
        self.raw_sig().ident.clone()
    }

    pub fn raw_body(&self) -> &Block {
        &self.raw.block
    }

    pub fn raw_sig(&self) -> &Signature {
        &self.raw.sig
    }

    pub fn take_raw_attrs(&mut self) -> Vec<Attribute> {
        std::mem::replace(&mut self.raw.attrs, Vec::new())
    }

    pub fn stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();

        self.raw.to_tokens(&mut tokens);

        tokens
    }
}

pub fn func_attrs(func: &mut Function) -> Result<Vec<FuncAttribute>> {
    let mut attrs = Vec::new();

    for attr in func.take_raw_attrs() {
        let attr = attr::parse_attr(attr)?;

        attrs.push(attr);
    }

    Ok(attrs)
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

fn validate_attrs(attrs: &[FuncAttribute]) -> Result<()> {
    validate_attrs_no_dups(attrs)?;
    validate_attrs_usage(attrs)?;
    validate_attrs_order(attrs)?;

    Ok(())
}

fn expand_endpoint_attr(func: &Function, attrs: &[FuncAttribute]) -> Result<TokenStream> {
    debug_assert!(has_endpoint_attr(attrs));

    let name = func.name();
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

fn endpoint_includes() -> TokenStream {
    quote! {
        use svm_sdk::traits::Host;

        #[cfg(test)]
        use svm_sdk::host::MockHost as Node;

        #[cfg(not(test))]
        use svm_sdk::host::ExtHost as Node;
    }
}

fn expand_endpoint_prologue(func: &Function) -> Result<TokenStream> {
    let includes = endpoint_includes();

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
    let ast = quote! {
        {
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

    let ast = quote! {
        //
    };

    Ok(ast)
}

fn expand_before_fund_attr(func: &Function, attrs: &[FuncAttribute]) -> Result<TokenStream> {
    debug_assert!(has_before_fund_attr(attrs));

    let func = func.stream();

    let ast = quote! {
        #[inline]
        #func
    };

    Ok(ast)
}

fn expand_other_attrs(ast: TokenStream, attrs: &[FuncAttribute]) -> Result<TokenStream> {
    let ast = quote! {
        //
    };

    Ok(ast)
}

fn expand_func(func: &Function, _attrs: &[FuncAttribute]) -> Result<TokenStream> {
    let ast = func.raw.to_token_stream();

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
                        "Each function can be annotated with `#[fundable]` exactly once.",
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
            "`endpoint` can't use raw pointers for its parameters types",
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
                    "`endpoint` can't use raw pointers for its parameters types",
                )),
                Type::Reference(..) => Err(Error::new(
                    span,
                    "`endpoint` can't use reference for its parameters types",
                )),
                Type::Slice(..) => Err(Error::new(
                    span,
                    "`endpoint` can't use dynamically sized slices for its parameters types",
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

fn has_endpoint_attr(attrs: &[FuncAttribute]) -> bool {
    attrs
        .iter()
        .any(|attr| matches!(attr, FuncAttribute::Endpoint))
}

fn has_before_fund_attr(attrs: &[FuncAttribute]) -> bool {
    attrs
        .iter()
        .any(|attr| matches!(attr, FuncAttribute::BeforeFund))
}

fn has_fundable_attr(attrs: &[FuncAttribute]) -> bool {
    attrs
        .iter()
        .any(|attr| matches!(attr, FuncAttribute::Fundable(..)))
}

fn has_other_attr(attrs: &[FuncAttribute]) -> bool {
    attrs
        .iter()
        .any(|attr| matches!(attr, FuncAttribute::Other(..)))
}
