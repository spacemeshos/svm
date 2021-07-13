use crate::{AccountAddr, TemplateAddr};

#[doc(hidden)]
#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    OOG,
    TemplateNotFound(TemplateAddr),
    AccountNotFound(AccountAddr),
    CompilationFailed {
        account_addr: AccountAddr,
        template_addr: TemplateAddr,
        msg: String,
    },
    InstantiationFailed {
        account_addr: AccountAddr,
        template_addr: TemplateAddr,
        msg: String,
    },
    FuncNotFound {
        account_addr: AccountAddr,
        template_addr: TemplateAddr,
        func: String,
    },
    FuncFailed {
        account_addr: AccountAddr,
        template_addr: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncNotAllowed {
        account_addr: AccountAddr,
        template_addr: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncInvalidSignature {
        account_addr: AccountAddr,
        template_addr: TemplateAddr,
        func: String,
    },
}
