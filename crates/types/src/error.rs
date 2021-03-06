use crate::{AppAddr, TemplateAddr};

#[doc(hidden)]
#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    OOG,
    TemplateNotFound(TemplateAddr),
    AppNotFound(AppAddr),
    CompilationFailed {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        msg: String,
    },
    InstantiationFailed {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        msg: String,
    },
    FuncNotFound {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        func: String,
    },
    FuncFailed {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncNotAllowed {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncInvalidSignature {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        func: String,
    },
}
