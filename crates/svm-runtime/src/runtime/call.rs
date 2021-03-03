use svm_types::gas::MaybeGas;
use svm_types::{AppAddr, State, TemplateAddr};

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    pub func_name: &'a str,

    pub calldata: &'a [u8],

    pub app_addr: &'a AppAddr,

    pub template_addr: &'a TemplateAddr,

    pub state: &'a State,

    pub gas_used: MaybeGas,

    pub gas_left: MaybeGas,

    pub within_spawn: bool,
}

#[doc(hidden)]
impl<'a> Call<'a> {
    #[doc(hidden)]
    pub fn func_name(&self) -> &str {
        &self.func_name
    }

    #[doc(hidden)]
    pub fn calldata(&self) -> &[u8] {
        &self.calldata
    }

    #[doc(hidden)]
    pub fn state(&self) -> &State {
        &self.state
    }

    #[doc(hidden)]
    #[allow(unused)]
    pub fn gas_used(&self) -> MaybeGas {
        self.gas_used
    }

    #[doc(hidden)]
    pub fn gas_left(&self) -> MaybeGas {
        self.gas_left
    }

    #[doc(hidden)]
    pub fn template_addr(&self) -> &TemplateAddr {
        &self.template_addr
    }

    #[doc(hidden)]
    pub fn app_addr(&self) -> &AppAddr {
        &self.app_addr
    }

    #[doc(hidden)]
    pub fn within_spawn(&self) -> bool {
        self.within_spawn
    }
}
