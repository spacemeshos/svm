use proc_macro2::{Span, TokenStream};
use syn::Result;

use super::attr::FuncAttr;
use crate::{schema::Schema, Function};

pub(crate) fn expand(func: &Function, attrs: &[FuncAttr], schema: &Schema) -> Result<TokenStream> {
    super::endpoint::expand(func, attrs, schema)
}
