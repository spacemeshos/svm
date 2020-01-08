use std::error;
use std::fmt;

use svm_app::error::ParseError;
use svm_common::Address;

/// `exec-app` error
#[allow(missing_docs)]
#[derive(PartialEq, Clone)]
pub enum ExecAppError {
    ParseFailed(ParseError),
    AppNotFound {
        app_addr: Address,
    },
    CompilationFailed {
        app_addr: Address,
        template_addr: Address,
        reason: String,
    },
    InstantiationFailed {
        app_addr: Address,
        template_addr: Address,
        reason: String,
    },
    FuncNotFound {
        app_addr: Address,
        template_addr: Address,
        func_name: String,
    },
    InvalidReturnValue {
        app_addr: Address,
        template_addr: Address,
        func_name: String,
        func_args: String,
        func_rets: String,
        reason: String,
    },
    ExecFailed {
        app_addr: Address,
        template_addr: Address,
        func_name: String,
        func_args: String,
        reason: String,
    },
}

impl error::Error for ExecAppError {
    fn description(&self) -> &'static str {
        match self {
            ExecAppError::ParseFailed(..) => "Parse failed",
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
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for ExecAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            ExecAppError::ParseFailed(e) => self.fmt_parse_failed(e),
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
                func_name,
            } => self.fmt_func_not_found(app_addr, template_addr, func_name),
            ExecAppError::ExecFailed {
                app_addr,
                template_addr,
                func_name,
                func_args,
                reason,
            } => self.fmt_exec_failed(app_addr, template_addr, func_name, func_args, reason),
            ExecAppError::InvalidReturnValue {
                app_addr,
                template_addr,
                func_name,
                func_args,
                func_rets,
                reason,
            } => self.fmt_invalid_ret_value(
                app_addr,
                template_addr,
                func_name,
                func_args,
                func_rets,
                reason,
            ),
        };

        write!(f, "{}", msg)
    }
}

impl ExecAppError {
    fn fmt_parse_failed(&self, e: &ParseError) -> String {
        e.to_string()
    }

    fn fmt_app_not_found(&self, app_addr: &Address) -> String {
        format!("App `{:?}` not found", app_addr)
    }

    fn fmt_compilation_failed(
        &self,
        app_addr: &Address,
        template_addr: &Address,
        reason: &str,
    ) -> String {
        format!(
            "Compilation failed for app `{:?}` template `{:?}` ({})",
            app_addr, template_addr, reason
        )
    }

    fn fmt_instantiation_failed(
        &self,
        app_addr: &Address,
        template_addr: &Address,
        reason: &str,
    ) -> String {
        format!(
            "Instance Instantiation failed for app `{:?}` template `{:?}`\rReason: {}",
            app_addr, template_addr, reason
        )
    }

    fn fmt_func_not_found(
        &self,
        app_addr: &Address,
        template_addr: &Address,
        func_name: &str,
    ) -> String {
        format!(
            "Function `{}` not found (app = `{:?}`, template=`{:?}`)",
            func_name, app_addr, template_addr
        )
    }

    fn fmt_exec_failed(
        &self,
        app_addr: &Address,
        template_addr: &Address,
        func_name: &str,
        func_args: &str,
        reason: &str,
    ) -> String {
        format!(
            "Execution failed for function `{}` with input `{}` (app=`{:?}`, template=`{:?}`)\nReason: {}",
            func_name, func_args, app_addr, template_addr, reason
        )
    }

    fn fmt_invalid_ret_value(
        &self,
        app_addr: &Address,
        template_addr: &Address,
        func_name: &str,
        func_args: &str,
        func_rets: &str,
        reason: &str,
    ) -> String {
        format!(
            "Function `{}` returned invalid values `{}` for input `{}` (app=`{:?}`, template=`{:?}`)\nReason: {}",
            func_name, func_rets, func_args, app_addr, template_addr, reason)
    }
}
