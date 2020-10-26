extern crate proc_macro;

use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::ParseStream;
use syn::{
    Data, DataStruct, DeriveInput, Expr, ExprLit, Field, Fields, FieldsNamed, Lit, Path,
    PathArguments, Type, TypeArray, TypePath,
};

struct Param {
    name: Ident,
    ty: Ident,
}

struct FuncSig {
    name: Ident,

    params: Vec<Param>,
}

pub fn parse_endpoint(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let includes = includes_ast();

    let (fn_sig, iter) = parse_func_sig(input.into());
    let body = parse_func_body(iter);

    let name = &fn_sig.name;
    let prologue = func_prologue(&fn_sig);

    (quote! {
        #includes

        fn #name() {
            #prologue

            #body
        }
    })
    .into()
}

fn parse_func_sig(mut input: TokenStream) -> (FuncSig, IntoIter) {
    let mut iter = input.into_iter();

    let name = parse_func_name(&mut iter);
    let params = parse_func_params(&mut iter);

    let sig = FuncSig { name, params };

    (sig, iter)
}

fn parse_func_name(iter: &mut IntoIter) -> Ident {
    let tt = iter.next();
    assert_ident_str(tt, "fn");

    let tt = iter.next();

    if let Some(TokenTree::Ident(name)) = tt {
        name
    } else {
        panic!("Expected function name")
    }
}

fn parse_func_params(iter: &mut IntoIter) -> Vec<Param> {
    let tt = iter.next().unwrap();

    if let TokenTree::Group(group) = tt {
        assert_eq!(group.delimiter(), Delimiter::Parenthesis);

        let mut params = Vec::new();
        let stream = group.stream();
        let mut iter = stream.into_iter();

        let colon = Punct::new(':', Spacing::Alone);
        let comma = Punct::new(',', Spacing::Alone);

        loop {
            let name = iter.next();
            if name.is_none() {
                break;
            }

            let tt = iter.next();
            assert_punct(tt, &colon);

            let ty = iter.next();
            if ty.is_none() {
                panic!("Expected function parameter type.");
            }

            let param = Param {
                name: as_ident(name),
                ty: as_ident(ty),
            };
            params.push(param);

            let tt = iter.next();
            if tt.is_some() {
                assert_punct(tt, &comma);
            } else {
                break;
            }
        }

        params
    } else {
        panic!("Expected parenthesis after function's name.")
    }
}

fn parse_func_body(mut iter: IntoIter) -> TokenStream {
    let tt = iter.next();

    if let Some(TokenTree::Group(group)) = tt {
        assert_eq!(group.delimiter(), Delimiter::Brace);

        let stream = group.stream();
        let iter = stream.into_iter();

        iter.collect()
    } else {
        panic!("`endpoint` can't have a return value (use `returncalldata` instead).")
    }
}

fn func_prologue(sig: &FuncSig) -> TokenStream {
    let mut assigns: Vec<TokenStream> = Vec::new();

    let init = quote! {
        let bytes = Node.get_calldata();

        let mut calldata = svm_abi_decoder::CallData::new(bytes);
    };

    for param in sig.params.iter() {
        let name = &param.name;
        let ty = &param.ty;

        let assign = quote! {
            let #name: #ty = calldata.next_1();
        };

        assigns.push(assign.into());
    }

    (quote! {
        #init

        #(#assigns)*
    })
    .into()
}

fn as_ident(tt: Option<TokenTree>) -> Ident {
    if let Some(TokenTree::Ident(ident)) = tt {
        ident
    } else {
        panic!("Expected ident.")
    }
}

fn assert_ident_str(tt: Option<TokenTree>, expected: &str) {
    if let Some(TokenTree::Ident(actual)) = tt {
        let actual = format!("{}", actual);
        let expected = format!("{}", expected);

        if actual.to_string() != expected {
            panic!(format!("Expected literal: {}, got: {}", expected, actual));
        }
    } else {
        panic!(format!("Expected literal: {}", expected));
    }
}

fn assert_literal(tt: Option<TokenTree>, expected: Literal) {
    if let Some(TokenTree::Literal(actual)) = tt {
        let actual = format!("{}", actual);
        let expected = format!("{}", expected);

        if actual != expected {
            panic!(format!("Expected literal: {}, got: {}", expected, actual));
        }
    } else {
        panic!(format!("Expected literal: {}", expected));
    }
}

fn assert_punct(tt: Option<TokenTree>, expected: &Punct) {
    if let Some(TokenTree::Punct(actual)) = tt {
        assert_eq!(actual.spacing(), expected.spacing());

        let actual = actual.as_char();
        let expected = expected.as_char();

        if actual != expected {
            panic!(format!(
                "Expected punctuation: {}, got: {}",
                expected, actual
            ));
        }
    } else {
        panic!(format!("Expected punctuation: {}", expected));
    }
}

fn includes_ast() -> TokenStream {
    quote! {
        use svm_sdk::host::Host;

        #[cfg(test)]
        use svm_sdk::host::MockHost as Node;

        #[cfg(not(test))]
        use svm_sdk::host::ExtHost as Node;

        use svm_sdk::{Amount, Address, LayerId, ensure, log};
    }
}
