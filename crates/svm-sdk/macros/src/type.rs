use std::todo;

use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{Error, Expr, ExprLit, Lit, Result, TypeArray, TypePath};

pub struct PrimType {
    pub ty: syn::Type,

    pub ty_str: String,
}

pub enum Type {
    Primitive(PrimType),

    Array { elem: PrimType, length: u32 },

    Tuple { elems: Vec<Box<Type>> },
}

pub fn parse_type(ty: &syn::Type) -> Result<Type> {
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
            let prim = PrimType { ty, ty_str};

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

    let ty = Type::Array { elem, length };
    Ok(ty)
}

fn parse_tuple_type(ty: &syn::TypeTuple) -> Result<Type> {
    let mut elems: Vec<Box<Type>> = Vec::new();

    for elem in ty.elems.iter() {
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

    let ty = Type::Tuple { elems };
    Ok(ty)
}

fn parse_array_element_type(ty: &TypeArray) -> Result<PrimType> {
    match *ty.elem {
        syn::Type::Path(ref path) => parse_primitive_type(path),
        _ => {
            let span = Span::call_site();

            Err(Error::new(
                span,
                "`Array elements must be of type path (for example: `svm_sdk::Amount`).",
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
