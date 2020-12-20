use proc_macro2::{Span, TokenStream};
use syn::Result;

use super::attr::FuncAttr;
use crate::Function;

pub fn expand(func: &Function, attrs: &[FuncAttr]) -> Result<TokenStream> {
    super::endpoint::expand(func, attrs)
}
