use svm_types::{Address, Context, Envelope, Gas, State, TemplateAddr};

use crate::AccessMode;

/// Information regarding a Wasm function call.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    pub func_name: &'a str,
    pub func_input: &'a [u8],
    pub target: Address,
    pub template: TemplateAddr,
    pub state: &'a State,
    pub gas_limit: Gas,
    pub within_spawn: bool,
    pub context: &'a Context,
    pub envelope: &'a Envelope,
    pub access_mode: AccessMode,
}
