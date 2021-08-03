use svm_types::{Address, Context, Envelope, Gas, State, TemplateAddr};

/// Information regarding a Wasm function call.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    pub func_name: &'a str,
    pub func_input: &'a [u8],
    pub target: Address,
    pub template: TemplateAddr,
    pub state: &'a State,
    pub gas_used: Gas,
    pub gas_left: Gas,
    pub within_spawn: bool,
    pub context: &'a Context,
    pub envelope: &'a Envelope,
}
