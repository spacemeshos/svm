use proc_macro2::{Ident, Span, TokenStream};

use quote::{quote, ToTokens};
use syn::{
    Error, Expr, ExprLit, Field, Fields, ItemStruct, Lit, Path, PathArguments, Result, Type,
    TypeArray, TypePath,
};

use super::{attr, Var, VarId};
use attr::{has_storage_attr, StructAttr};

use crate::Struct;

pub fn expand(strukt: &Struct, attrs: &[StructAttr]) -> Result<TokenStream> {
    debug_assert!(has_storage_attr(attrs));

    let vars = storage_vars(strukt)?;

    let name = strukt.raw_name();
    let getters = getters_ast(&vars);
    let setters = setters_ast(&vars);

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

    let fields = strukt.raw_fields();

    ensure_named_fields(fields)?;

    for f in fields {
        let var = field_var(f, id)?;

        vars.push(var);

        id = next_var(id);
    }

    Ok(vars)
}

fn field_var(field: &Field, id: VarId) -> Result<Var> {
    let span = Span::call_site();

    if !field.attrs.is_empty() {
        let msg = "`#[storage]` fields should have no attributes.";

        return Err(Error::new(span, msg));
    }

    let var = match &field.ty {
        Type::Array(array) => {
            let (ty, ty_str) = parse_array_element_type(&array)?;
            let length = parse_array_length(&array)?;
            let name = field_ident(field);

            Var::Array {
                id,
                name,
                ty,
                ty_str,
                length,
            }
        }
        Type::Path(path) => {
            let name = field_ident(field);
            let (ty, ty_str) = parse_type_path(path)?;

            Var::Primitive {
                id,
                name,
                ty,
                ty_str,
            }
        }
        _ => {
            return Err(Error::new(
                span,
                "`#[storage]` supports only path (for example: `svm_sdk::Amount`) and Array types.",
            ));
        }
    };

    Ok(var)
}

fn parse_array_element_type(array: &TypeArray) -> Result<(Type, String)> {
    match *array.elem {
        Type::Path(ref path) => parse_type_path(path),
        _ => {
            let span = Span::call_site();

            Err(Error::new(span, "`#[storage]` Array elements must be of type path (for example: `svm_sdk::Amount`)."))
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

fn parse_type_path(path: &TypePath) -> Result<(Type, String)> {
    let ty_str = path_as_str(&path);

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
            let ty = Type::Path(path.clone());

            Ok((ty, ty_str))
        }
        _ => {
            let span = Span::call_site();
            let msg = format!("Invalid `#[storage]` field type: {}", ty_str);

            Err(Error::new(span, msg))
        }
    }
}

fn path_as_str(path: &TypePath) -> String {
    let path = &path.path;
    let path = quote! { #path };

    path.to_string()
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

fn getters_ast(vars: &[Var]) -> TokenStream {
    let mut getters = Vec::new();

    for var in vars.iter() {
        let ast = getter_ast(var);

        getters.push(ast);
    }

    quote! {
        #(#getters)*
    }
}

fn setters_ast(vars: &[Var]) -> TokenStream {
    let mut setters = Vec::new();

    for var in vars.iter() {
        let ast = setter_ast(var);

        setters.push(ast);
    }

    quote! {
        #(#setters)*
    }
}

fn getter_ast(var: &Var) -> TokenStream {
    let includes = include_storage_ast();

    match var {
        Var::Primitive {
            id,
            name,
            ty,
            ty_str,
        } => {
            let getter_name = getter_ident(name);

            match ty_str.as_str() {
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
                ty => unreachable!(format!("Type `{}` is not supported", ty)),
            }
        }
        Var::Array {
            id,
            name,
            ty,
            ty_str,
            length,
        } => {
            let getter_name = getter_ident(name);

            match ty_str.as_str() {
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" => {
                    quote! {
                        fn #getter_name (index: usize) -> #ty {
                            #includes

                            let value = svm_sdk::storage::ops::array_get32::<StorageImpl>(#id, index, #length);
                            value as #ty
                        }
                    }
                }
                "u64" | "i64" => {
                    quote! {
                        fn #getter_name (index: usize) -> #ty {
                            #includes

                            let value = svm_sdk::storage::ops::array_get64::<StorageImpl>(#id, index, #length);
                            value as #ty
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
                ty => unreachable!(format!("Type `{}` is not supported", ty)),
            }
        }
    }
}

fn setter_ast(var: &Var) -> TokenStream {
    let includes = include_storage_ast();

    match var {
        Var::Primitive {
            id,
            name,
            ty,
            ty_str,
        } => {
            let setter_name = setter_ident(name);

            match ty_str.as_str() {
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
                ty => unreachable!(format!("Type `{}` is not supported", ty)),
            }
        }
        Var::Array {
            id,
            name,
            ty,
            ty_str,
            length,
        } => {
            let setter_name = setter_ident(name);

            match ty_str.as_str() {
                "i8" | "u8" | "i16" | "u16" | "i32" | "u32" => {
                    quote! {
                        fn #setter_name (index: usize, value: #ty) {
                            #includes

                            svm_sdk::storage::ops::array_set32::<StorageImpl>(#id, index, #length, value as u32);
                        }
                    }
                }
                "u64" | "i64" => {
                    quote! {
                        fn #setter_name (index: usize, value: #ty) {
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
                ty => unreachable!(format!("Type `{}` is not supported", ty)),
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

fn include_storage_ast() -> TokenStream {
    quote! {
        use svm_sdk::traits::Storage;

        #[cfg(feature = "mock")]
        use svm_sdk::storage::MockStorage as StorageImpl;

        #[cfg(feature = "ffi")]
        use svm_sdk::storage::ExtStorage as StorageImpl;
    }
}

fn next_var(var_id: VarId) -> VarId {
    let id = var_id.0;
    VarId(id + 1)
}

fn field_ident(f: &Field) -> Ident {
    f.ident.as_ref().unwrap().clone()
}
