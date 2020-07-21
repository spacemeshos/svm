mod deploy_template;
mod exec_app;
mod spawn_app;

use crate::{AppAddr, TemplateAddr};

#[derive(Debug, PartialEq, Clone)]
pub enum ReceiptError {
    OOG,
    TemplateNotFound(TemplateAddr),
    FuncFailed {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        func_idx: u16,
        func_args: String,
        reason: String,
    },
    AppNotFound {
        app_addr: AppAddr,
    },
    CompilationFailed {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        reason: String,
    },
    InstantiationFailed {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        reason: String,
    },
    FuncNotFound {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        func_idx: u16,
    },
    InvalidReturnValue {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        func_idx: u16,
        func_args: String,
        func_rets: String,
        reason: String,
    },
}
