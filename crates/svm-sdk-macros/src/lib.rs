#![allow(unused)]

extern crate proc_macro;

use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Expr, ExprLit, Field, Fields, FieldsNamed,
    Lit, Path, PathArguments, Type, TypeArray, TypePath,
};

#[proc_macro_derive(AppStorage)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = storage_name(&input.ident);
    let input = as_struct(input);
    let fields = take_fields(input);
    let vars = assign_vars(&fields);

    let getters = getters_ast(&vars);
    let setters = setters_ast(&vars);

    (quote! {
        #[derive(Debug)]
        struct #name;

        impl #name {
            #getters

            #setters
        }
    })
    .into()
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Hash, Copy, Clone)]
struct VarId(u32);

impl ToTokens for &VarId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        tokens.extend(quote! { #id });
    }
}

macro_rules! ident_as_str {
    ($ident:expr) => {
        $ident.to_string().as_str()
    };
}

enum Var {
    Primitive {
        id: VarId,
        name: Ident,
        ty: Ident,
    },
    Array {
        id: VarId,
        name: Ident,
        ty: Ident,
        size: u32,
    },
}

fn as_struct(input: DeriveInput) -> DataStruct {
    match input.data {
        Data::Struct(s) => s,
        _ => panic!("macro can decorate only a `Struct`"),
    }
}

fn take_fields(input: DataStruct) -> FieldsNamed {
    let fields = input.fields;

    match fields {
        Fields::Named(fields) => fields,
        _ => panic!("Struct fields should be named"),
    }
}

fn assign_vars(fields: &FieldsNamed) -> Vec<Var> {
    let mut vars = Vec::new();
    let mut index = 0;

    for f in fields.named.iter() {
        let id = VarId(index);
        let var = field_as_var(id, f);

        match &var {
            Var::Primitive { id, name, ty } => {
                index += 1;
            }
            Var::Array { id, name, ty, size } => {
                index += size;
            }
        }

        vars.push(var);
    }

    vars
}

fn getters_ast(vars: &[Var]) -> TokenStream {
    let mut getters = Vec::new();

    for var in vars.iter() {
        let ast = getter_ast(var);

        getters.push(ast);
    }

    let ast = quote! {
        #(#getters)*
    };

    ast.into()
}

fn setters_ast(vars: &[Var]) -> TokenStream {
    let mut setters = Vec::new();

    for var in vars.iter() {
        let ast = setter_ast(var);

        setters.push(ast);
    }

    let ast = quote! {
        #(#setters)*
    };

    ast.into()
}

fn field_as_var(id: VarId, field: &Field) -> Var {
    match &field.ty {
        Type::Array(array) => {
            let ty = parse_array_elem_type(&array);
            let size = parse_array_length(&array);
            let name = field_ident(field);

            Var::Array { id, name, ty, size }
        }
        Type::Path(path) => {
            let name = field_ident(field);
            let ty = parse_type_path(path);

            Var::Primitive { id, name, ty }
        }
        _ => panic!("Invalid Type"),
    }
}

fn parse_array_elem_type(array: &TypeArray) -> Ident {
    match *array.elem {
        Type::Path(ref path) => parse_type_path(path),
        _ => panic!("Invalid array type"),
    }
}

fn parse_array_length(array: &TypeArray) -> u32 {
    match &array.len {
        Expr::Lit(ExprLit { attrs, lit }) => {
            assert!(attrs.is_empty());

            match lit {
                Lit::Int(int) => {
                    let int = int.base10_parse();

                    match int {
                        Ok(int) => return int,
                        Err(..) => panic!("Invalid array length"),
                    }
                }
                _ => panic!("Invalid array length"),
            }
        }
        _ => panic!("Invalid array length"),
    }
}

fn parse_type_path(path: &TypePath) -> Ident {
    assert!(path.qself.is_none());

    let path = &path.path;
    let ty = segments_path_as_ident(&path);

    match ident_as_str!(ty) {
        #[rustfmt::skip]
                "bool"    | 
                "Amount"  |
                "i8"      |
                "u8"      |
                "i16"     |
                "u16"     |
                "i32"     |
                "u32"     |
                "i64"     |
                "u64"     |
                "Address" |
                "AddressOwned" => ty.clone(),
        _ => panic!("Invalid Storage field type: {}", ty),
    }
}

fn segments_path_as_ident(path: &Path) -> Ident {
    match path.segments.len() {
        1 => single_segment_path_ident(path),
        2 => double_segments_path_ident(path),
        _ => panic!("Invalid field type"),
    }
}

fn single_segment_path_ident(path: &Path) -> Ident {
    path_segment_as_ident(path, 0)
}

fn double_segments_path_ident(path: &Path) -> Ident {
    todo!()
    // let first = path_segment_as_ident(path, 0);
    // let second = path_segment_as_ident(path, 1);

    // let merged = format!("{}::{}", first, second);

    // Ident::new(&merged, Span::call_site())
}

fn path_segment_as_ident(path: &Path, index: usize) -> Ident {
    debug_assert!(path.segments.len() > index);

    let segment = &path.segments[index];
    assert!(matches!(segment.arguments, PathArguments::None));

    segment.ident.clone()
}

fn getter_ident(var_name: &Ident) -> Ident {
    Ident::new(&format!("get_{}", var_name), Span::call_site())
}

fn setter_ident(var_name: &Ident) -> Ident {
    Ident::new(&format!("set_{}", var_name), Span::call_site())
}

fn field_ident(f: &Field) -> Ident {
    f.ident.as_ref().unwrap().clone()
}

fn storage_name(name: &Ident) -> Ident {
    Ident::new(&format!("{}Storage", name), Span::call_site())
}

fn getter_ast(var: &Var) -> TokenStream {
    let includes = include_storage_ast();

    match var {
        Var::Primitive { id, name, ty } => {
            let getter_name = getter_ident(name);

            match ident_as_str!(ty) {
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" => {
                    quote! {
                        fn #getter_name () -> #ty {
                            #includes

                            let v = Storage::get32(#id);
                            v as #ty
                        }
                    }
                }
                "u64" | "i64" => {
                    quote! {
                        fn #getter_name () -> #ty {
                            #includes

                            let v = Storage::get64(#id);
                            v as #ty
                        }
                    }
                }
                "bool" => quote! {
                    fn #getter_name () -> bool {
                        #includes

                        let v = Storage::get32(#id);
                        match v {
                            0 => false,
                            1 => true,
                            _ => unreachable!()
                        }
                    }
                },
                "Amount" => quote! {
                    fn #getter_name () -> svm_sdk::Amount {
                        #includes

                        let v = Storage::get64(#id);
                        svm_sdk::Amount(v)
                    }
                },
                "AddressOwned" => quote! {
                    fn #getter_name () -> svm_sdk::value::AddressOwned {
                        #includes

                        let offset = svm_sdk::memory::alloc(20);
                        Storage::load160(#id, offset);

                        let slice = unsafe {
                            core::slice::from_raw_parts(offset as *const u8, 20)
                        };

                        slice.into()
                    }
                },
                _ => unreachable!(),
            }
        }
        Var::Array { id, name, ty, size } => {
            let getter_name = getter_ident(name);

            match ident_as_str!(ty) {
                "AddressOwned" => quote! {
                    fn #getter_name (index: usize) -> svm_sdk::value::AddressOwned {
                        #includes

                        todo!()
                    }
                },
                _ => unreachable!(),
            }
        }
    }
}

fn setter_ast(var: &Var) -> TokenStream {
    let includes = include_storage_ast();

    match var {
        Var::Primitive { id, name, ty } => {
            let setter_name = setter_ident(name);

            match ident_as_str!(ty) {
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" => {
                    quote! {
                        fn #setter_name (value: #ty) {
                            #includes

                            Storage::set32(#id, value as u32);
                        }
                    }
                }
                "u64" | "i64" => {
                    quote! {
                        fn #setter_name (value: #ty) {
                            #includes

                            Storage::set64(#id, value as u64);
                        }
                    }
                }
                "bool" => quote! {
                    fn #setter_name (value: bool) {
                        #includes

                        match value {
                            true => Storage::set32(#id, 1),
                            false => Storage::set32(#id, 0),
                        }
                    }
                },
                "Amount" => quote! {
                    fn #setter_name (amount: svm_sdk::Amount) {
                        #includes

                        Storage::set64(#id, amount.0);
                    }
                },
                "AddressOwned" => quote! {
                    fn #setter_name (addr: &svm_sdk::value::AddressOwned) {
                        #includes

                        let off = addr.offset();
                        Storage::store160(#id, off);
                    }
                },
                _ => unreachable!(),
            }
        }
        Var::Array { id, name, ty, size } => {
            let setter_name = setter_ident(name);

            match ident_as_str!(ty) {
                "AddressOwned" => quote! {
                    fn #setter_name(addr: &svm_sdk::value::AddressOwned, index: usize) {
                        todo!()
                    }
                },
                _ => unreachable!(),
            }
        }
    }
}

fn include_storage_ast() -> TokenStream {
    quote! {
        #[cfg(test)]
        use svm_sdk::MockStorage as Storage;

        #[cfg(not(test))]
        use svm_sdk::ExtStorage as Storage;
    }
}
