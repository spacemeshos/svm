use svm_types::{AccountAddr, Gas, State, TemplateAddr};

/// Information regarding a WASM function call.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    pub func_name: &'a str,
    pub calldata: &'a [u8],
    pub account_addr: &'a AccountAddr,
    pub template_addr: &'a TemplateAddr,
    pub state: &'a State,
    pub gas_used: Gas,
    pub gas_left: Gas,
    pub within_spawn: bool,
}
