use std::hint::unreachable_unchecked;

use svm_types::gas::MaybeGas;
use svm_types::{AppAddr, State, TemplateAddr};

#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    pub addr: CallAddr<'a>,

    pub func_name: &'a str,

    pub calldata: &'a [u8],

    pub state: &'a State,

    pub gas_used: MaybeGas,

    pub gas_left: MaybeGas,

    pub within_spawn: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallAddr<'a>(Option<&'a TemplateAddr>, Option<&'a AppAddr>);

impl<'a> CallAddr<'a> {
    pub fn new(template: &'a TemplateAddr, app: &'a AppAddr) -> Self {
        Self(Some(template), Some(app))
    }

    pub fn with_template(addr: &'a TemplateAddr) -> Self {
        Self(Some(addr), None)
    }

    pub fn with_app(addr: &'a AppAddr) -> Self {
        Self(None, Some(addr))
    }

    pub fn template_addr(&self) -> &TemplateAddr {
        self.0.as_ref().unwrap()
    }

    pub fn app_addr(&self) -> &AppAddr {
        self.1.as_ref().unwrap()
    }
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
        self.addr.template_addr()
    }

    pub fn app_addr(&self) -> &AppAddr {
        self.addr.app_addr()
    }

    pub fn within_spawn(&self) -> bool {
        self.within_spawn
    }
}
