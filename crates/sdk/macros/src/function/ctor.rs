use proc_macro2::TokenStream;
use syn::Result;

use super::attr::FuncAttr;
use crate::{Function, Template};

pub(crate) fn expand(
    func: &Function,
    attrs: &[FuncAttr],
    template: &Template,
) -> Result<TokenStream> {
    super::endpoint::expand(func, attrs, template)
}
