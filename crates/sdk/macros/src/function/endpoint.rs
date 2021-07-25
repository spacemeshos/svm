use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Error, FnArg, Pat, PatType, Result, ReturnType, Type};

use super::{attr, fundable};
use attr::{has_endpoint_or_ctor_attr, has_fundable_attr, FuncAttr};

use crate::{function, Function, Template};

pub fn expand(func: &Function, attrs: &[FuncAttr], template: &Template) -> Result<TokenStream> {
    debug_assert!(has_endpoint_or_ctor_attr(attrs));

    validate_sig(func)?;

    let name = func.raw_name();
    let prologue = expand_prologue(func)?;
    let epilogue = expand_epilogue(func)?;
    let returns = expand_returns(func)?;
    let body = func.raw_body();

    let call_fundable_hook = if has_fundable_attr(attrs) {
        fundable::expand(&attrs, template)?
    } else {
        quote! {}
    };

    fn func_attrs(func: &Function) -> TokenStream {
        if cfg!(target_arch = "wasm32") {
            let export_name = func.export_name();

            quote! { #[export_name = #export_name] }
        } else {
            quote! { #[no_mangle] }
        }
    }

    let attrs = func_attrs(func);

    let ast = quote! {
        #attrs
        pub extern "C" fn #name() {
            #call_fundable_hook

            fn __inner__() #returns {
                #prologue

                #body
            }

            #epilogue
        }
    };

    Ok(ast)
}

fn expand_prologue(func: &Function) -> Result<TokenStream> {
    let sig = func.raw_sig();

    if sig.inputs.is_empty() {
        return Ok(quote! {});
    }

    let calldata = quote! {
        let bytes: &'static [u8] = Node.calldata();

        let mut calldata = svm_sdk::CallData::new(bytes);
    };

    let mut assigns: Vec<TokenStream> = Vec::new();

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

    let includes = function::host_includes();

    let ast = quote! {
        #includes

        #calldata

        #(#assigns)*
    }
    .into();

    Ok(ast)
}

fn expand_returns_size(func: &Function) -> Result<TokenStream> {
    let sig = func.raw_sig();
    let mut ty_tokens = TokenStream::new();

    match &sig.output {
        ReturnType::Type(.., ty) => ty.to_tokens(&mut ty_tokens),
        ReturnType::Default => unreachable!(),
    };

    // We derive in compile-time the byte-capacity to be fed into the `returndata` buffer
    // (when doing `Vec::with_capacity(..)`)
    let ast = quote! {
        {
            use svm_sdk::traits::ByteSize;

            < #ty_tokens > :: max_byte_size()
        }
    };

    Ok(ast)
}

fn expand_epilogue(func: &Function) -> Result<TokenStream> {
    let ast = if func.has_returns() {
        let includes = function::host_includes();
        let returns_size = expand_returns_size(func)?;

        quote! {
            {
                #includes

                use svm_sdk::traits::Encoder;

                let returns = __inner__();

                let capacity = #returns_size;

                let mut bytes: svm_sdk::Vec<u8> = svm_sdk::Vec::with_capacity(capacity);

                returns.encode(&mut bytes);

                if bytes.len() > 0 {
                    let bytes: &'static [u8] = bytes.leak();

                    Node.set_returndata(bytes);
                }
            }
        }
    } else {
        // Function has no returns (it returns `()` if to be more precise)
        // We don't want to encode `()` as the `returndata` (even though we could)
        quote! {
            {
                let _: () = __inner__();
            }
        }
    };

    Ok(ast)
}

fn expand_returns(func: &Function) -> Result<TokenStream> {
    let mut tokens = TokenStream::new();

    let sig = func.raw_sig();
    sig.output.to_tokens(&mut tokens);

    Ok(tokens)
}

fn validate_sig(func: &Function) -> Result<()> {
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
        Pat::Ident(..) => Ok(()),
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
