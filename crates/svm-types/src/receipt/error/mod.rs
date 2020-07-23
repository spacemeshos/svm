mod deploy_template;
mod exec_app;
mod spawn_app;

use crate::{AppAddr, TemplateAddr};

#[derive(Debug, PartialEq, Clone)]
pub enum ReceiptError {
    OOG,
    TemplateNotFound(TemplateAddr),
    AppNotFound(TemplateAddr, AppAddr),
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
        func_idx: u16,
    },
    FuncFailed {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        func_idx: u16,
        msg: String,
    },
}
