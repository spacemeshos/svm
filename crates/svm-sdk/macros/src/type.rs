use std::todo;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Error, Expr, ExprLit, Lit, Result, TypeArray, TypePath};

pub struct PrimType {
    ty_raw: syn::Type,

    ty_str: String,
}

impl ToTokens for PrimType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ty_raw().to_tokens(tokens)
    }
}

impl PrimType {
    pub fn new(path: &TypePath) -> Result<Self> {
        parse_primitive_type(path)
    }

    pub fn ty_raw(&self) -> &syn::Type {
        &self.ty_raw
    }

    pub fn as_str(&self) -> &str {
        &self.ty_str
    }
}

pub enum Type {
    Primitive(PrimType),

    Array {
        elem: PrimType,
        length: u32,
        elem_raw: syn::Type,
    },

    Tuple {
        elems: Vec<Box<Type>>,
    },
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Type::Primitive(prim) => prim.to_tokens(tokens),
            Type::Array { array_raw, .. } => array_raw.to_tokens(tokens),
            Type::Tuple { tuple_raw, .. } => tuple_raw.to_tokens(tokens),
        }
    }
}

impl Type {
    pub fn new(ty: &syn::Type) -> Result<Self> {
        match ty {
            syn::Type::Array(ty) => parse_array_type(ty),
            syn::Type::Path(ty) => {
                let prim = parse_primitive_type(ty)?;
                let ty = Type::Primitive(prim);

                Ok(ty)
            }
            syn::Type::Tuple(ty) => parse_tuple_type(ty),
            _ => unreachable!(),
        }
    }

    pub fn into_primitive(self) -> PrimType {
        match self {
            Type::Primitive(prim) => prim,
            _ => unreachable!(),
        }
    }
}

fn parse_primitive_type(path: &TypePath) -> Result<PrimType> {
    let ty_str = type_path_as_str(&path);

    match ty_str.as_str() {
        #[rustfmt::skip]
        "bool"    | 
        "Amount"  |
        "Address" |
        "svm_sdk :: Amount"  |
        "svm_sdk :: Address" |
        "i8"      |
        "u8"      |
        "i16"     |
        "u16"     |
        "i32"     |
        "u32"     |
        "i64"     |
        "u64"     => {
            let ty = syn::Type::Path(path.clone());
            let prim = PrimType { ty_raw: ty, ty_str};

            Ok(prim)
        }
        _ => {
            let span = Span::call_site();
            let msg = format!("Invalid `#[storage]` field type: {}", ty_str);

            Err(Error::new(span, msg))
        }
    }
}

fn parse_array_type(ty: &TypeArray) -> Result<Type> {
    let elem = parse_array_element_type(ty)?;
    let length = parse_array_length(ty)?;

    let elem_raw = Type::Array { elem, length };
    Ok(ty)
}

fn parse_tuple_type(tuple_raw: &syn::TypeTuple) -> Result<Type> {
    let mut elems = Vec::new();

    for elem in tuple_raw.elems.iter() {
        match elem {
            syn::Type::Path(path) => {
                let prim = parse_primitive_type(path)?;
                let elem = Type::Primitive(prim);
                elems.push(Box::new(elem));
            }
            syn::Type::Array(array) => {
                let elem = parse_array_type(array)?;
                elems.push(Box::new(elem));
            }
            _ => unreachable!(),
        };
    }

    let ty = Type::Tuple { elems, tuple_raw };
    Ok(ty)
}

fn parse_array_element_type(ty: &TypeArray) -> Result<PrimType> {
    match *ty.elem {
        syn::Type::Path(ref path) => parse_primitive_type(path),
        _ => {
            let span = Span::call_site();

            Err(Error::new(
                span,
                "`Array elements must be primitives (for example: `svm_sdk::Amount`).",
            ))
        }
    }
}

fn parse_array_length(array: &TypeArray) -> Result<u32> {
    if let Expr::Lit(ExprLit { attrs, lit }) = &array.len {
        assert!(attrs.is_empty());

        if let Lit::Int(num) = lit {
            let num = num.base10_parse();

            if num.is_ok() {
                return num;
            }
        }
    }

    let span = Span::call_site();
    let msg = "Invalid array length";

    Err(Error::new(span, msg))
}

fn type_path_as_str(path: &TypePath) -> String {
    let path = &path.path;
    let path = quote! { #path };

    path.to_string()
}
