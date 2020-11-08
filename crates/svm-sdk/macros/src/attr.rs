use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::{braced, token, Attribute, Ident, Result, Token};

#[derive(Debug)]
pub enum FuncAttribute {
    Endpoint,

    BeforeFund,

    Fundable(String),

    Other(TokenStream),
}

#[derive(Debug)]
pub enum FuncAttrKind {
    Endpoint,

    BeforeFund,

    Fundable,

    Other,
}

pub fn parse_attr(attr: &Attribute) -> Result<FuncAttribute> {
    let kind = parse_attr_kind(attr)?;

    let attr = match kind {
        FuncAttrKind::Endpoint => {
            assert!(attr.tokens.is_empty());

            FuncAttribute::Endpoint
        }
        FuncAttrKind::BeforeFund => {
            assert!(attr.tokens.is_empty());

            FuncAttribute::BeforeFund
        }
        FuncAttrKind::Fundable => {
            let tokens = attr.tokens.clone();
            let mut iter = tokens.into_iter();

            if let Some(TokenTree::Group(group)) = iter.next() {
                assert_eq!(group.delimiter(), Delimiter::Parenthesis);

                let stream = group.stream();
                let ident = syn::parse2::<Ident>(stream)?;

                FuncAttribute::Fundable(ident.to_string())
            } else {
                todo!("explain we expected a different input format")
            }
        }
        FuncAttrKind::Other => FuncAttribute::Other(quote! { #attr }),
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
            "before_fund" => FuncAttrKind::BeforeFund,
            _ => FuncAttrKind::Other,
        };

        Ok(kind)
    }
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

        let func_attr = parse_attr(&attr).unwrap();
        assert!(matches!(func_attr, FuncAttribute::Endpoint));
    }

    #[test]
    fn func_attr_before_fund() {
        let attr: Attribute = parse_quote! {
            #[before_fund]
        };

        let func_attr = parse_attr(&attr).unwrap();
        assert!(matches!(func_attr, FuncAttribute::BeforeFund));
    }

    #[test]
    fn func_attr_fundable() {
        let attr: Attribute = parse_quote! {
            #[fundable(deny_funding)]
        };

        let actual = parse_attr(&attr);
        let expected = FuncAttribute::Fundable("deny_funding".to_string());

        assert!(matches!(actual, expected));
    }

    #[test]
    fn func_attr_other() {
        let attr: Attribute = parse_quote! {
            #[derive(Debug, Copy, Clone)]
        };

        let attr = parse_attr(&attr).unwrap();
        assert!(matches!(attr, FuncAttribute::Other(..)));

        if let FuncAttribute::Other(tokens) = attr {
            assert_eq!(tokens.to_string(), "# [derive (Debug , Copy , Clone)]");
        } else {
            unreachable!()
        }
    }
}
