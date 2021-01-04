use quote::quote;
use syn::{FnArg, PatType, ReturnType, Type, TypeTuple};

use crate::function::{func_attrs, has_ctor_attr, has_endpoint_attr, has_fundable_attr};
use crate::r#struct::has_storage_attr;
use crate::storage_vars;
use crate::{App, Function, Var};

#[derive(Debug)]
pub struct Schema {
    name: String,

    exports: Vec<Export>,

    storage: Vec<Var>,
}

#[derive(Debug)]
pub struct Export {
    pub is_ctor: bool,

    pub is_fundable: bool,

    pub api_name: String,

    pub wasm_name: String,

    pub signature: Signature,
}

#[derive(Debug)]
pub enum Param {
    Primitive {
        name: String,
        ty: String,
    },
    Array {
        name: String,
        ty: String,
        length: usize,
    },
}

#[derive(Debug)]
pub enum Output {
    Primitive { ty: String },
    Tuple { elems: Vec<String> },
}

#[derive(Debug)]
pub struct Signature {
    params: Vec<Param>,

    output: Option<Output>,
}

impl Signature {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            output: None,
        }
    }

    pub fn push_param(&mut self, param: Param) {
        self.params.push(param);
    }

    pub fn set_output(&mut self, out: Output) {
        self.output = Some(out);
    }

    pub fn params(&self) -> &[Param] {
        &self.params
    }

    pub fn output(&self) -> Option<&Output> {
        self.output.as_ref()
    }
}

impl Schema {
    pub fn new(name: String) -> Self {
        Self {
            name,
            exports: Vec::new(),
            storage: Vec::new(),
        }
    }

    pub fn add_export(&mut self, export: Export) {
        self.exports.push(export);
    }

    pub fn name(&self) -> String {
        self.name.clone()
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
    let name = app.name().to_string();
    let storage = storage_schema(app);

    let exports = app
        .functions()
        .iter()
        .filter(|func| {
            let attrs = func_attrs(func).unwrap();

            let is_endpoint = has_endpoint_attr(&attrs);
            let is_ctor = has_ctor_attr(&attrs);

            is_endpoint || is_ctor
        })
        .map(export_schema)
        .collect();

    Schema {
        name,
        storage,
        exports,
    }
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
    let attrs = func_attrs(func).unwrap();

    let is_ctor = has_ctor_attr(&attrs);
    let is_fundable = has_fundable_attr(&attrs);

    let api_name = func.raw_name().to_string();

    // TODO: future PR will uglify the name of the endpoint
    // in order to save space in the transactions.
    // The original (code) name will appear in the `schema.json` (off-chain).
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

            let param = Param::Primitive {
                name: name.to_string(),
                ty: ty.to_string(),
            };

            sig.push_param(param);
        } else {
            unreachable!()
        }
    }

    if let ReturnType::Type(.., ty) = &raw_sig.output {
        match &**ty {
            Type::Path(ty) => {
                let ty = quote! { #ty };
                let out = Output::Primitive { ty: ty.to_string() };

                sig.set_output(out);
            }
            Type::Array(ty) => {
                todo!();
            }
            Type::Tuple(tuple) => {
                let mut elems = Vec::new();

                for elem in tuple.elems.iter() {
                    let elem = quote! { #elem };

                    elems.push(elem.to_string());
                }

                let out = Output::Tuple { elems };
                sig.set_output(out);
            }
            _ => unreachable!(),
        }
    }

    sig
}
