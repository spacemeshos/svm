use std::collections::hash_map::Values;
use std::collections::HashMap;

use quote::quote;
use syn::{FnArg, PatType, Result, ReturnType};

use crate::function::{find_attr, func_attrs, has_ctor_attr, has_endpoint_attr, has_fundable_attr};
use crate::r#struct::has_storage_attr;
use crate::storage_vars;
use crate::{FuncAttr, FuncAttrKind, Function, Template, Type, Var};

pub struct TemplateMeta {
    name: String,
    schema: Vec<Var>,
    exports: HashMap<String, Export>,
}

pub struct Export {
    pub is_ctor: bool,
    pub is_fundable: bool,
    pub name: String,
    pub wasm_name: String,
    pub signature: Signature,
    pub doc: String,
}

pub struct Signature {
    params: Vec<(String, Type)>,
    output: Option<Type>,
}

impl Signature {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            output: None,
        }
    }

    pub fn push_param(&mut self, param: (String, Type)) {
        self.params.push(param);
    }

    pub fn set_output(&mut self, out: Type) {
        self.output = Some(out);
    }

    pub fn params(&self) -> &[(String, Type)] {
        &self.params
    }

    pub fn output(&self) -> Option<&Type> {
        self.output.as_ref()
    }
}

impl TemplateMeta {
    pub fn new(name: String) -> Self {
        Self {
            name,
            exports: HashMap::new(),
            schema: Vec::new(),
        }
    }

    pub fn add_export(&mut self, export: Export) {
        let name = export.name.clone();
        self.exports.insert(name, export);
    }

    pub fn export(&self, name: &str) -> &Export {
        self.exports.get(name).as_ref().unwrap()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn exports(&self) -> Values<String, Export> {
        self.exports.values().into_iter()
    }

    pub fn endpoints(&self) -> Vec<&Export> {
        self.exports().filter(|exp| exp.is_ctor == false).collect()
    }

    pub fn ctors(&self) -> Vec<&Export> {
        self.exports().filter(|exp| exp.is_ctor).collect()
    }

    pub fn schema(&self) -> &[Var] {
        &self.schema
    }
}

pub fn template_meta(template: &Template) -> Result<TemplateMeta> {
    let name = template.name().to_string();
    let schema = template_schema(template);

    let exports = template
        .functions()
        .iter()
        .filter(|func| {
            let attrs = func_attrs(func).unwrap();

            let is_endpoint = has_endpoint_attr(&attrs);
            let is_ctor = has_ctor_attr(&attrs);

            is_endpoint || is_ctor
        })
        .map(export_schema)
        .map(|export| (export.name.clone(), export))
        .collect();

    let schema = TemplateMeta {
        name,
        schema,
        exports,
    };

    Ok(schema)
}

fn template_schema(template: &Template) -> Vec<Var> {
    let storage = template.structs().iter().find(|s| {
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
    let export_name = func.export_name();

    let attr = if is_ctor {
        find_attr(&attrs, FuncAttrKind::Ctor)
    } else {
        find_attr(&attrs, FuncAttrKind::Endpoint)
    };

    let doc = match attr.unwrap() {
        FuncAttr::Ctor(doc) => doc.to_string(),
        FuncAttr::Endpoint(doc) => doc.to_string(),
        _ => unreachable!(),
    };

    let signature = function_sig(func);

    Export {
        is_ctor,
        is_fundable,
        name: api_name,
        wasm_name: export_name,
        signature,
        doc,
    }
}

fn function_sig(func: &Function) -> Signature {
    let raw_sig = func.raw_sig();

    let mut sig = Signature::new();

    for input in &raw_sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = input {
            let ty = Type::new(ty).unwrap();
            let name = quote! { #pat };

            sig.push_param((name.to_string(), ty));
        } else {
            unreachable!()
        }
    }

    if let ReturnType::Type(.., ty) = &raw_sig.output {
        let ty = Type::new(&ty).unwrap();

        sig.set_output(ty);
    }

    sig
}
