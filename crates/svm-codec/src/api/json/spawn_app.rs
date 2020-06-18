use svm_types::SpawnApp;

use crate::api::native::SpawnAppBuilder;

use serde_json::Value;

///
/// ```json
/// {
///   version: 0,            // number
///   template: '0xA29F...', // string
///   ctor_idx: 0,           // number
///   ctor_buf: Blob(...),   // Uint8Array
///   ctor_args: ['10i32', '20i64', ...] // Array of string
/// }
/// ```

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
