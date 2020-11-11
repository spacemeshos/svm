use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Expr, ExprLit, Field, Fields, FieldsNamed, Generics,
    ItemStruct, Lit, Path, PathArguments, Type, TypeArray, TypePath,
};

mod storage;

pub struct Struct {
    raw_struct: ItemStruct,
}

impl Struct {
    pub fn new(raw_struct: ItemStruct) -> Self {
        Self { raw_struct }
    }

    pub fn raw_name(&self) -> Ident {
        self.raw_struct.ident.clone()
    }

    pub fn raw_attrs(&self) -> Vec<Attribute> {
        self.raw_struct.attrs.clone()
    }

    pub fn raw_generics(&self) -> &Generics {
        &self.raw_struct.generics
    }

    pub fn raw_fields(&self) -> &Fields {
        &self.raw_struct.fields
    }

    pub fn stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();

        self.raw_struct.to_tokens(&mut tokens);

        tokens
    }
}

pub fn expand(strukt: &Struct) -> proc_macro::TokenStream {
    todo!()
}
