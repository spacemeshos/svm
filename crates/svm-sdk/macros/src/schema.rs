use quote::quote;
use syn::{FnArg, PatType};

use crate::r#struct::has_storage_attr;
use crate::storage_vars;
use crate::{App, Function, Var};

pub struct AppSchema {
    exports: Vec<Export>,

    storage: Vec<Var>,
}

pub struct Export {
    is_ctor: bool,

    is_fundable: bool,

    api_name: String,

    wasm_name: String,

    sig: Signature,
}

pub struct Signature {
    params: Vec<(String, String)>,

    returns: Vec<String>,
}

impl Signature {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            returns: Vec::new(),
        }
    }

    pub fn add_param(&mut self, name: String, ty: String) {
        self.params.push((name, ty));
    }

    pub fn add_return(&mut self, ty: String) {
        self.returns.push(ty);
    }

    pub fn params(&self) -> &[(String, String)] {
        &self.params
    }

    pub fn returns(&self) -> &[String] {
        &self.returns
    }
}

impl AppSchema {
    pub fn new() -> Self {
        Self {
            exports: Vec::new(),
            storage: Vec::new(),
        }
    }

    pub fn add_export(&mut self, export: Export) {
        self.exports.push(export);
    }

    pub fn endpoints(&self) -> Vec<&Export> {
        self.exports
            .iter()
            .filter(|exp| exp.is_ctor == false)
            .collect()
    }

    pub fn ctors(&self) -> Vec<&Export> {
        self.exports.iter().filter(|exp| exp.is_ctor).collect()
    }

    pub fn exports(&self) -> &[Export] {
        &self.exports
    }

    pub fn storage(&self) -> &[Var] {
        &self.storage
    }
}

pub fn storage_schema(app: &App) -> Vec<Var> {
    let storage = app
        .structs()
        .iter()
        .find(|s| {
            let attrs = s.attrs().as_ref().unwrap();

            has_storage_attr(attrs)
        })
        .unwrap();

    storage_vars(&storage).unwrap()
}

pub fn function_sig(func: &Function) -> Signature {
    let raw_sig = func.raw_sig();

    let mut sig = Signature::new();

    for input in &raw_sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = input {
            let name = quote! { #pat };
            let ty = quote! { #ty };

            sig.add_param(name.to_string(), ty.to_string());
        } else {
            unreachable!()
        }
    }

    sig
}
