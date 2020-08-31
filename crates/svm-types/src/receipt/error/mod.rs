mod deploy_template;
mod exec_app;
mod spawn_app;

use crate::{AppAddr, TemplateAddr};

#[derive(Debug, PartialEq, Clone)]
pub enum ReceiptError {
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
}
