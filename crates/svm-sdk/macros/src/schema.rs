use std::collections::{hash_map::Values, HashMap};

use proc_macro2::Span;
use quote::quote;
use syn::{Error, FnArg, PatType, Result, ReturnType, TypeTuple};

use crate::function::{
    find_attr, func_attrs, has_ctor_attr, has_default_fundable_hook_attr, has_endpoint_attr,
    has_fundable_attr,
};
use crate::r#struct::has_storage_attr;
use crate::storage_vars;
use crate::{App, FuncAttr, FuncAttrKind, Function, Type, Var};

pub struct Schema {
    name: String,

    exports: HashMap<String, Export>,

    storage: Vec<Var>,
}

pub struct Export {
    pub is_ctor: bool,

    pub is_fundable: bool,

    pub api_name: String,

    pub export_name: String,

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

impl Schema {
    pub fn new(name: String) -> Self {
        Self {
            name,
            exports: HashMap::new(),
            storage: Vec::new(),
        }
    }

    pub fn add_export(&mut self, export: Export) {
        let name = export.api_name.clone();

        self.exports.insert(name, export);
    }

    pub fn get_export(&self, name: &str) -> &Export {
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

    pub fn storage(&self) -> &[Var] {
        &self.storage
    }
}

pub fn app_schema(app: &App) -> Result<Schema> {
    validate_app(app)?;

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
        .map(|export| (export.api_name.clone(), export))
        .collect();

    let schema = Schema {
        name,
        storage,
        exports,
    };

    Ok(schema)
}

fn validate_app(app: &App) -> Result<()> {
    let span = Span::call_site();
    let mut seen_default_fundable_hook = false;

    for func in app.functions().iter() {
        let attrs = func_attrs(func)?;

        if has_default_fundable_hook_attr(&attrs) {
            if seen_default_fundable_hook {
                return Err(Error::new(
                    span,
                    "There can be only a single default `fundable hook`",
                ));
            }

            seen_default_fundable_hook = true;
        }
    }

    if seen_default_fundable_hook == false {
        return Err(Error::new(span, "A default `fundable hook` is missing"));
    }

    Ok(())
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
        api_name,
        export_name,
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
