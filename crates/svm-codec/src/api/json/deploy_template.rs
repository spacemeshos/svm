use svm_types::AppTemplate;

use serde_json::Value;

pub fn deploy_template(json: &Value) -> Result<AppTemplate, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn json_deploy_template_missing_version() {
        todo!()
    }

    #[test]
    #[ignore]
    fn json_deploy_template_unsupported_version() {
        todo!()
    }

    #[test]
    #[ignore]
    fn json_deploy_template_missing_name() {
        todo!()
    }

    #[test]
    #[ignore]
    fn json_deploy_template_missing_code() {
        todo!()
    }

    #[test]
    #[ignore]
    fn json_deploy_template_missing_data() {
        todo!()
    }
}
