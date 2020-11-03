extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Expr, ExprLit, Field, Fields, FieldsNamed,
    Lit, Path, PathArguments, Type, TypeArray, TypePath,
};

pub fn parse_storage(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
        length: u32,
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
            Var::Primitive { .. } => {
                index += 1;
            }
            Var::Array { length, .. } => {
                index += length;
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
                "Address" => ty.clone(),
        _ => panic!("Invalid Storage field type: {}", ty),
    }
}

fn segments_path_as_ident(path: &Path) -> Ident {
    match path.segments.len() {
        1 => single_segment_path_ident(path),
        _ => panic!("Invalid field type"),
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

fn include_storage_ast() -> TokenStream {
    quote! {
        #[cfg(test)]
        use svm_sdk::storage::MockStorage as StorageImpl;

        #[cfg(not(test))]
        use svm_sdk::storage::ExtStorage as StorageImpl;
    }
}
