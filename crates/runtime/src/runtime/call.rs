use svm_types::{Address, Gas, State, TemplateAddr};

/// Information regarding a Wasm function call.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    pub func_name: &'a str,
    pub calldata: &'a [u8],
    pub target_addr: &'a Address,
    pub target_template: &'a TemplateAddr,
    pub state: &'a State,
    pub gas_used: Gas,
    pub gas_left: Gas,
    pub within_spawn: bool,
}
