use proc_macro2::{Span, TokenStream};
use syn::Result;

use super::attr::FuncAttr;
use crate::{App, Function};

pub(crate) fn expand(func: &Function, attrs: &[FuncAttr], app: &App) -> Result<TokenStream> {
    super::endpoint::expand(func, attrs, app)
}
