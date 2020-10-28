extern crate proc_macro;

use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use quote::{quote, ToTokens};

use syn::parse::ParseStream;
use syn::{
    Data, DataStruct, DeriveInput, Expr, ExprLit, Field, Fields, FieldsNamed, Lit, Path,
    PathArguments, Type, TypeArray, TypePath,
};

#[derive(Debug, Clone)]
struct Param {
    name: Ident,

    ty: Ident,
}

#[derive(Debug, Clone)]
struct FuncSig {
    name: Ident,

    params: Vec<Param>,

    returns: TokenStream,
}

impl FuncSig {
    fn name(&self) -> Ident {
        self.name.clone()
    }

    fn params(&self) -> &[Param] {
        &self.params
    }

    fn returns(&self) -> &TokenStream {
        &self.returns
    }
}

pub fn parse_endpoint(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (fn_sig, next) = parse_func_sig(input.into());
    let body = parse_func_body(next);

    let name = fn_sig.name();
    let returns = fn_sig.returns();
    let prologue = func_prologue(&fn_sig);

    let includes = endpoint_includes();

    (quote! {
        fn #name() {
            #includes

            fn __inner__() #returns {
                #prologue

                #body
            }

            use svm_abi_encoder::Encoder;

            let mut bytes = Vec::new();

            let rets = __inner__();
            rets.encode(&mut bytes);

            Node.set_returndata(&bytes);
        }
    })
    .into()
}

fn parse_func_sig(mut input: TokenStream) -> (FuncSig, TokenTree) {
    let mut iter = input.into_iter();

    let name = parse_func_name(&mut iter);
    let params = parse_func_params(&mut iter);
    let (returns, next) = parse_func_returns(iter);

    let sig = FuncSig {
        name,
        params,
        returns,
    };

    (sig, next)
}

fn parse_func_name(iter: &mut IntoIter) -> Ident {
    let tt = iter.next();
    assert_ident_str(tt, "fn");

    let tt = iter.next();

    if let Some(TokenTree::Ident(name)) = tt {
        name
    } else {
        panic!("Expected function's name")
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

fn parse_func_returns(mut iter: IntoIter) -> (TokenStream, TokenTree) {
    let mut tts: Vec<TokenTree> = Vec::new();

    loop {
        let tt = iter.next();

        if tt.is_none() {
            panic!("Missing function body.")
        }

        let tt = tt.unwrap();

        let returns_end = match &tt {
            TokenTree::Group(g) => g.delimiter() == Delimiter::Brace,
            _ => false,
        };

        if returns_end {
            let tts = quote! {
                #(#tts)*
            };

            return (tts, tt);
        } else {
            tts.push(tt);
        }
    }

    unreachable!()
}
fn parse_func_body(body: TokenTree) -> TokenStream {
    if let TokenTree::Group(group) = body {
        assert_eq!(group.delimiter(), Delimiter::Brace);

        let stream = group.stream();
        let iter = stream.into_iter();

        iter.collect()
    } else {
        unreachable!()
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

fn endpoint_includes() -> TokenStream {
    quote! {
        use svm_sdk::host::traits::Host;

        #[cfg(test)]
        use svm_sdk::host::MockHost as Node;

        #[cfg(not(test))]
        use svm_sdk::host::ExtHost as Node;
    }
}
