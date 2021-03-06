use svm_types::{AppAddr, Gas, State, TemplateAddr};

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct Call<'a> {
    pub func_name: &'a str,
    pub calldata: &'a [u8],
    pub app_addr: &'a AppAddr,
    pub template_addr: &'a TemplateAddr,
    pub state: &'a State,
    pub gas_used: Gas,
    pub gas_left: Gas,
    pub within_spawn: bool,
}
