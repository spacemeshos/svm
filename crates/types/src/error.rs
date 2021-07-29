use crate::{Address, TemplateAddr};

#[doc(hidden)]
#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    OOG,
    TemplateNotFound(TemplateAddr),
    AccountNotFound(Address),
    CompilationFailed {
        target: Address,
        template: TemplateAddr,
        msg: String,
    },
    InstantiationFailed {
        target: Address,
        template: TemplateAddr,
        msg: String,
    },
    FuncNotFound {
        target: Address,
        template: TemplateAddr,
        func: String,
    },
    FuncFailed {
        target: Address,
        template: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncNotAllowed {
        target: Address,
        template: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncInvalidSignature {
        target: Address,
        template: TemplateAddr,
        func: String,
    },
}
