use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Block, Error, ItemFn, Result, ReturnType, Signature, Type};

mod attr;
mod ctor;
mod endpoint;
mod fundable;
pub mod fundable_hook;

pub use attr::{
    find_attr, func_attrs, has_ctor_attr, has_default_fundable_hook_attr, has_endpoint_attr,
    has_fundable_attr, has_fundable_hook_attr,
};

pub use attr::{FuncAttr, FuncAttrKind};

use crate::program::Program;
use crate::Template;

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

    pub fn has_returns(&self) -> bool {
        let sig = self.raw_sig();

        match &sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_arrow, _ty) => true,
        }
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

pub fn expand(func: &Function, template: &Template) -> Result<TokenStream> {
    let attrs = func_attrs(func)?;

    validate_attrs(&attrs)?;

    let ast = if has_ctor_attr(&attrs) {
        ctor::expand(func, &attrs, template)?
    } else if has_endpoint_attr(&attrs) {
        endpoint::expand(func, &attrs, template)?
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
