use syn::{AttrStyle, Attribute, Result};

#[derive(Debug)]
pub enum FuncAttribute {
    Endpoint,

    BeforeFund,

    Fundable(String),
}
