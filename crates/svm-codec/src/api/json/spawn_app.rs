use svm_types::SpawnApp;

use serde_json::Value;

pub fn spawn_app(json: &Value) -> Result<SpawnApp, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn json_spawn_app_missing_version() {
        todo!()
    }

    #[test]
    #[ignore]
    fn json_spawn_app_unsupported_version() {
        todo!()
    }

    #[test]
    #[ignore]
    fn json_spawn_app_missing_template_addr() {
        todo!()
    }

    #[test]
    #[ignore]
    fn json_spawn_app_missing_ctor_index() {
        todo!()
    }

    #[test]
    #[ignore]
    fn json_spawn_app_missing_ctor_buf() {
        todo!()
    }

    #[test]
    #[ignore]
    fn json_spawn_app_missing_ctor_args() {
        todo!()
    }
}
