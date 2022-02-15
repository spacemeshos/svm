use proc_macro2::{Ident, Span, TokenStream};

use quote::quote;
use syn::{Error, Field, Fields, Result};

use super::{attr, Var, VarId};
use attr::{has_storage_attr, StructAttr};

use crate::{PrimType, Struct, Type};

pub fn expand(strukt: &Struct, attrs: &[StructAttr], must_mock: bool) -> Result<TokenStream> {
    debug_assert!(has_storage_attr(attrs));

    let vars = storage_vars(strukt)?;

    let name = strukt.raw_name();
    let getters = getters_ast(&vars, must_mock);
    let setters = setters_ast(&vars, must_mock);

    let ast = quote! {
        struct #name;

        impl #name {
            #getters

            #setters
        }
    };

    Ok(ast)
}

pub fn storage_vars(strukt: &Struct) -> Result<Vec<Var>> {
    let mut vars = Vec::new();
    let mut id = VarId(0);
    let mut offset = 0;

    let fields = strukt.raw_fields();

    ensure_named_fields(fields)?;

    for f in fields {
        let var = field_var(f, id, offset)?;

        match var {
            Var::Primitive { .. } => {
                offset += var.byte_count();
                id = next_var(id, 1);
            }
            Var::Array { length, .. } => {
                offset += var.byte_count() * (length as usize);
                id = next_var(id, length);
            }
        }

        vars.push(var);
    }

    Ok(vars)
}

fn field_var(field: &Field, id: VarId, offset: usize) -> Result<Var> {
    let span = Span::call_site();

    if !field.attrs.is_empty() {
        let msg = "`#[storage]` fields should have no attributes.";

        return Err(Error::new(span, msg));
    }

    let name = field_ident(field);
    let ty = Type::new(&field.ty)?;

    let var = match ty {
        Type::Array {
            elem_ty, length, ..
        } => {
            let byte_count = field_byte_count(&elem_ty);

            Var::Array {
                id,
                name,
                elem_ty,
                length,
                offset,
                byte_count,
            }
        }
        Type::Primitive(ty) => {
            let byte_count = field_byte_count(&ty);

            Var::Primitive {
                id,
                name,
                ty,
                offset,
                byte_count,
            }
        }
        _ => {
            return Err(Error::new(
                span,
                "`#[storage]` supports only Primitive (for example: `svm_sdk::Amount`) and Array types.",
            ));
        }
    };

    Ok(var)
}

fn ensure_named_fields(fields: &Fields) -> Result<()> {
    if let Fields::Named(..) = fields {
        Ok(())
    } else {
        let span = Span::call_site();

        Err(Error::new(
            span,
            "#[storage] annotated struct must have named fields.",
        ))
    }
}

fn getters_ast(vars: &[Var], must_mock: bool) -> TokenStream {
    let mut getters = Vec::new();

    for var in vars.iter() {
        let ast = getter_ast(var, must_mock);

        getters.push(ast);
    }

    quote! {
        #(#getters)*
    }
}

fn setters_ast(vars: &[Var], must_mock: bool) -> TokenStream {
    let mut setters = Vec::new();

    for var in vars.iter() {
        let ast = setter_ast(var, must_mock);

        setters.push(ast);
    }

    quote! {
        #(#setters)*
    }
}

fn getter_ast(var: &Var, must_mock: bool) -> TokenStream {
    let includes = include_storage_ast(must_mock);

    match var {
        Var::Primitive { id, name, ty, .. } => {
            let getter_name = getter_ident(name);

            match ty.as_str() {
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" => {
                    quote! {
                        fn #getter_name () -> #ty {
                            #includes

                            svm_sdk::storage::ops::get32::<StorageImpl>(#id) as #ty
                        }
                    }
                }
                "u64" | "i64" => {
                    quote! {
                        fn #getter_name () -> #ty {
                            #includes

                            svm_sdk::storage::ops::get64::<StorageImpl>(#id) as #ty
                        }
                    }
                }
                "bool" => {
                    quote! {
                        fn #getter_name () -> bool {
                            #includes

                            svm_sdk::storage::ops::get_bool::<StorageImpl>(#id) as #ty
                        }
                    }
                }
                "svm_sdk :: Amount" | "Amount" => {
                    quote! {
                        fn #getter_name () -> svm_sdk::Amount {
                            #includes

                            svm_sdk::storage::ops::get_amount::<StorageImpl>(#id)
                        }
                    }
                }
                "svm_sdk :: Address" | "Address" => {
                    quote! {
                        fn #getter_name () -> svm_sdk::Address {
                            #includes

                            svm_sdk::storage::ops::get_addr::<StorageImpl>(#id)
                        }
                    }
                }
                ty => panic!("Type `{}` is not supported", ty),
            }
        }
        Var::Array {
            id,
            name,
            elem_ty,
            length,
            ..
        } => {
            let getter_name = getter_ident(name);

            match elem_ty.as_str() {
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" => {
                    quote! {
                        fn #getter_name (index: usize) -> #elem_ty {
                            #includes

                            let value = svm_sdk::storage::ops::array_get32::<StorageImpl>(#id, index, #length);
                            value as #elem_ty
                        }
                    }
                }
                "u64" | "i64" => {
                    quote! {
                        fn #getter_name (index: usize) -> #elem_ty {
                            #includes

                            let value = svm_sdk::storage::ops::array_get64::<StorageImpl>(#id, index, #length);
                            value as #elem_ty
                        }
                    }
                }
                "bool" => {
                    quote! {
                        fn #getter_name (index: usize) -> bool {
                            #includes

                            svm_sdk::storage::ops::array_get_bool::<StorageImpl>(#id, index, #length)
                        }
                    }
                }
                "svm_sdk :: Amount" | "Amount" => quote! {
                    fn #getter_name (index: usize) -> svm_sdk::Amount {
                        #includes

                        svm_sdk::storage::ops::array_get_amount::<StorageImpl>(#id, index, #length)
                    }
                },
                "svm_sdk :: Address" | "Address" => quote! {
                    fn #getter_name (index: usize) -> svm_sdk::Address {
                        #includes

                        svm_sdk::storage::ops::array_get_addr::<StorageImpl>(#id, index, #length)
                    }
                },
                ty => panic!("Type `{}` is not supported", ty),
            }
        }
    }
}

fn setter_ast(var: &Var, must_mock: bool) -> TokenStream {
    let includes = include_storage_ast(must_mock);

    match var {
        Var::Primitive { id, name, ty, .. } => {
            let setter_name = setter_ident(name);

            match ty.as_str() {
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" => {
                    quote! {
                        fn #setter_name (value: #ty) {
                            #includes

                            svm_sdk::storage::ops::set32::<StorageImpl>(#id, value as u32);
                        }
                    }
                }
                "u64" | "i64" => {
                    quote! {
                        fn #setter_name (value: #ty) {
                            #includes

                            svm_sdk::storage::ops::set64::<StorageImpl>(#id, value as u64);
                        }
                    }
                }
                "bool" => quote! {
                    fn #setter_name (value: bool) {
                        #includes

                        svm_sdk::storage::ops::set_bool::<StorageImpl>(#id, value);
                    }
                },
                "svm_sdk :: Amount" | "Amount" => quote! {
                    fn #setter_name (value: svm_sdk::Amount) {
                        #includes

                        svm_sdk::storage::ops::set_amount::<StorageImpl>(#id, value);
                    }
                },
                "svm_sdk :: Address" | "Address" => quote! {
                    fn #setter_name(value: &svm_sdk::Address) {
                        #includes

                        svm_sdk::storage::ops::set_addr::<StorageImpl>(#id, value);
                    }
                },
                ty => panic!("Type `{}` is not supported", ty),
            }
        }
        Var::Array {
            id,
            name,
            elem_ty,
            length,
            ..
        } => {
            let setter_name = setter_ident(name);

            match elem_ty.as_str() {
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" => {
                    quote! {
                        fn #setter_name (index: usize, value: #elem_ty) {
                            #includes

                            svm_sdk::storage::ops::array_set32::<StorageImpl>(#id, index, #length, value as u32);
                        }
                    }
                }
                "u64" | "i64" => {
                    quote! {
                        fn #setter_name (index: usize, value: #elem_ty) {
                            #includes

                            svm_sdk::storage::ops::array_set64::<StorageImpl>(#id, index, #length, value as u64);
                        }
                    }
                }
                "bool" => {
                    quote! {
                        fn #setter_name (index: usize, value: bool) {
                            #includes

                            svm_sdk::storage::ops::array_set_bool::<StorageImpl>(#id, index, #length, value);
                        }
                    }
                }
                "svm_sdk :: Amount" | "Amount" => {
                    quote! {
                        fn #setter_name (index: usize, value: svm_sdk::Amount) {
                            #includes

                            svm_sdk::storage::ops::array_set_amount::<StorageImpl>(#id, index, #length, value);
                        }
                    }
                }
                "svm_sdk :: Address" | "Address" => {
                    quote! {
                        fn #setter_name (index: usize, value: &svm_sdk::Address) {
                            #includes

                            svm_sdk::storage::ops::array_set_addr::<StorageImpl>(#id, index, #length, value);
                        }
                    }
                }
                ty => panic!("Type `{}` is not supported", ty),
            }
        }
    }
}

fn getter_ident(var_name: &Ident) -> Ident {
    Ident::new(&format!("get_{}", var_name), Span::call_site())
}

fn setter_ident(var_name: &Ident) -> Ident {
    Ident::new(&format!("set_{}", var_name), Span::call_site())
}

fn include_storage_ast(must_mock: bool) -> TokenStream {
    if must_mock {
        quote! {
            use svm_sdk::traits::Storage;

            use svm_sdk::storage::MockStorage as StorageImpl;
        }
    } else {
        quote! {
            use svm_sdk::traits::Storage;

            use svm_sdk::storage::ExtStorage as StorageImpl;
        }
    }
}

fn next_var(var_id: VarId, length: u32) -> VarId {
    let id = var_id.0;
    VarId(id + length)
}

fn field_ident(f: &Field) -> Ident {
    f.ident.as_ref().unwrap().clone()
}

fn field_byte_count(ty: &PrimType) -> usize {
    match ty.as_str() {
        "bool" => 1,
        "Amount" => 8,
        "Address" => 20,
        "svm_sdk :: Amount" => 8,
        "svm_sdk :: Address" => 20,
        "i8" => 1,
        "u8" => 1,
        "i16" => 2,
        "u16" => 2,
        "i32" => 4,
        "u32" => 4,
        "i64" => 8,
        "u64" => 8,
        _ => unreachable!(),
    }
}
