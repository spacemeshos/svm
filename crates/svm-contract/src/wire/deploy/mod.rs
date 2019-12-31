//!      Deploy AppTemplate Wire Protocol Version 0.0.0.0
//!  -------------------------------------------------------
//!  |   proto    |                |                       |
//!  |  version   |  name length   |     name (UTF-8)      |
//!  |  (4 bytes) |   (1 byte)     |                       |
//!  |____________|________________|_______________________|
//!  |                                                     |
//!  |                      author                         |
//!  |                    (20 bytes)                       |
//!  |_____________________________________________________|
//!  |             |                                       |
//!  |  #admins    |         admins addresses              |
//!  |  (2 bytes)  |          (20 bytes each)              |
//!  |_____________|_______________________________________|
//!  |           |                                         |
//!  |   #deps   |           dependencies                  |
//!  | (2 bytes) |              (TBD)                      |
//!  |___________|_________________________________________|
//!  |                |                                    |
//!  |  code length   |              code                  |
//!  |   (8 bytes)    |             (wasm)                 |
//!  |________________|____________________________________|

mod build;
mod error;
mod field;
mod parse;
mod validate;

pub use build::WireAppTemplateBuilder;
pub use error::AppTemplateBuildError;
pub use parse::parse_template;
pub use validate::validate_contract;
