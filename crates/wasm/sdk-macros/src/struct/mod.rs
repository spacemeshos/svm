use proc_macro2::{Ident, TokenStream};
use syn::{Fields, ItemStruct, Result};

mod attr;
mod storage;
mod var;

pub use attr::{has_storage_attr, StructAttr, StructAttrKind};
pub use storage::storage_vars;
pub use var::{Var, VarId};

pub struct Struct {
    raw_struct: ItemStruct,
    attrs: Result<Vec<StructAttr>>,
}

impl Struct {
    pub fn new(raw_struct: ItemStruct) -> Self {
        let attrs = attr::struct_attrs(&raw_struct.attrs);

        Self { raw_struct, attrs }
    }

    pub fn raw_name(&self) -> Ident {
        self.raw_struct.ident.clone()
    }

    pub fn raw_fields(&self) -> &Fields {
        &self.raw_struct.fields
    }

    pub fn attrs(&self) -> &Result<Vec<StructAttr>> {
        &self.attrs
    }
}

pub fn expand(strukt: &Struct, must_mock: bool) -> Result<TokenStream> {
    match strukt.attrs() {
        Ok(attrs) => {
            if has_storage_attr(attrs) {
                storage::expand(strukt, attrs, must_mock)
            } else {
                todo!()
            }
        }
        Err(err) => Err(err.clone()),
    }
}
