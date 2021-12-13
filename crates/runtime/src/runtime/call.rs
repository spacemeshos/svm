use svm_types::{Address, Context, Envelope, State, TemplateAddr};

use crate::AccessMode;

use super::gas_tank::GasTank;

/// Information regarding a Wasm function call.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    pub func_name: &'a str,
    pub func_input: &'a [u8],
    pub target: Address,
    pub template_addr: TemplateAddr,
    pub state: &'a State,
    pub gas_left: GasTank,
    pub within_spawn: bool,
    pub context: &'a Context,
    pub envelope: &'a Envelope,
    pub access_mode: AccessMode,
}
