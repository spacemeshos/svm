use std::{error, fmt};

use crate::{AppAddr, TemplateAddr};

/// `exec-app` error
#[allow(missing_docs)]
#[derive(PartialEq, Clone)]
pub enum ExecAppError {
    OOG,
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
    ExecFailed {
        app_addr: AppAddr,
        template_addr: TemplateAddr,
        func_idx: u16,
        func_args: String,
        reason: String,
    },
}

impl error::Error for ExecAppError {
    fn description(&self) -> &'static str {
        match self {
            ExecAppError::OOG => "Out of Gas",
            ExecAppError::AppNotFound { .. } => "App not found",
            ExecAppError::CompilationFailed { .. } => "Compilation failed",
            ExecAppError::InstantiationFailed { .. } => "Instance Instantiation failed",
            ExecAppError::FuncNotFound { .. } => "Function not found",
            ExecAppError::ExecFailed { .. } => "Execution failed",
            ExecAppError::InvalidReturnValue { .. } => "Invalid return value",
        }
    }
}

impl fmt::Debug for ExecAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for ExecAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            ExecAppError::OOG => self.fmt_oog(),
            ExecAppError::AppNotFound { app_addr } => self.fmt_app_not_found(app_addr),
            ExecAppError::CompilationFailed {
                app_addr,
                template_addr,
                reason,
            } => self.fmt_compilation_failed(app_addr, template_addr, reason),
            ExecAppError::InstantiationFailed {
                app_addr,
                template_addr,
                reason,
            } => self.fmt_instantiation_failed(app_addr, template_addr, reason),
            ExecAppError::FuncNotFound {
                app_addr,
                template_addr,
                func_idx,
            } => self.fmt_func_not_found(app_addr, template_addr, *func_idx),
            ExecAppError::ExecFailed {
                app_addr,
                template_addr,
                func_idx,
                func_args,
                reason,
            } => self.fmt_exec_failed(app_addr, template_addr, *func_idx, func_args, reason),
            ExecAppError::InvalidReturnValue {
                app_addr,
                template_addr,
                func_idx,
                func_args,
                func_rets,
                reason,
            } => self.fmt_invalid_ret_value(
                app_addr,
                template_addr,
                *func_idx,
                func_args,
                func_rets,
                reason,
            ),
        };

        write!(f, "{}", msg)
    }
}

impl ExecAppError {
    #[inline]
    fn fmt_oog(&self) -> String {
        "Out of Gas".to_string()
    }

    #[inline]
    fn fmt_app_not_found(&self, app_addr: &AppAddr) -> String {
        format!("App `{:?}` not found", app_addr.inner())
    }

    #[inline]
    fn fmt_compilation_failed(
        &self,
        app_addr: &AppAddr,
        template_addr: &TemplateAddr,
        reason: &str,
    ) -> String {
        format!(
            "Compilation failed for app `{:?}` template `{:?}` ({})",
            app_addr.inner(),
            template_addr.inner(),
            reason
        )
    }

    #[inline]
    fn fmt_instantiation_failed(
        &self,
        app_addr: &AppAddr,
        template_addr: &TemplateAddr,
        reason: &str,
    ) -> String {
        format!(
            "Instance Instantiation failed for app `{:?}` template `{:?}`\rReason: {}",
            app_addr.inner(),
            template_addr.inner(),
            reason
        )
    }

    #[inline]
    fn fmt_func_not_found(
        &self,
        app_addr: &AppAddr,
        template_addr: &TemplateAddr,
        func_idx: u16,
    ) -> String {
        format!(
            "Function `{}` not found (app = `{:?}`, template=`{:?}`)",
            func_idx,
            app_addr.inner(),
            template_addr.inner()
        )
    }

    #[inline]
    fn fmt_exec_failed(
        &self,
        app_addr: &AppAddr,
        template_addr: &TemplateAddr,
        func_idx: u16,
        func_args: &str,
        reason: &str,
    ) -> String {
        format!(
            "Execution failed for function `{}` with input `{}` (app=`{:?}`, template=`{:?}`)\nReason: {}",
            func_idx, func_args, app_addr.inner(), template_addr.inner(), reason
        )
    }

    #[inline]
    fn fmt_invalid_ret_value(
        &self,
        app_addr: &AppAddr,
        template_addr: &TemplateAddr,
        func_idx: u16,
        func_args: &str,
        func_rets: &str,
        reason: &str,
    ) -> String {
        format!(
            "Function `{}` returned invalid values `{}` for input `{}` (app=`{:?}`, template=`{:?}`)\nReason: {}",
            func_idx, func_rets, func_args, app_addr.inner(), template_addr.inner(), reason)
    }
}
