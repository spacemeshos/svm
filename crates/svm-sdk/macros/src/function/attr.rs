use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Error, Ident, Result};

use crate::Function;

#[derive(Debug, PartialEq)]
pub enum FuncAttrKind {
    Endpoint,

    Fundable,

    FundableHook,

    Other,
}

#[derive(Debug)]
pub enum FuncAttr {
    Endpoint,

    Fundable(String),

    FundableHook,

    Other(TokenStream),
}

impl FuncAttr {
    pub fn kind(&self) -> FuncAttrKind {
        match self {
            FuncAttr::Endpoint => FuncAttrKind::Endpoint,
            FuncAttr::FundableHook => FuncAttrKind::FundableHook,
            FuncAttr::Fundable(..) => FuncAttrKind::Fundable,
            FuncAttr::Other(..) => FuncAttrKind::Other,
        }
    }
}

pub fn func_attrs(func: &Function) -> Result<Vec<FuncAttr>> {
    let mut attrs = Vec::new();

    for attr in func.raw_attrs() {
        let attr = parse_attr(attr)?;

        attrs.push(attr);
    }

    Ok(attrs)
}

pub fn parse_attr(attr: Attribute) -> Result<FuncAttr> {
    let kind = parse_attr_kind(&attr)?;

    let attr = match kind {
        FuncAttrKind::Endpoint => {
            assert!(attr.tokens.is_empty());

            FuncAttr::Endpoint
        }
        FuncAttrKind::FundableHook => {
            assert!(attr.tokens.is_empty());

            FuncAttr::FundableHook
        }
        FuncAttrKind::Fundable => {
            let tokens = attr.tokens;
            let mut iter = tokens.into_iter();

            if let Some(TokenTree::Group(group)) = iter.next() {
                assert_eq!(group.delimiter(), Delimiter::Parenthesis);

                let stream = group.stream();
                let ident = syn::parse2::<Ident>(stream)?;

                FuncAttr::Fundable(ident.to_string())
            } else {
                let span = Span::call_site();

                return Err(Error::new(
                    span,
                    "`fundable` attribute should be of format `#[fundable(hook-fn)]`",
                ));
            }
        }
        FuncAttrKind::Other => FuncAttr::Other(quote! { #attr }),
    };

    Ok(attr)
}

fn parse_attr_kind(attr: &Attribute) -> Result<FuncAttrKind> {
    let mut tokens = TokenStream::new();

    let path = &attr.path;
    path.to_tokens(&mut tokens);

    syn::parse2::<FuncAttrKind>(tokens)
}

impl Parse for FuncAttrKind {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let ident_str = format!("{}", ident);
        let ident_str = ident_str.as_str();

        let kind = match ident_str {
            "endpoint" => FuncAttrKind::Endpoint,
            "fundable" => FuncAttrKind::Fundable,
            "fundable_hook" => FuncAttrKind::FundableHook,
            _ => FuncAttrKind::Other,
        };

        Ok(kind)
    }
}

pub fn has_endpoint_attr(attrs: &[FuncAttr]) -> bool {
    has_attr(attrs, FuncAttrKind::Endpoint)
}

pub fn has_fundable_hook_attr(attrs: &[FuncAttr]) -> bool {
    has_attr(attrs, FuncAttrKind::FundableHook)
}

pub fn has_fundable_attr(attrs: &[FuncAttr]) -> bool {
    has_attr(attrs, FuncAttrKind::Fundable)
}

pub fn has_other_attr(attrs: &[FuncAttr]) -> bool {
    has_attr(attrs, FuncAttrKind::Other)
}

pub fn has_attr(attrs: &[FuncAttr], kind: FuncAttrKind) -> bool {
    attrs.iter().any(|attr| attr.kind() == kind)
}

pub fn find_attr(attrs: &[FuncAttr], kind: FuncAttrKind) -> &FuncAttr {
    let attr = attrs.iter().find(|attr| attr.kind() == kind);

    attr.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    use proc_macro2::TokenStream;

    use quote::quote;
    use quote::ToTokens;

    use syn::{parse_quote, Attribute};

    #[test]
    fn func_attr_endpoint() {
        let attr: Attribute = parse_quote! {
            #[endpoint]
        };

        let func_attr = parse_attr(attr).unwrap();
        assert!(matches!(func_attr, FuncAttr::Endpoint));

        assert_eq!(func_attr.kind(), FuncAttrKind::Endpoint);
    }

    #[test]
    fn func_attr_fundable_hook() {
        let attr: Attribute = parse_quote! {
            #[fundable_hook]
        };

        let func_attr = parse_attr(attr).unwrap();
        assert!(matches!(func_attr, FuncAttr::FundableHook));

        assert_eq!(func_attr.kind(), FuncAttrKind::FundableHook);
    }

    #[test]
    fn func_attr_fundable() {
        let attr: Attribute = parse_quote! {
            #[fundable(deny_funding)]
        };

        let actual = parse_attr(attr).unwrap();
        assert_eq!(actual.kind(), FuncAttrKind::Fundable);

        let expected = FuncAttr::Fundable("deny_funding".to_string());
        assert!(matches!(actual, expected));
    }

    #[test]
    fn func_attr_fundable_without_hook() {
        let attr: Attribute = parse_quote! {
            #[fundable]
        };

        let err = parse_attr(attr).unwrap_err();

        assert_eq!(
            err.to_string(),
            "`fundable` attribute should be of format `#[fundable(hook-fn)]`"
        );
    }

    #[test]
    fn func_attr_other() {
        let attr: Attribute = parse_quote! {
            #[derive(Debug, Copy, Clone)]
        };

        let func_attr = parse_attr(attr).unwrap();
        assert_eq!(func_attr.kind(), FuncAttrKind::Other);

        if let FuncAttr::Other(tokens) = func_attr {
            assert_eq!(tokens.to_string(), "# [derive (Debug , Copy , Clone)]");
        } else {
            unreachable!()
        }
    }
}
