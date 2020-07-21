mod deploy_template;
mod error;
mod exec_app;
mod helpers;
mod spawn_app;

pub(crate) mod logs;

pub mod testing;

use error::encode_error;

mod types {
    pub const DEPLOY_TEMPLATE: u8 = 0;

    pub const SPAWN_APP: u8 = 1;

    pub const EXEC_APP: u8 = 2;
}

pub use deploy_template::encode_template_receipt;
pub use exec_app::encode_exec_receipt;
pub use spawn_app::encode_app_receipt;
