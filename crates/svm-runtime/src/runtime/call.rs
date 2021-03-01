use std::hint::unreachable_unchecked;

use svm_types::{gas::MaybeGas, receipt::TemplateReceipt};
use svm_types::{AppAddr, State, TemplateAddr};

#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    pub app_addr: &'a AppAddr,

    pub template_addr: &'a TemplateAddr,

    pub func_name: &'a str,

    pub calldata: &'a [u8],

    pub state: &'a State,

    pub gas_used: MaybeGas,

    pub gas_left: MaybeGas,

    pub within_spawn: bool,
}

#[derive(Debug, PartialEq, Copy, Clone, Hash)]
pub enum CallKind {
    Ctor,

    Verify,

    Alloc,

    Endpoint,
}

impl<'a> Call<'a> {
    pub fn func_name(&self) -> &str {
        &self.func_name
    }

    pub fn calldata(&self) -> &[u8] {
        &self.calldata
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn gas_used(&self) -> MaybeGas {
        self.gas_used
    }

    pub fn gas_left(&self) -> MaybeGas {
        self.gas_left
    }

    pub fn template_addr(&self) -> &TemplateAddr {
        &self.template_addr
    }

    pub fn app_addr(&self) -> &AppAddr {
        &self.app_addr
    }

    pub fn within_spawn(&self) -> bool {
        self.within_spawn
    }
}
