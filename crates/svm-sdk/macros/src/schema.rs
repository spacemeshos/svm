use quote::quote;
use syn::{FnArg, PatType};

use crate::r#struct::has_storage_attr;
use crate::storage_vars;
use crate::{App, Function, Var};

#[derive(Debug)]
pub struct Schema {
    exports: Vec<Export>,

    storage: Vec<Var>,
}

#[derive(Debug)]
pub struct Export {
    is_ctor: bool,

    is_fundable: bool,

    api_name: String,

    wasm_name: String,

    signature: Signature,
}

#[derive(Debug)]
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

impl Schema {
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

pub fn app_schema(app: &App) -> Schema {
    let storage = storage_schema(app);

    let exports = app.functions().iter().map(export_schema).collect();

    Schema { storage, exports }
}

fn storage_schema(app: &App) -> Vec<Var> {
    let storage = app.structs().iter().find(|s| {
        let attrs = s.attrs().as_ref().unwrap();

        has_storage_attr(attrs)
    });

    if let Some(storage) = storage {
        storage_vars(&storage).unwrap()
    } else {
        Vec::new()
    }
}

fn export_schema(func: &Function) -> Export {
    let is_ctor = false;
    let is_fundable = false;
    let api_name = func.raw_name().to_string();
    let wasm_name = func.raw_name().to_string();
    let signature = function_sig(func);

    Export {
        is_ctor,
        is_fundable,
        api_name,
        wasm_name,
        signature,
    }
}

fn function_sig(func: &Function) -> Signature {
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
