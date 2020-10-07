#![allow(unused)]

extern crate proc_macro;

use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Type};

#[proc_macro_derive(AppStorage)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.clone();
    let input = as_struct(input);
    let fields = take_fields(input);

    let vars = assign_vars(&fields);

    let getters = getters_ast(&fields, &vars);
    let setters = setters_ast(&fields, &vars);

    let new_name = Ident::new(&format!("{}Storage", name), Span::call_site());

    (quote! {
        #[derive(Debug)]
        struct #new_name;

        impl #new_name {
            #getters

            #setters
        }
    })
    .into()
}

enum StorageVar {
    Primitive(Ident),

    Array(Ident, usize),
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

fn assign_vars(fields: &FieldsNamed) -> HashMap<Ident, usize> {
    let mut vars = HashMap::new();
    let mut index = 0;

    for f in fields.named.iter() {
        let ty = field_type(f);

        match ty {
            StorageVar::Primitive(ident) => {
                vars.insert(ident, index);
                index += 1;
            }
            StorageVar::Array(ident, size) => {
                vars.insert(ident, index);
                index += size;
            }
        }
    }

    vars
}

fn getters_ast(fields: &FieldsNamed, vars: &HashMap<Ident, usize>) -> TokenStream {
    let mut getters = Vec::new();

    for f in fields.named.iter() {
        let ident = getter_ident(f);
        let ty = field_type(f);

        let getter = match ty {
            StorageVar::Primitive(ty) => {
                quote! {
                    pub fn #ident() -> #ty {
                        todo!()
                    }
                }
            }
            StorageVar::Array(ty, size) => {
                quote! {
                    //
                }
            }
        };

        getters.push(getter);
    }

    let ast = quote! {
        #(#getters)*
    };

    ast.into()
}

fn setters_ast(fields: &FieldsNamed, vars: &HashMap<Ident, usize>) -> TokenStream {
    for f in fields.named.iter() {}

    quote! {
        //
    }
}

fn field_type(field: &Field) -> StorageVar {
    // match &field.ty {
    //     Type::Array(array) => {}
    //     _ => panic!("Invalid Type"),
    // }

    todo!()
}

fn getter_ident(f: &Field) -> Ident {
    Ident::new(&format!("get_{}", field_ident(f)), Span::call_site())
}

fn getter_type(f: &Field) -> Ident {
    Ident::new("usize", Span::call_site())
}

fn setter_type(f: &Field) -> Ident {
    Ident::new("usize", Span::call_site())
}

fn setter_ident(f: &Field) -> Ident {
    Ident::new(&format!("set_{}", field_ident(f)), Span::call_site())
}

fn field_ident(f: &Field) -> Ident {
    f.ident.as_ref().unwrap().clone()
}

fn prim_getter_ast(name: Ident, ty: Ident) -> TokenStream {
    todo!()

    // match ty.to_string().as_str() {
    //     "bool" => {
    //         quote! {}
    //     }
    //     "i8" => {}
    //     "u8" => {}
    //     _ => todo!(),
    // };

    // let body_ast = quote! {
    //     todo!()
    // };

    // quote! {
    //     pub fn #name () -> #ty {
    //         #body_ast
    //     }
    // }
}
