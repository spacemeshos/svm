use crate::{AccountAddr, TemplateAddr};

#[doc(hidden)]
#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    OOG,
    TemplateNotFound(TemplateAddr),
    AppNotFound(AccountAddr),
    CompilationFailed {
        app_addr: AccountAddr,
        template_addr: TemplateAddr,
        msg: String,
    },
    InstantiationFailed {
        app_addr: AccountAddr,
        template_addr: TemplateAddr,
        msg: String,
    },
    FuncNotFound {
        app_addr: AccountAddr,
        template_addr: TemplateAddr,
        func: String,
    },
    FuncFailed {
        app_addr: AccountAddr,
        template_addr: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncNotAllowed {
        app_addr: AccountAddr,
        template_addr: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncInvalidSignature {
        app_addr: AccountAddr,
        template_addr: TemplateAddr,
        func: String,
    },
}
