use crate::{Address, TemplateAddr};

#[doc(hidden)]
#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    OOG,
    TemplateNotFound(TemplateAddr),
    AccountNotFound(Address),
    CompilationFailed {
        account_addr: Address,
        template_addr: TemplateAddr,
        msg: String,
    },
    InstantiationFailed {
        account_addr: Address,
        template_addr: TemplateAddr,
        msg: String,
    },
    FuncNotFound {
        account_addr: Address,
        template_addr: TemplateAddr,
        func: String,
    },
    FuncFailed {
        account_addr: Address,
        template_addr: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncNotAllowed {
        account_addr: Address,
        template_addr: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncInvalidSignature {
        account_addr: Address,
        template_addr: TemplateAddr,
        func: String,
    },
}
