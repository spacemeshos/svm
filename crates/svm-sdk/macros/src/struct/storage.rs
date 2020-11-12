use proc_macro2::{Ident, Span, TokenStream};

use quote::{quote, ToTokens};
use syn::{
    Error, Expr, ExprLit, Field, Fields, Lit, Path, PathArguments, Result, Type, TypeArray,
    TypePath,
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

macro_rules! ident_as_str {
    ($ident:expr) => {
        $ident.to_string().as_str()
    };
}

fn storage_vars(strukt: &Struct) -> Result<Vec<Var>> {
    let mut vars = Vec::new();
    let mut index = 0;

    let fields = strukt.raw_fields();

    ensure_named_fields(fields)?;

    for f in fields {
        let id = VarId(index);
        let var = field_var(f, id)?;
        vars.push(var);
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
            let ty = parse_array_elem_type(&array);
            let length = parse_array_length(&array);
            let name = field_ident(field);

            Var::Array {
                id,
                name,
                ty,
                length,
            }
        }
        Type::Path(path) => {
            let name = field_ident(field);
            let ty = parse_type_path(path);

            Var::Primitive { id, name, ty }
        }
        _ => todo!("Invalid Type"),
    };

    Ok(var)
}

fn parse_array_elem_type(array: &TypeArray) -> Ident {
    match *array.elem {
        Type::Path(ref path) => parse_type_path(path),
        _ => todo!("Invalid array type"),
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
                        Err(..) => todo!("Invalid array length"),
                    }
                }
                _ => todo!("Invalid array length"),
            }
        }
        _ => todo!("Invalid array length"),
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
                "Address" => ty.clone(),
        _ => todo!("Invalid Storage field type: {}", ty),
    }
}

fn segments_path_as_ident(path: &Path) -> Ident {
    match path.segments.len() {
        1 => single_segment_path_ident(path),
        _ => todo!("Invalid field type"),
    }
}

fn single_segment_path_ident(path: &Path) -> Ident {
    path_segment_as_ident(path, 0)
}

fn path_segment_as_ident(path: &Path, index: usize) -> Ident {
    debug_assert!(path.segments.len() > index);

    let segment = &path.segments[index];
    assert!(matches!(segment.arguments, PathArguments::None));

    segment.ident.clone()
}

fn field_ident(f: &Field) -> Ident {
    f.ident.as_ref().unwrap().clone()
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
        Var::Primitive { id, name, ty } => {
            let getter_name = getter_ident(name);

            match ident_as_str!(ty) {
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
                "Amount" => {
                    quote! {
                        fn #getter_name () -> svm_sdk::Amount {
                            #includes

                            svm_sdk::storage::ops::get_amount::<StorageImpl>(#id)
                        }
                    }
                }
                "Address" => {
                    quote! {
                        fn #getter_name () -> svm_sdk::Address {
                            #includes

                            svm_sdk::storage::ops::get_addr::<StorageImpl>(#id)
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Var::Array {
            id,
            name,
            ty,
            length,
        } => {
            let getter_name = getter_ident(name);

            match ident_as_str!(ty) {
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
                "Amount" => quote! {
                    fn #getter_name (index: usize) -> svm_sdk::Amount {
                        #includes

                        svm_sdk::storage::ops::array_get_amount::<StorageImpl>(#id, index, #length)
                    }
                },
                "Address" => quote! {
                    fn #getter_name (index: usize) -> svm_sdk::Address {
                        #includes

                        svm_sdk::storage::ops::array_get_addr::<StorageImpl>(#id, index, #length)
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
                "Amount" => quote! {
                    fn #setter_name (value: svm_sdk::Amount) {
                        #includes

                        svm_sdk::storage::ops::set_amount::<StorageImpl>(#id, value);
                    }
                },
                "Address" => quote! {
                    fn #setter_name(value: &svm_sdk::Address) {
                        #includes

                        svm_sdk::storage::ops::set_addr::<StorageImpl>(#id, value);
                    }
                },
                _ => unreachable!(),
            }
        }
        Var::Array {
            id,
            name,
            ty,
            length,
        } => {
            let setter_name = setter_ident(name);

            match ident_as_str!(ty) {
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
                "Amount" => {
                    quote! {
                        fn #setter_name (index: usize, value: Amount) {
                            #includes

                            svm_sdk::storage::ops::array_set_amount::<StorageImpl>(#id, index, #length, value);
                        }
                    }
                }
                "Address" => {
                    quote! {
                        fn #setter_name (index: usize, value: &svm_sdk::Address) {
                            #includes

                            svm_sdk::storage::ops::array_set_addr::<StorageImpl>(#id, index, #length, value);
                        }
                    }
                }
                _ => unreachable!(),
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
        #[cfg(test)]
        use svm_sdk::storage::MockStorage as StorageImpl;

        #[cfg(not(test))]
        use svm_sdk::storage::ExtStorage as StorageImpl;
    }
}
