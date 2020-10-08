#![allow(unused)]

extern crate proc_macro;

use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Path,
    PathArguments, Type,
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
        Type::Array(array) => todo!(),
        Type::Path(path) => {
            assert!(path.qself.is_none());

            let path = &path.path;
            assert_eq!(path.segments.len(), 1);

            let segment = &path.segments[0];
            assert!(matches!(segment.arguments, PathArguments::None));

            let name = field_ident(field);
            let ty = segment.ident.clone();

            match ty.to_string().as_str() {
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
                "AddressOwned" => {
                    Var::Primitive {
                        id,
                        name,
                        ty
                    }
                },
                _ => panic!("Invalid Storage field type: {}", ty),
            }
        }
        _ => panic!("Invalid Type"),
    }
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
    if let Var::Primitive { id, name, ty } = var {
        let getter_name = getter_ident(name);

        match ty.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" => {
                quote! {
                    fn #getter_name () -> #ty {
                        let v = svm_sdk::Storage::get32(#id);

                        v as #ty
                    }
                }
            }
            "u64" | "i64" => {
                quote! {
                    fn #getter_name () -> #ty {
                        let v = svm_sdk::Storage::get64(#id);

                        v as #ty
                    }
                }
            }
            "bool" => quote! {
                fn #getter_name () -> bool {
                    let v = svm_sdk::Storage::get32(0);

                    match v {
                        0 => false,
                        1 => true,
                        _ => unreachable!()
                    }
                }
            },
            "Amount" => quote! {
                fn #getter_name () -> svm_sdk::Amount {
                    let v = svm_sdk::Storage::get64(#id);

                    svm_sdk::Amount(v)
                }
            },
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}

fn setter_ast(var: &Var) -> TokenStream {
    if let Var::Primitive { id, name, ty } = var {
        let setter_name = setter_ident(name);

        match ty.to_string().as_str() {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" => {
                quote! {
                    fn #setter_name (value: #ty) {
                        svm_sdk::Storage::set32(#id, value);
                    }
                }
            }
            "u64" | "i64" => {
                quote! {
                    fn #setter_name (value: #ty) {
                        svm_sdk::Storage::set64(#id, value);
                    }
                }
            }
            "bool" => quote! {
                fn #setter_name (value: bool) {
                    match value {
                        true => svm_sdk::Storage::set32(#id, 1),
                        false => svm_sdk::Storage::set32(#id, 0),
                    }
                }
            },
            "Amount" => quote! {
                fn #setter_name (value: svm_sdk::Amount) {
                    let v = amount.0;
                    svm_sdk::Storage::set64(#id, v);
                }
            },
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}
