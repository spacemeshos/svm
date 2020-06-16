use svm_types::AppTransaction;

use serde_json::Value;

pub fn exec_app(json: &Value) -> AppTransaction {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn exec_app_missing_version() {
        todo!()
    }

    #[test]
    #[ignore]
    fn exec_app_missing_app_addr() {
        todo!()
    }

    #[test]
    #[ignore]
    fn exec_app_missing_func_index() {
        todo!()
    }

    #[test]
    #[ignore]
    fn exec_app_missing_func_buf() {
        todo!()
    }

    #[test]
    #[ignore]
    fn exec_app_missing_func_args() {
        todo!()
    }
}
